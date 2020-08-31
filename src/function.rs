//! LLVM Function.

use std::rc::Rc;
use super::basic_block::BasicBlock;

pub struct Function {
    args: Vec<Argument>,
    entry_block: Rc<BasicBlock>,
}

/// Argument to a function.
pub struct Argument {
    parent: Rc<Function>,
    name: String,
}
