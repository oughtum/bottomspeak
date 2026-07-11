use std::range::Range;

use crate::{
    diagnostic,
    diagnostics::ErrorKind,
    lexer::token::{PrintKind, Token, TokenStream, TokenType},
    source::SourceContext,
};

pub(crate) mod tests;
pub(crate) mod token;

pub(crate) const KEYSMASH_MAX_LEN: u8 = 128;

pub(crate) struct Lexer<'lx> {
    /// A reference to the full context for the source.
    ctx: &'lx mut SourceContext,

    /// A vector of the characters in the [`source`](SourceContext::source).
    chars: Vec<char>,

    /// The index of the first character within the current token's lexeme.
    start: usize,

    /// The index of the current character.
    current: usize,

    /// The index of the first byte within the current token's lexeme, used
    /// for token range calculations.
    start_byte: usize,

    /// The index of the current byte.
    current_byte: usize,

    /// The [`TokenStream`] to construct.
    tokens: TokenStream,

    /// Whether the source code contains any comments.
    has_comments: bool,
}

impl<'lx> Lexer<'lx> {
    /// Constructs a new lexer.
    pub(crate) fn new(ctx: &'lx mut SourceContext) -> Self {
        let chars = ctx.source.chars().collect();

        Self {
            ctx,
            chars,
            start: 0,
            current: 0,
            start_byte: 0,
            current_byte: 0,
            tokens: TokenStream::default(),
            has_comments: false,
        }
    }

    pub(crate) fn tokens(self) -> TokenStream {
        self.tokens
    }

    /// Appends a new [`Token`] to the [`TokenStream`] and resets [`start`] to the
    /// beginning of the next [`Token`].
    ///
    /// ## Parameters
    ///
    /// * `kind` - The [`TokenType`] to tokenise.
    /// * `lexeme` - The string representing the token within the source.
    ///
    /// [`start`]: Lexer::start
    fn tokenise(&mut self, kind: TokenType, lexeme: String) {
        let range = self.byte_range();

        self.tokens.push(Token::new(kind, &lexeme, range));

        self.start = self.current;
        self.start_byte = self.current_byte;
    }

    /// Returns whether a character is valid within a keysmash.
    fn is_valid_keysmash_char(&self, c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    /// Returns a token's lexeme.
    fn lexeme(&self) -> String {
        self.chars[self.range()].iter().collect()
    }

    /// Returns a token's lexeme with a custom range.
    fn lexemec(&self, range: impl Into<Range<usize>>) -> String {
        self.chars[range.into()].iter().collect()
    }

    /// Returns the range spanning the characters of the current token's lexeme.
    fn range(&'lx self) -> Range<usize> {
        (self.start..self.current).into()
    }

    /// Returns the range spanning the bytes of the current token's lexeme.
    fn byte_range(&'lx self) -> Range<usize> {
        (self.start_byte..self.current_byte).into()
    }

    /// Returns the current character in [`chars`](Lexer::chars) without
    /// consuming it.
    fn peek(&mut self) -> Option<char> {
        self.chars.get(self.current).copied()
    }

    /// Returns and consumes the current character in [`chars`](Lexer::chars).
    fn next(&mut self) -> Option<char> {
        let peek = self.peek()?;
        self.current += 1;
        self.current_byte += peek.len_utf8();

        Some(peek)
    }

    /// Returns whether the next character is equal to `expected` without
    /// consuming it.
    fn check(&mut self, expected: char) -> bool {
        self.peek().is_some_and(|peek| peek == expected)
    }

    /// Returns whether the next character is equal to `expected` and
    /// consumes it if so.
    fn matches(&mut self, expected: char) -> bool {
        if self.check(expected) {
            self.next();
            return true;
        }

        false
    }

    /// Traverses [`chars`], constructs a [`TokenStream`] and returns it.
    ///
    /// [`chars`]: Lexer::chars
    /// [`TokenStream`]: crate::lexer::token::TokenStream
    pub(crate) fn lex_tokens(&mut self, in_repl: bool) {
        while let Some(char) = self.next() {
            let token_type = match char {
                '>' => self.lex_flustered_emoticon(),
                'U' => self.lex_uwu_owo(TokenType::Uwu, 'U'),
                '@' | 'O' | '0' => self.lex_heavy_flustered_emoticon(char),
                '^' => self.lex_happy_emoticon(),
                ':' => self.lex_colon_three(true),
                '🥺' => TokenType::Sub,
                '👉' => {
                    if self.matches('👈') {
                        TokenType::Point
                    } else {
                        self.ctx.report(diagnostic!(
                            ErrorKind::UnfinishedEmoticon {
                                praise_term: self.ctx.rand_praise_term().into(),
                                petname: self.ctx.rand_petname().into(),
                                char_to_add: '👈'
                            },
                            labels = [(self.byte_range(), "")]
                        ));
                        TokenType::Error
                    }
                }
                '🏳' => {
                    self.lex_flag_emoji();
                    continue;
                }
                c if c.is_whitespace() || c == ';' => {
                    self.start = self.current;
                    self.start_byte = self.current_byte;
                    continue;
                }
                c => self.lex_keysmash(c),
            };

            self.tokenise(token_type, self.lexeme());
        }

        if !self.has_comments && !in_repl {
            self.ctx.report(diagnostic!(
                ErrorKind::UncommentedSource {
                    interp_title: self.ctx.rand_interp_title().into(),
                    petname: self.ctx.rand_petname().into(),
                    praise_term: self.ctx.rand_praise_term().into(),
                },
                labels = [(self.byte_range(), "")]
            ))
        }

        self.tokenise(TokenType::Eof, '\0'.into());
    }

    /// Lexes an emoticon surrounded by `><`.
    fn lex_flustered_emoticon(&mut self) -> TokenType {
        match self.peek() {
            Some('x') => self.lex_flustered_end(TokenType::FlusteredX, '<'),
            Some('o') => self.lex_flustered_end(TokenType::FlusteredO, '<'),
            Some('w') => self.lex_flustered_end(TokenType::FlusteredW, '<'),
            Some('~') => self.lex_flustered_end(TokenType::FlusteredTilde, '<'),
            Some('.') => self.lex_flustered_end(TokenType::FlusteredDot, '<'),
            Some(':') => {
                self.next();
                self.lex_colon_three(false)
            }
            c if c == Some('/') || c == Some('\\') => {
                self.next();
                self.lex_blush_slashes_emoticon(c.unwrap())
            }
            Some(_) | None => {
                self.ctx.report(diagnostic!(
                    ErrorKind::AmbiguousFlusteredEmoticon {
                        interp_title: self.ctx.rand_interp_title().into(),
                        petname: self.ctx.rand_petname().into(),
                    },
                    labels = [(self.byte_range(), "")]
                ));
                TokenType::Error
            }
        }
    }

    /// Lexes `UwU` or `OwO`.
    fn lex_uwu_owo(&mut self, kind: TokenType, end: char) -> TokenType {
        let start_range = self.byte_range();

        match self.peek() {
            Some('w') => {
                self.next();

                if self.matches(end) {
                    kind
                } else {
                    // treat the previous `U`/`O` and `w` as keysmash parts
                    // since the next character isn't another `U`/`O`
                    self.tokenise(
                        TokenType::Keysmash {
                            tilde: false,
                            lowercase: false,
                            len: 1,
                        },
                        self.lexemec(start_range),
                    );

                    self.lex_keysmash('w')
                }
            }
            // end is the same as the start so we can just use that here
            Some(_) | None => self.lex_keysmash(end),
        }
    }

    /// Lexes the final character of a flustered emoticon.
    fn lex_flustered_end(&mut self, kind: TokenType, end: char) -> TokenType {
        self.next();

        if self.matches(end) {
            kind
        } else {
            self.ctx.report(diagnostic!(
                ErrorKind::UnfinishedEmoticon {
                    praise_term: self.ctx.rand_praise_term().into(),
                    petname: self.ctx.rand_petname().into(),
                    char_to_add: end
                },
                labels = [(self.byte_range(), "")]
            ));
            TokenType::Error
        }
    }

    /// Lexes the `>//<` and `>\\<` emoticons.
    fn lex_blush_slashes_emoticon(&mut self, slash: char) -> TokenType {
        fn check_double_slash(this: &mut Lexer, slash: char) -> bool {
            if this.peek().is_some_and(|peek| peek != slash) {
                this.ctx.report(diagnostic!(
                    ErrorKind::UnfinishedEmoticon {
                        praise_term: this.ctx.rand_praise_term().into(),
                        petname: this.ctx.rand_petname().into(),
                        char_to_add: slash
                    },
                    labels = [(this.byte_range(), "")]
                ));
                false
            } else {
                this.next();
                true
            }
        }

        if !check_double_slash(self, slash) {
            return TokenType::Error;
        }

        let mut len = 1;

        while self.matches(slash) {
            if !check_double_slash(self, slash) {
                return TokenType::Error;
            }

            if len == KEYSMASH_MAX_LEN {
                self.ctx.report(diagnostic!(
                    ErrorKind::OverlongKeysmash {
                        interp_title: self.ctx.rand_interp_title().into(),
                        petname: self.ctx.rand_petname().into()
                    },
                    labels = [(self.byte_range(), "")]
                ));
                return TokenType::Error;
            }

            len += 1;
        }

        if self.matches('<') {
            return TokenType::Blush {
                double: slash == '\\',
                len,
            };
        }

        self.ctx.report(diagnostic!(
            ErrorKind::UnfinishedEmoticon {
                praise_term: self.ctx.rand_praise_term().into(),
                petname: self.ctx.rand_petname().into(),
                char_to_add: '<'
            },
            labels = [(self.byte_range(), "")]
        ));

        TokenType::Error
    }

    /// Lexes a flag emoji.
    fn lex_flag_emoji(&mut self) {
        let diag = |this: &&mut Lexer, glyph: char| {
            diagnostic!(
                ErrorKind::UnexpectedToken {
                    petname: this.ctx.rand_petname().into(),
                    interp_title: this.ctx.rand_interp_title().into(),
                    praise_term: this.ctx.rand_praise_term().into(),
                    char: glyph,
                },
                labels = [(this.range(), "")]
            )
        };

        // initial chars (White Flag + Variation Selector 16 + Zero-Width Joiner)
        for glyph in ['\u{fe0f}', '\u{200d}'] {
            if !self.matches(glyph) {
                self.ctx.report(diag(&self, glyph));
                return self.tokenise(TokenType::Error, self.lexeme());
            }
        }

        // rainbow flag chars (🌈)
        if self.matches('\u{1f308}') {
            return self.tokenise(
                TokenType::Print {
                    kind: PrintKind::Ansi,
                },
                self.lexeme(),
            );
        }

        // trans flag chars (Transgender Symbol + Variation Selector 16)
        for glyph in ['\u{26a7}', '\u{fe0f}'] {
            if !self.matches(glyph) {
                self.ctx.report(diag(&self, glyph));
                return self.tokenise(TokenType::Error, self.lexeme());
            }
        }

        // token must be a trans flag and so we lex the rest as a comment
        while self.peek().is_some_and(|peek| peek != '\n') {
            self.has_comments = true;
            self.next();
        }
    }

    /// Lexes a heavy flustered emoticon i.e. `@~@`, `O~O` or `0~0`.
    fn lex_heavy_flustered_emoticon(&mut self, start: char) -> TokenType {
        match start {
            '@' => {
                if self.check('~') {
                    self.lex_flustered_end(TokenType::HeavyFlusteredAt, '@')
                } else {
                    self.ctx.report(diagnostic!(
                        ErrorKind::UnfinishedEmoticon {
                            praise_term: self.ctx.rand_praise_term().into(),
                            petname: self.ctx.rand_petname().into(),
                            char_to_add: '@'
                        },
                        labels = [(self.byte_range(), "")]
                    ));
                    return TokenType::Error;
                }
            }
            'O' => {
                if self.check('w') {
                    self.lex_uwu_owo(TokenType::Owo, 'O')
                } else if self.check('~') {
                    self.lex_flustered_end(TokenType::HeavyFlusteredO, 'O')
                } else {
                    self.lex_keysmash(start)
                }
            }
            '0' => {
                if self.check('~') {
                    self.lex_flustered_end(TokenType::HeavyFlusteredZero, '0')
                } else {
                    self.ctx.report(diagnostic!(
                        ErrorKind::UnfinishedEmoticon {
                            praise_term: self.ctx.rand_praise_term().into(),
                            petname: self.ctx.rand_petname().into(),
                            char_to_add: '0'
                        },
                        labels = [(self.byte_range(), "")]
                    ));
                    return TokenType::Error;
                }
            }
            _ => {
                self.ctx.report(diagnostic!(
                    ErrorKind::Bug {
                        interp_title: self.ctx.rand_interp_title().into(),
                        praise_term: self.ctx.rand_praise_term().into(),
                    },
                    labels = [(self.byte_range(), "")]
                ));
                TokenType::Error
            }
        }
    }

    /// Lexes an emoticon surrounded by `^^`
    fn lex_happy_emoticon(&mut self) -> TokenType {
        match self.peek() {
            Some('x') => self.lex_flustered_end(TokenType::HappyX, '^'),
            Some('o') => self.lex_flustered_end(TokenType::HappyO, '^'),
            Some('w') => self.lex_flustered_end(TokenType::HappyW, '^'),
            Some(_) | None => {
                self.ctx.report(diagnostic!(
                    ErrorKind::AmbiguousHappyEmoticon {
                        interp_title: self.ctx.rand_interp_title().into(),
                        petname: self.ctx.rand_petname().into(),
                    },
                    labels = [(self.byte_range(), "")]
                ));
                TokenType::Error
            }
        }
    }

    /// Lexes the `:3` emoticon.
    fn lex_colon_three(&mut self, add: bool) -> TokenType {
        let mut len = 0;

        while self.matches('3') {
            if len == KEYSMASH_MAX_LEN {
                self.ctx.report(diagnostic!(
                    ErrorKind::OverlongKeysmash {
                        interp_title: self.ctx.rand_interp_title().into(),
                        petname: self.ctx.rand_petname().into()
                    },
                    labels = [(self.byte_range(), "")]
                ));
                return TokenType::Error;
            }

            len += 1;
        }

        if len == 0 {
            self.ctx.report(diagnostic!(
                ErrorKind::UnfinishedEmoticon {
                    praise_term: self.ctx.rand_praise_term().into(),
                    petname: self.ctx.rand_petname().into(),
                    char_to_add: '3'
                },
                labels = [(self.byte_range(), "")]
            ));
            return TokenType::Error;
        }

        TokenType::ColonThree { add, len }
    }

    /// Lexes a keysmash.
    fn lex_keysmash(&mut self, start: char) -> TokenType {
        let lowercase = start.is_lowercase();

        if !self.is_valid_keysmash_char(start) {
            println!("{start}");
            self.ctx.report(diagnostic!(
                ErrorKind::UnexpectedToken {
                    petname: self.ctx.rand_petname().into(),
                    interp_title: self.ctx.rand_interp_title().into(),
                    praise_term: self.ctx.rand_praise_term().into(),
                    char: start
                },
                labels = [(self.byte_range(), "")]
            ));
            return TokenType::Error;
        }

        let mut len = 1;

        while let Some(char) = self.peek() {
            if !self.is_valid_keysmash_char(char) {
                break;
            }

            if lowercase != char.is_lowercase() {
                return TokenType::Keysmash {
                    tilde: self.matches('~'),
                    lowercase,
                    len,
                };
            }

            if len == KEYSMASH_MAX_LEN {
                self.ctx.report(diagnostic!(
                    ErrorKind::OverlongKeysmash {
                        interp_title: self.ctx.rand_interp_title().into(),
                        petname: self.ctx.rand_petname().into()
                    },
                    labels = [(self.byte_range(), "")]
                ));
                return TokenType::Error;
            }

            len += 1;
            self.next();
            continue;
        }

        let lexeme = self.lexeme();

        if self.ctx.env_vars.print_keywords.contains(&lexeme) {
            let kind = match self.peek() {
                Some('~') => {
                    self.next();
                    PrintKind::Utf
                }
                Some('!') => {
                    self.next();
                    PrintKind::Literal
                }
                Some(_) | None => PrintKind::Normal,
            };

            return TokenType::Print { kind };
        }

        if self.ctx.env_vars.interp_titles.contains(&lexeme) {
            TokenType::InterpTitle {
                tilde: self.matches('~'),
            }
        } else {
            TokenType::Keysmash {
                tilde: self.matches('~'),
                lowercase,
                len,
            }
        }
    }
}
