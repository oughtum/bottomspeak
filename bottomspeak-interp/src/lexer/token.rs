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
    /// '`>w<`'
    FlusteredW,
    /// `'>~<'`
    FlusteredTilde,
    /// '`:3`'
    ColonThree {
        len: u8,
    },
    /// '`>//<`'
    Blush {
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
        lowercase: bool,
        len: u8,
    },
    Print {
        utf: bool,
    },
    Error,
    /// '`\0`'
    Eof,
}
