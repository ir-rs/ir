//! LLVM Basic Block.

use std::rc::Rc;
use super::function::Function;
use super::value::Value;
use super::instruction::{Instruction, InstructionListNode};

/// Basic Block. A [Value] that is composed of a list of instructions.
// TODO: List the conditions for a basic block to be well formed in these
// doc comments.
pub struct BasicBlock {
    parent: Rc<Function>,
    instrs: InstructionListNode,
}

impl Value for BasicBlock {
}

/// Sentinel value for the end of a basic block instruction list.
pub struct EndBasicBlock {}

impl Value for EndBasicBlock {}
impl Instruction for EndBasicBlock {}



