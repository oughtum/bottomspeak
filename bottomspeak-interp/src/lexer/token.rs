use std::range::Range;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TokenStream(Vec<Token>);

impl TokenStream {
    pub(crate) fn push(&mut self, tok: Token) {
        self.0.push(tok);
    }

    pub(crate) fn inner(&self) -> &Vec<Token> {
        &self.0
    }
}

/// Represents a single token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Token {
    pub(crate) kind: TokenType,
    pub(crate) lexeme: String,
    pub(crate) range: Range<usize>,
}

impl Token {
    pub(crate) fn new(kind: TokenType, lexeme: &str, range: impl Into<Range<usize>>) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            range: range.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum TokenType {
    /// '`>w<`'
    BlushW,
    /// '`:3`'
    ColonThree {
        len: u8,
    },
    /// '`>//<`'
    BlushSlash {
        len: u8,
    },
    /// '`>.<`'
    BlushDot,
    /// '🥺'
    Sub,
    /// '👉👈'
    Point,
    /// '`asdlfjhalfadlfkj`'
    Keysmash {
        lowercase: bool,
        len: u8,
    },
    Print {
        utf: bool,
    },
    /// '`🏳️‍⚧️this is a comment :3`'
    Comment,
    Error,
    /// '`\0`'
    Eof,
}
