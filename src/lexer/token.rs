use std::range::Range;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TokenStream(Vec<Token>);

impl TokenStream {
    pub(crate) fn push(&mut self, tok: Token) {
        self.0.push(tok);
    }

    #[cfg(test)]
    pub(crate) fn inner(&self) -> &Vec<Token> {
        &self.0
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn get(&self, idx: usize) -> Option<&Token> {
        self.0.get(idx)
    }
}

/// Represents a single token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Token {
    kind: TokenType,
    lexeme: String,
    range: Range<usize>,
}

impl Token {
    pub(crate) fn new(kind: TokenType, lexeme: &str, range: impl Into<Range<usize>>) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            range: range.into(),
        }
    }

    pub(crate) fn kind(&self) -> TokenType {
        self.kind
    }

    pub(crate) fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub(crate) fn range(&self) -> Range<usize> {
        self.range
    }

    pub(crate) fn end(&self) -> usize {
        self.range.end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum TokenType {
    /// '`^x^`'
    HappyX,
    /// '`^o^`'
    HappyO,
    /// '`^w^`'
    HappyW,
    /// '`>x<`'
    FlusteredX,
    /// '`>o<`'
    FlusteredO,
    /// '`>w<`'
    FlusteredW,
    /// '`>~<`'
    FlusteredTilde,
    /// '`@~@`'
    HeavyFlusteredAt,
    /// '`O~O`'
    HeavyFlusteredO,
    /// '`0~0`'
    HeavyFlusteredZero,
    /// '`uwu`'
    Uwu,
    /// '`owo`'
    Owo,
    /// '`:3`'
    ColonThree {
        add: bool,
        len: u8,
    },
    /// '`>//<`' or '`>\\<`'
    Blush {
        double: bool,
        len: u8,
    },
    /// '`>.<`'
    FlusteredDot,
    /// '🥺'
    Sub,
    /// '👉👈'
    Point,
    /// '`asdlfjhalfadlfkj`'
    Keysmash {
        tilde: bool,
        lowercase: bool,
        len: u8,
    },
    /// `'meow'`
    Print {
        kind: PrintKind,
    },
    /// '`mommy`'
    InterpTitle {
        tilde: bool,
    },
    Error,
    /// '`\0`'
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum PrintKind {
    Normal,
    Utf,
    Ansi,
    Literal,
}
