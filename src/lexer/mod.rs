use std::range::Range;

use crate::{
    diagnostic,
    diagnostics::ErrorKind,
    lexer::token::{Token, TokenStream, TokenType},
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
        c.is_ascii_alphabetic() || (192..=255).contains(&(c as u32))
    }

    /// Returns a token's lexeme.
    fn lexeme(&self) -> String {
        self.chars[self.range()].iter().collect()
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

    /// Returns whether the next character is equal to `expected` and
    /// consumes it if so.
    fn matches(&mut self, expected: char) -> bool {
        // technically `char` could equal `expected` despite being None
        // as CharIndices::peek will return None for a null byte
        // but we won't be checking for the null byte explicitly like this
        if let Some(char) = self.peek()
            && char == expected
        {
            self.next();
            return true;
        }

        false
    }

    /// Traverses [`chars`], constructs a [`TokenStream`] and returns it.
    ///
    /// [`chars`]: Lexer::chars
    /// [`TokenStream`]: crate::lexer::token::TokenStream
    pub(crate) fn lex_tokens(&mut self) {
        while let Some(char) = self.next() {
            let token_type = match char {
                '>' => self.lex_emoticon(),
                ':' => self.lex_colon_three(),
                '🥺' => TokenType::Sub,
                '👉' => {
                    if self.matches('👈') {
                        TokenType::Point
                    } else {
                        self.ctx.report(diagnostic!(
                            ErrorKind::UnfinishedEmoticon {
                                petname: self.ctx.rand_petname().into(),
                                char_to_add: '👈'
                            },
                            labels = [(self.byte_range(), "")]
                        ));
                        TokenType::Error
                    }
                }
                '🏳' => {
                    self.lex_comment();
                    continue;
                }
                c if c.is_whitespace() || c == ';' => {
                    self.start = self.current;
                    self.start_byte = self.current_byte;
                    continue;
                }
                c => {
                    self.lex_keysmash(c);
                    continue;
                }
            };

            self.tokenise(token_type, self.lexeme());
        }

        self.tokenise(TokenType::Eof, '\0'.into());
    }

    fn lex_emoticon(&mut self) -> TokenType {
        match self.peek() {
            Some('w') => {
                self.next();

                if self.matches('<') {
                    TokenType::FlusteredW
                } else {
                    self.ctx.report(diagnostic!(
                        ErrorKind::UnfinishedEmoticon {
                            petname: self.ctx.rand_petname().into(),
                            char_to_add: '<'
                        },
                        labels = [(self.byte_range(), "")]
                    ));
                    TokenType::Error
                }
            }
            Some('~') => {
                self.next();
                if self.matches('<') {
                    TokenType::FlusteredTilde
                } else {
                    self.ctx.report(diagnostic!(
                        ErrorKind::UnfinishedEmoticon {
                            petname: self.ctx.rand_petname().into(),
                            char_to_add: '<'
                        },
                        labels = [(self.byte_range(), "")]
                    ));
                    TokenType::Error
                }
            }
            Some('/') => {
                self.next();
                self.lex_blush_slashes_emoticon()
            }
            Some('.') => {
                self.next();

                if self.matches('<') {
                    TokenType::FlusteredDot
                } else {
                    self.ctx.report(diagnostic!(
                        ErrorKind::UnfinishedEmoticon {
                            petname: self.ctx.rand_petname().into(),
                            char_to_add: '<'
                        },
                        labels = [(self.byte_range(), "")]
                    ));
                    TokenType::Error
                }
            }
            Some(_) | None => {
                self.ctx.report(diagnostic!(
                    ErrorKind::AmbiguousUnfinishedEmoticon {
                        interp_title: self.ctx.rand_interp_title().into(),
                        petname: self.ctx.rand_petname().into(),
                    },
                    labels = [(self.byte_range(), "")]
                ));
                TokenType::Error
            }
        }
    }

    /// Lexes the `>//<` emoticon.
    fn lex_blush_slashes_emoticon(&mut self) -> TokenType {
        fn check_double_slash(this: &mut Lexer) -> bool {
            if this.peek().is_some_and(|peek| peek != '/') {
                this.ctx.report(diagnostic!(
                    ErrorKind::UnfinishedEmoticon {
                        petname: this.ctx.rand_petname().into(),
                        char_to_add: '/'
                    },
                    labels = [(this.byte_range(), "")]
                ));
                false
            } else {
                this.next();
                true
            }
        }

        if !check_double_slash(self) {
            return TokenType::Error;
        }

        let mut len = 1;

        while self.matches('/') {
            if !check_double_slash(self) {
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
            return TokenType::Blush { len };
        }

        self.ctx.report(diagnostic!(
            ErrorKind::UnfinishedEmoticon {
                petname: self.ctx.rand_petname().into(),
                char_to_add: '<'
            },
            labels = [(self.byte_range(), "")]
        ));

        TokenType::Error
    }

    /// Lexes the `:3` emoticon.
    fn lex_colon_three(&mut self) -> TokenType {
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
                    petname: self.ctx.rand_petname().into(),
                    char_to_add: '3'
                },
                labels = [(self.byte_range(), "")]
            ));
            return TokenType::Error;
        }

        TokenType::ColonThree { len }
    }

    /// Lexes an inline comment.
    fn lex_comment(&mut self) {
        fn check_trans_flag_glyphs(this: &mut Lexer, glyph: char) -> bool {
            let valid = this.matches(glyph);

            if !valid {
                this.ctx.report(diagnostic!(
                    ErrorKind::UnexpectedToken {
                        petname: this.ctx.rand_petname().into(),
                        interp_title: this.ctx.rand_interp_title().into(),
                        praise_term: this.ctx.rand_praise_term().into(),
                        char: glyph,
                    },
                    labels = [(this.range(), "")]
                ))
            }

            valid
        }

        for glyph in ['\u{fe0f}', '\u{200d}', '\u{26a7}', '\u{fe0f}'] {
            if !check_trans_flag_glyphs(self, glyph) {
                return self.tokenise(TokenType::Error, self.lexeme());
            }
        }

        while self.peek().is_some_and(|peek| peek != '\n') {
            self.next();
        }
    }

    /// Lexes a keysmash.
    fn lex_keysmash(&mut self, start: char) {
        let lowercase = start.is_lowercase();

        if !self.is_valid_keysmash_char(start) {
            self.ctx.report(diagnostic!(
                ErrorKind::UnexpectedToken {
                    petname: self.ctx.rand_petname().into(),
                    interp_title: self.ctx.rand_interp_title().into(),
                    praise_term: self.ctx.rand_praise_term().into(),
                    char: start
                },
                labels = [(self.byte_range(), "")]
            ));
            return self.tokenise(TokenType::Error, self.lexeme());
        }

        let mut len = 1;

        while let Some(char) = self.peek() {
            if self.is_valid_keysmash_char(char) {
                if lowercase == char.is_lowercase() {
                    if len == KEYSMASH_MAX_LEN {
                        self.ctx.report(diagnostic!(
                            ErrorKind::OverlongKeysmash {
                                interp_title: self.ctx.rand_interp_title().into(),
                                petname: self.ctx.rand_petname().into()
                            },
                            labels = [(self.byte_range(), "")]
                        ));
                        return self.tokenise(TokenType::Error, self.lexeme());
                    }

                    len += 1;
                    self.next();
                    continue;
                }

                return self.tokenise(TokenType::Keysmash { lowercase, len }, self.lexeme());
            }

            break;
        }

        let lexeme = self.lexeme();

        let token = if self.ctx.env_vars.print_keywords.contains(&lexeme) {
            TokenType::Print {
                utf: self.matches('~'),
            }
        } else {
            TokenType::Keysmash { lowercase, len }
        };

        self.tokenise(token, lexeme);
    }
}
