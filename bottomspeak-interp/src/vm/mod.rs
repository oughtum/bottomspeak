use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
    range::Range,
};

use crate::{diagnostic, diagnostics::ErrorKind, source::SourceContext};

pub(crate) mod tests;

pub(crate) const INTERNAL_ROOT_SUBROUTINE: &str = "__root";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Register(pub(crate) usize);

impl Deref for Register {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Register {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OpCode {
    Push(u8),
    Pop,
    Swap,
    Add,
    Print,
    PrintUtf,
    Duplicate,
    Jump(String),
    Return,
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

#[derive(Default, Debug)]
pub(crate) struct Subroutine {
    /// An array of op codes to evaluate.
    bytecode: Vec<Op>,
}

impl Subroutine {
    pub(crate) fn emit_op(&mut self, op: Op) {
        self.bytecode.push(op);
    }
}

pub(crate) struct CallFrame {
    /// The instruction pointer.
    ins_ptr: Register,
    /// The identifier for the subroutine being called.
    ident: String,
}

impl CallFrame {
    pub(crate) fn new(ident: String) -> Self {
        Self {
            ins_ptr: Register(0),
            ident,
        }
    }
}

pub(crate) type SubroutineMap = BTreeMap<String, Subroutine>;

pub(crate) struct Vm<'vm> {
    /// A mutable reference to the full context of the source.
    ctx: &'vm mut SourceContext,
    /// The evaluated values pushed to the user-facing stack.
    stack: Vec<u8>,
    /// A map of strings to the initial registers of subroutine declarations.
    submap: SubroutineMap,
    /// An array of call frames which keeps track of a subroutine's evaluation.
    frames: Vec<CallFrame>,
    /// The string to print once execution has finished.
    /// Pushing chars to this string is preferred over printing every char each time to avoid
    /// I/O overhead.
    pub(crate) printed_output: String,
}

impl<'vm> Vm<'vm> {
    pub(crate) fn new(ctx: &'vm mut SourceContext, submap: SubroutineMap) -> Self {
        Self {
            ctx,
            stack: Vec::new(),
            submap,
            frames: vec![CallFrame::new(INTERNAL_ROOT_SUBROUTINE.to_string())],
            printed_output: String::new(),
        }
    }

    fn create_frame(&mut self, name: String) {
        self.frames.push(CallFrame::new(name));
    }

    fn last_frame(&self) -> Option<&CallFrame> {
        let idx = self.frames.len().saturating_sub(1);
        self.frames.get(idx)
    }

    fn last_frame_mut(&mut self) -> Option<&mut CallFrame> {
        let idx = self.frames.len().saturating_sub(1);
        self.frames.get_mut(idx)
    }

    pub(crate) fn run(&mut self) -> crate::Result<()> {
        #[cfg(debug_assertions)]
        println!("RANGE\t| OP\t\t| STACK ");

        while !self.frames.is_empty() {
            let frame = self.last_frame().unwrap();

            if *frame.ins_ptr == u16::MAX as usize {
                self.ctx.report(diagnostic!(ErrorKind::ReachedStackLimit {
                    praise_term: self.ctx.rand_praise_term().into(),
                    interp_title: self.ctx.rand_interp_title().into(),
                }));
                break;
            }

            let sub = match self.submap.get(&frame.ident) {
                Some(sub) => sub,
                None => {
                    self.ctx
                        .report(diagnostic!(ErrorKind::UnresolvedSubroutine {
                            name: frame.ident.clone(),
                            interp_title: self.ctx.rand_interp_title().into(),
                            petname: self.ctx.rand_petname().into(),
                        }));
                    break;
                }
            };

            let op = match sub.bytecode.get(*frame.ins_ptr) {
                Some(op) => op,
                None => {
                    #[cfg(debug_assertions)]
                    println!("\t\t\t| {:?}", self.stack);
                    break;
                }
            };

            let range = op.range;

            match &op.code {
                OpCode::Push(val) => self.stack.push(*val),
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::Swap => {
                    let current = self.stack.pop();
                    let prev = self.stack.pop();

                    if let Some(current) = current
                        && let Some(prev) = prev
                    {
                        self.stack.push(current);
                        self.stack.push(prev);
                    } else {
                        self.ctx.report(diagnostic!(
                            ErrorKind::InsufficientElements {
                                op: "swap".into(),
                                petname: self.ctx.rand_petname().into(),
                                interp_title: self.ctx.rand_interp_title().into(),
                            },
                            labels = [(range, "")]
                        ));
                    }
                }
                OpCode::Add => {
                    let current = self.stack.pop();
                    let prev = self.stack.pop();

                    if let Some(current) = current
                        && let Some(prev) = prev
                    {
                        self.stack.push(current.wrapping_add(prev));
                    } else {
                        self.ctx.report(diagnostic!(
                            ErrorKind::InsufficientElements {
                                op: "add".into(),
                                petname: self.ctx.rand_petname().into(),
                                interp_title: self.ctx.rand_interp_title().into(),
                            },
                            labels = [(range, "")]
                        ));
                    }
                }
                OpCode::Print => {
                    let current = self.stack.pop();

                    if let Some(current) = current {
                        self.printed_output.push(current as char);
                    } else {
                        self.ctx.report(diagnostic!(
                            ErrorKind::InsufficientElements {
                                op: "print".into(),
                                petname: self.ctx.rand_petname().into(),
                                interp_title: self.ctx.rand_interp_title().into(),
                            },
                            labels = [(range, "")]
                        ));
                    }
                }
                OpCode::PrintUtf => {
                    let byte1 = self.stack.pop();
                    let byte2 = self.stack.pop();
                    let byte3 = self.stack.pop();

                    if let Some(byte1) = byte1
                        && let Some(byte2) = byte2
                        && let Some(byte3) = byte3
                    {
                        let bytes = [byte1, byte2, byte3, 0x00];
                        let codepoint = u32::from_le_bytes(bytes);

                        if let Some(char) = char::from_u32(codepoint) {
                            self.printed_output.push(char);
                        } else {
                            self.ctx.report(diagnostic!(
                                ErrorKind::InvalidCodepoint {
                                    petname: self.ctx.rand_petname().into(),
                                    interp_title: self.ctx.rand_interp_title().into(),
                                },
                                labels = [(range, "")]
                            ));
                        }
                    } else {
                        self.ctx.report(diagnostic!(
                            ErrorKind::InsufficientElements {
                                op: "print unicode".into(),
                                petname: self.ctx.rand_petname().into(),
                                interp_title: self.ctx.rand_interp_title().into(),
                            },
                            labels = [(range, "")]
                        ));
                    }
                }
                OpCode::Duplicate => {
                    let current = self.stack.last();

                    if let Some(current) = current {
                        self.stack.push(*current);
                    } else {
                        self.ctx.report(diagnostic!(
                            ErrorKind::InsufficientElements {
                                op: "duplicate".into(),
                                petname: self.ctx.rand_petname().into(),
                                interp_title: self.ctx.rand_interp_title().into(),
                            },
                            labels = [(op.range, "")]
                        ));
                    }
                }
                OpCode::Jump(ident) => {
                    let current = self.stack.last();

                    if current.is_some_and(|current| *current == 0) || current.is_none() {
                        self.create_frame(ident.clone());
                        continue;
                    }
                }
                OpCode::Return => {
                    self.frames.pop();
                }
            }

            #[cfg(debug_assertions)]
            println!(
                "{}..{}\t| {:?}\t| {:?}",
                range.start, range.end, op.code, self.stack
            );

            if let Some(frame) = self.last_frame_mut() {
                *frame.ins_ptr += 1;
            }
        }

        Ok(())
    }
}
