use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
    range::Range,
};

use crossterm::style::{Attribute, Color::AnsiValue, Stylize};

use crate::{diagnostic, diagnostics::ErrorKind, source::SourceContext};

pub(crate) mod tests;

pub(crate) const INTERNAL_ROOT_SUBROUTINE: &str = "__root";
const ANSI_ATTRIBUTES: [Attribute; 8] = [
    // 1
    Attribute::Bold,
    // 2
    Attribute::Dim,
    // 4
    Attribute::Italic,
    // 8
    Attribute::Underlined,
    // 16
    Attribute::SlowBlink,
    // 32
    Attribute::Reverse,
    // 64
    Attribute::Hidden,
    // 128
    Attribute::CrossedOut,
];

/// Evaluates a comparison operator.
///
/// # Examples
///
/// ```
/// cmp_op!(self, >=, 0..5);
/// ```
macro_rules! cmp_op {
    ($self:ident, $op:tt, $range:expr) => {{
        let current = $self.stack.last();
        let prev = $self.stack.get($self.stack.len().saturating_sub(2));

        if let Some(current) = current
            && let Some(prev) = prev
        {
            if !(prev $op current)
                && let Some(frame) = $self.last_frame_mut()
            {
                *frame.ins_ptr = frame.ins_ptr.saturating_add(1);
            }
        } else {
            $self.ctx.report(diagnostic!(
                ErrorKind::InsufficientElements {
                    op: "compare".into(),
                    petname: $self.ctx.rand_petname().into(),
                    interp_title: $self.ctx.rand_interp_title().into()
                },
                labels = [($range, "")]
            ))
        }
    }};
}

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
    Eq,
    Greater,
    Less,
    Neq,
    GreaterEq,
    LessEq,
    Push(u8),
    PushScratchPad(u8),
    Pop,
    PopScratchPad,
    Swap,
    Rotate,
    Flip,
    Add,
    Sub,
    Input,
    Print,
    PrintUtf,
    PrintAnsi,
    PrintLiteral,
    PrintStack(bool),
    Duplicate(bool),
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

    /// A scratchpad for inetermediate storage of a single byte.
    scratchpad: Option<u8>,

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
            scratchpad: None,
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

            #[cfg(debug_assertions)]
            let code = op.code.clone();

            match &op.code {
                OpCode::Eq => cmp_op!(self, ==, range),
                OpCode::Greater => cmp_op!(self, >, range),
                OpCode::Less => cmp_op!(self, <, range),
                OpCode::Neq => cmp_op!(self, !=, range),
                OpCode::GreaterEq => cmp_op!(self, >=, range),
                OpCode::LessEq => cmp_op!(self, <=, range),
                OpCode::Push(val) => self.stack.push(*val),
                OpCode::PushScratchPad(val) => {
                    self.scratchpad = Some(*val);
                }
                OpCode::Pop => {
                    if self.stack.is_empty() {
                        self.ctx.report(diagnostic!(
                            ErrorKind::InsufficientElements {
                                op: "pop".to_string(),
                                petname: self.ctx.rand_petname().into(),
                                interp_title: self.ctx.rand_interp_title().into()
                            },
                            labels = [(range, "")]
                        ))
                    }

                    self.stack.pop();
                }
                OpCode::PopScratchPad => {
                    if let Some(val) = self.scratchpad {
                        self.stack.push(val);
                        self.scratchpad = None;
                    } else {
                        self.ctx.report(diagnostic!(
                            ErrorKind::EmptyScratchPad {
                                petname: self.ctx.rand_petname().into(),
                                interp_title: self.ctx.rand_interp_title().into(),
                            },
                            labels = [(range, "")]
                        ));
                    }
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
                OpCode::Rotate => {
                    let val3 = self.stack.pop();
                    let val2 = self.stack.pop();
                    let val1 = self.stack.pop();

                    if let Some(val3) = val3
                        && let Some(val2) = val2
                        && let Some(val1) = val1
                    {
                        self.stack.push(val2);
                        self.stack.push(val3);
                        self.stack.push(val1);
                    } else {
                        self.ctx.report(diagnostic!(
                            ErrorKind::InsufficientElements {
                                op: "rotate".into(),
                                petname: self.ctx.rand_petname().into(),
                                interp_title: self.ctx.rand_interp_title().into(),
                            },
                            labels = [(range, "")]
                        ));
                    }
                }
                OpCode::Flip => self.stack.reverse(),
                OpCode::Add => {
                    let current = self.stack.pop();
                    let prev = self.stack.pop();

                    if let Some(current) = current
                        && let Some(prev) = prev
                    {
                        self.stack.push(prev.wrapping_add(current));
                    } else {
                        self.ctx.report(diagnostic!(
                            ErrorKind::InsufficientElements {
                                op: "add".into(),
                                petname: self.ctx.rand_petname().into(),
                                interp_title: self.ctx.rand_interp_title().into()
                            },
                            labels = [(range, "")]
                        ))
                    }
                }
                OpCode::Sub => {
                    let current = self.stack.pop();
                    let prev = self.stack.pop();

                    if let Some(current) = current
                        && let Some(prev) = prev
                    {
                        self.stack.push(prev.wrapping_sub(current));
                    } else {
                        self.ctx.report(diagnostic!(
                            ErrorKind::InsufficientElements {
                                op: "sub".into(),
                                petname: self.ctx.rand_petname().into(),
                                interp_title: self.ctx.rand_interp_title().into(),
                            },
                            labels = [(range, "")]
                        ));
                    }
                }
                OpCode::Input => {
                    let byte = self.ctx.input.pop();

                    if let Some(byte) = byte {
                        self.stack.push(byte);
                    } else {
                        self.ctx.report(diagnostic!(
                            ErrorKind::NoInput {
                                interp_title: self.ctx.rand_interp_title().into(),
                                petname: self.ctx.rand_petname().into(),
                                praise_term: self.ctx.rand_praise_term().into()
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
                                    codepoint,
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
                OpCode::PrintAnsi => {
                    let modifier = self.stack.pop();
                    let bg = self.stack.pop();
                    let fg = self.stack.pop();
                    let char = self.stack.pop();

                    if let Some(modifier) = modifier
                        && let Some(bg) = bg
                        && let Some(fg) = fg
                        && let Some(char) = char
                    {
                        let mut char = (char as char).with(AnsiValue(fg)).on(AnsiValue(bg));

                        for i in 0..8 {
                            let mask = 1 << i;
                            let is_set = (modifier & mask) > 0;

                            if is_set {
                                char = char.attribute(ANSI_ATTRIBUTES[i]);
                            }
                        }

                        self.printed_output.push_str(&char.to_string());
                    } else {
                        self.ctx.report(diagnostic!(
                            ErrorKind::InsufficientElements {
                                op: "print with colour".into(),
                                petname: self.ctx.rand_petname().into(),
                                interp_title: self.ctx.rand_interp_title().into(),
                            },
                            labels = [(range, "")]
                        ));
                    }
                }
                OpCode::PrintLiteral => {
                    let current = self.stack.pop();

                    if let Some(current) = current {
                        self.printed_output.push_str(&current.to_string());
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
                OpCode::PrintStack(pretty) => {
                    let scratchpad = self
                        .scratchpad
                        .map(|byte| byte.to_string())
                        .unwrap_or_default();

                    if *pretty {
                        println!("[{}]:{:#?}", scratchpad, self.stack);
                    } else {
                        println!("[{}]:{:?}", scratchpad, self.stack);
                    }
                }
                OpCode::Duplicate(double) => {
                    if *double && self.stack.len() <= 1 || !double && self.stack.is_empty() {
                        self.ctx.report(diagnostic!(
                            ErrorKind::InsufficientElements {
                                op: "duplicate".into(),
                                petname: self.ctx.rand_petname().into(),
                                interp_title: self.ctx.rand_interp_title().into(),
                            },
                            labels = [(op.range, "")]
                        ));
                    } else {
                        let current = *self.stack.last().unwrap();

                        if *double {
                            let prev = *self.stack.get(self.stack.len().saturating_sub(2)).unwrap();

                            self.stack.push(prev);
                        }

                        self.stack.push(current);
                    }
                }
                OpCode::Jump(ident) => {
                    self.create_frame(ident.clone());
                    continue;
                }
                OpCode::Return => {
                    self.frames.pop();
                }
            }

            #[cfg(debug_assertions)]
            println!(
                "{}..{}\t| {:?}\t| {:?}",
                range.start, range.end, code, self.stack
            );

            if let Some(frame) = self.last_frame_mut() {
                *frame.ins_ptr += 1;
            }
        }

        Ok(())
    }
}
