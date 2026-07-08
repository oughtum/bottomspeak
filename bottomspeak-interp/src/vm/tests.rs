#![cfg(test)]

use pretty_assertions::assert_eq;

use super::*;
use crate::lexer::Lexer;

#[test]
fn test_runtime_eval() -> crate::Result<()> {
    // source is irrelevant for this test
    let mut ctx = SourceContext::new("", "<test>")?;

    let mut lexer = Lexer::new(&mut ctx);
    lexer.lex_tokens();

    let mut sub = Subroutine::default();
    sub.emit_op(Op::new(OpCode::Push(8), 0..0));
    sub.emit_op(Op::new(OpCode::Push(16), 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Print, 0..0));
    sub.emit_op(Op::new(OpCode::Push(28), 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Push(1), 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Print, 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Push(8), 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Print, 0..0));
    sub.emit_op(Op::new(OpCode::Print, 0..0));
    sub.emit_op(Op::new(OpCode::Push(3), 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Print, 0..0));
    sub.emit_op(Op::new(OpCode::Push(32), 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Push(12), 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Print, 0..0));
    sub.emit_op(Op::new(OpCode::Print, 0..0));
    sub.emit_op(Op::new(OpCode::Swap, 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Push(8), 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Push(11), 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Print, 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Push(3), 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Print, 0..0));
    sub.emit_op(Op::new(OpCode::Duplicate, 0..0));
    sub.emit_op(Op::new(OpCode::Push(6), 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Print, 0..0));
    sub.emit_op(Op::new(OpCode::Print, 0..0));
    sub.emit_op(Op::new(OpCode::Print, 0..0));
    sub.emit_op(Op::new(OpCode::Push(1), 0..0));
    sub.emit_op(Op::new(OpCode::Add, 0..0));
    sub.emit_op(Op::new(OpCode::Print, 0..0));
    sub.emit_op(Op::new(OpCode::Return, 0..0));

    let submap = BTreeMap::from([("hawwo".to_string(), sub)]);

    let mut vm = Vm::new(&mut ctx, submap);
    vm.run()?;

    assert_eq!(vm.printed_output, "Hello, world!".to_string());

    Ok(())
}
