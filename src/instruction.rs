//! LLVM Instruction class

use std::rc::Rc;
use super::basic_block::BasicBlock;
use super::value::Value;

/// The Instruction trait is similar to the [Any] trait.
pub trait Instruction: Value {
}

/// Any instruction. Forms a circular linked-list.
// TODO: Possibly reconsider this structure for storing
// instructions. 
pub struct InstructionListNode {
    parent: Rc<BasicBlock>,
    next: Rc<InstructionListNode>,
    prev: Rc<InstructionListNode>,
    instr: Box<dyn Instruction>,
}
