use std::{collections::HashMap, range::Range};

use crate::{diagnostic, diagnostic::ErrorKind, source::SourceContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Register(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OpCode {
    Push(u8),
    Swap,
    Add,
    Print,
    PrintUtf,
    Duplicate,
    Subroutine(String),
    Call(Register),
    Jump(Register),
    Return(Register),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Op {
    /// This op's actual code.
    code: OpCode,
    /// The range from within the source code this op is emitted.
    range: Range<usize>,
}

impl Op {
    pub(crate) fn new(code: OpCode, range: impl Into<Range<usize>>) -> Self {
        Self {
            code,
            range: range.into(),
        }
    }
}

pub(crate) struct Vm<'vm> {
    /// A mutable reference to the full context of the source.
    ctx: &'vm mut SourceContext,
    /// An array of opcodes to evaluate.
    bytecode: Vec<Op>,
    /// The instruction pointer.
    ip: usize,
    /// The evaluated values pushed to the user-facing stack.
    stack: Vec<u8>,
    /// A map of strings to the initial registers of subroutines.
    submap: HashMap<String, Register>,
}

impl<'vm> Vm<'vm> {
    pub(crate) fn new(ctx: &'vm mut SourceContext, bytecode: Vec<Op>) -> Self {
        Self {
            ctx,
            bytecode,
            ip: 0,
            stack: Vec::new(),
            submap: HashMap::new(),
        }
    }

    pub(crate) fn run(&mut self) -> crate::Result<()> {
        #[cfg(debug_assertions)]
        println!("RANGE\t| OP\t\t| STACK ");

        loop {
            if self.ip == u16::MAX as usize {
                self.ctx.report(diagnostic!(ErrorKind::ReachedStackLimit {
                    praise_honorific: self.ctx.rand_praise_honorific().into(),
                    interp_title: self.ctx.rand_interp_title().into(),
                }));
                break;
            }

            let op = self.bytecode.get(self.ip);

            if op.is_none() {
                #[cfg(debug_assertions)]
                println!("\t\t\t| {:?}", self.stack);
                break;
            }

            let op = op.unwrap();

            #[cfg(debug_assertions)]
            println!(
                "{}..{}\t| {:?}\t| {:?}",
                op.range.start, op.range.end, op.code, self.stack
            );

            match &op.code {
                OpCode::Push(val) => self.stack.push(*val),
                OpCode::Swap => {
                    let current = self.stack.pop();
                    let prev = self.stack.pop();

                    if let Some(current) = current
                        && let Some(prev) = prev
                    {
                        self.stack.push(current);
                        self.stack.push(prev);
                        self.ip += 1;
                        continue;
                    }

                    self.ctx.report(diagnostic!(
                        ErrorKind::InsufficientElements {
                            op: "swap".into(),
                            petname: self.ctx.rand_petname().into(),
                            interp_title: self.ctx.rand_interp_title().into(),
                        },
                        labels = [(op.range, "")]
                    ));
                }
                OpCode::Add => {
                    let current = self.stack.pop();
                    let prev = self.stack.pop();

                    if let Some(current) = current
                        && let Some(prev) = prev
                    {
                        self.stack.push(current + prev);
                        self.ip += 1;
                        continue;
                    }

                    self.ctx.report(diagnostic!(
                        ErrorKind::InsufficientElements {
                            op: "add".into(),
                            petname: self.ctx.rand_petname().into(),
                            interp_title: self.ctx.rand_interp_title().into(),
                        },
                        labels = [(op.range, "")]
                    ));
                }
                OpCode::Print => {
                    let current = self.stack.pop();

                    if let Some(current) = current {
                        print!("{}", current as char);
                        self.ip += 1;
                        continue;
                    }

                    self.ctx.report(diagnostic!(
                        ErrorKind::InsufficientElements {
                            op: "print".into(),
                            petname: self.ctx.rand_petname().into(),
                            interp_title: self.ctx.rand_interp_title().into(),
                        },
                        labels = [(op.range, "")]
                    ));
                }
                OpCode::PrintUtf => {
                    let byte3 = self.stack.pop();
                    let byte2 = self.stack.pop();
                    let byte1 = self.stack.pop();

                    if let Some(byte3) = byte3
                        && let Some(byte2) = byte2
                        && let Some(byte1) = byte1
                    {
                        let bytes = [byte1, byte2, byte3, 0x00];
                        let codepoint = u32::from_be_bytes(bytes);

                        if let Some(char) = char::from_u32(codepoint) {
                            println!("{}", char);
                            self.ip += 1;
                            continue;
                        }

                        self.ctx.report(diagnostic!(
                            ErrorKind::InvalidCodepoint {
                                petname: self.ctx.rand_petname().into(),
                                interp_title: self.ctx.rand_interp_title().into(),
                            },
                            labels = [(op.range, "")]
                        ));
                    }

                    self.ctx.report(diagnostic!(
                        ErrorKind::InsufficientElements {
                            op: "print unicode".into(),
                            petname: self.ctx.rand_petname().into(),
                            interp_title: self.ctx.rand_interp_title().into(),
                        },
                        labels = [(op.range, "")]
                    ));
                }
                OpCode::Duplicate => {
                    let current = self.stack.last();

                    if let Some(current) = current {
                        self.stack.push(*current);
                        self.ip += 1;
                        continue;
                    }

                    self.ctx.report(diagnostic!(
                        ErrorKind::InsufficientElements {
                            op: "duplicate".into(),
                            petname: self.ctx.rand_petname().into(),
                            interp_title: self.ctx.rand_interp_title().into(),
                        },
                        labels = [(op.range, "")]
                    ));
                }
                OpCode::Subroutine(ident) => {
                    // self.submap.insert(ident.clone(), Register(self.ip));
                }
                OpCode::Call(reg) => todo!(),
                OpCode::Jump(reg) => todo!(),
                OpCode::Return(reg) => break,
            }

            self.ip += 1;
        }

        Ok(())
    }
}
