use std::{cmp::min, collections::BTreeMap};

use crate::{
    diagnostic,
    diagnostics::ErrorKind,
    lexer::{
        KEYSMASH_MAX_LEN,
        token::{Token, TokenStream, TokenType},
    },
    source::SourceContext,
    vm::{INTERNAL_ROOT_SUBROUTINE, Op, OpCode, Subroutine, SubroutineMap},
};

pub(crate) mod tests;

pub(crate) struct Parser<'p> {
    /// A mutable reference to the full context of the source.
    ctx: &'p mut SourceContext,

    /// The tokens parsed by the [`Lexer`](crate::lexer::Lexer).
    tokens: TokenStream,

    /// The index of the current [`Token`](crate::lexer::token::Token).
    current: usize,

    /// A map of identifiers to parsed subroutines.
    pub(crate) submap: SubroutineMap,
}

impl<'p> Parser<'p> {
    pub(crate) fn new(tokens: TokenStream, ctx: &'p mut SourceContext) -> Self {
        // create an internal root subroutine that wraps the whole source code
        let mut submap = BTreeMap::new();
        submap.insert(INTERNAL_ROOT_SUBROUTINE.to_string(), Subroutine::default());

        Self {
            ctx,
            tokens,
            current: 0,
            submap,
        }
    }

    /// Returns a reference to the current token in the [`TokenStream`]
    /// without consuming it.
    fn peek(&self) -> Option<&Token> {
        let peek = self.tokens.get(self.current);

        peek
    }

    /// Returns a reference to the previous token in the [`TokenStream`].
    fn peekp(&self) -> Option<&Token> {
        if self.current == 0 {
            return None;
        }

        self.tokens.get(self.current.saturating_sub(1))
    }

    /// Returns and consumes the current token in the [`TokenStream`].
    fn next(&mut self) -> Option<&Token> {
        let len = self.tokens.len();
        let current = self.current.saturating_add(1);
        self.current = min(current, len);

        self.peekp()
    }

    /// Returns whether the current token's [`TokenType`] is the same as
    /// the expected [`TokenType`].
    fn check(&self, expected: TokenType) -> bool {
        self.peek().is_some_and(|tok| tok.kind() == expected)
    }

    /// Returns whether the current token's [`TokenType`] matches what is expected and
    /// consumes it if so.
    fn matches(&mut self, expected: TokenType) -> bool {
        if self.check(expected) {
            self.next();
            return true;
        }

        false
    }

    /// Returns whether the current token is [`None`], and thus all tokens have been consumed, or
    /// it is [`Some(TokenType::Eof)`].
    fn is_eof(&mut self) -> bool {
        let peek = self.peek();
        peek.is_none() || peek.is_some_and(|tok| tok.kind() == TokenType::Eof)
    }
    fn emit_op(&mut self, op: Op) {
        if let Some((_, sub)) = self.submap.iter_mut().next_back() {
            sub.emit_op(op);
        }
    }

    pub(crate) fn parse(&mut self) {
        if self.is_eof() {
            self.ctx.report(diagnostic!(
                ErrorKind::EmptySource {
                    praise_term: self.ctx.rand_praise_term().into(),
                    interp_title: self.ctx.rand_interp_title().into(),
                },
                labels = [(0..0, "")]
            ));
        }

        while !self.is_eof() {
            if self.parse_instruction().is_none() {
                break;
            }
        }

        let end = self.ctx.source.len();
        self.emit_op(Op::new(OpCode::Return, end..end));
    }

    fn parse_instruction(&mut self) -> Option<()> {
        if let Some(tok) = self.next() {
            let range = tok.range();

            match tok.kind() {
                TokenType::FlusteredW => self.emit_op(Op::new(OpCode::Swap, range)),
                TokenType::FlusteredTilde => self.emit_op(Op::new(OpCode::Pop, range)),
                TokenType::ColonThree { len } => {
                    let range = tok.range();

                    for _ in 0..len {
                        self.emit_op(Op::new(OpCode::Add, range));
                    }
                }
                TokenType::Blush { len } => {
                    let range = tok.range();

                    for _ in 0..len {
                        self.emit_op(Op::new(OpCode::Duplicate, range));
                    }
                }
                TokenType::FlusteredDot => {
                    self.emit_op(Op::new(OpCode::Return, range));
                }
                TokenType::Sub => {
                    let range = tok.range();

                    self.ctx.report(diagnostic!(
                        ErrorKind::UnnamedSub {
                            petname: self.ctx.rand_petname().into(),
                            interp_title: self.ctx.rand_interp_title().into(),
                        },
                        labels = [(range, "")]
                    ));
                }
                TokenType::Point => {
                    let range = tok.range();

                    self.ctx.report(diagnostic!(
                        ErrorKind::UnnamedJump {
                            petname: self.ctx.rand_petname().into(),
                            interp_title: self.ctx.rand_interp_title().into(),
                            praise_term: self.ctx.rand_praise_term().into(),
                        },
                        labels = [(range, "")]
                    ));
                }
                TokenType::Keysmash { lowercase, len } => {
                    let ident = tok.lexeme().to_owned();
                    let range = tok.range();

                    let val = if lowercase {
                        len - 1
                    } else {
                        len - 1 + KEYSMASH_MAX_LEN
                    };

                    if self.matches(TokenType::Sub) {
                        return self.parse_subroutine(range.start);
                    }

                    if self.matches(TokenType::Point) {
                        self.emit_op(Op::new(OpCode::Jump(ident), range))
                    }

                    self.emit_op(Op::new(OpCode::Push(val), range))
                }
                TokenType::Print { utf } => {
                    let range = tok.range();

                    if utf {
                        self.emit_op(Op::new(OpCode::PrintUtf, range))
                    } else {
                        self.emit_op(Op::new(OpCode::Print, range))
                    }
                }
                TokenType::Error | TokenType::Eof => {
                    self.next();
                    return None;
                }
            }
        }

        Some(())
    }

    fn parse_subroutine(&mut self, start: usize) -> Option<()> {
        loop {
            let end = self.peekp()?.end();

            if self.is_eof() {
                self.ctx.report(diagnostic!(
                    ErrorKind::SubWithoutReturn {
                        interp_title: self.ctx.rand_interp_title().into()
                    },
                    labels = [(start..end, "")]
                ));
                return None;
            }

            if self.matches(TokenType::FlusteredDot) {
                self.emit_op(Op::new(OpCode::Return, start..end));
                return Some(());
            }

            self.parse_instruction();
        }
    }
}
