pub mod basicblock;
pub use basicblock::BasicBlock;
pub mod constant;
pub use constant::Constant;
pub mod function;
pub use function::Function;
pub mod instruction;
pub use instruction::Instruction;
// pub mod metadata;
// pub use metadata::Metadata;
pub mod module;
pub use module::Module;
pub mod name;
pub use name::Name;
pub mod operand;
pub use operand::Operand;
pub mod predicates;
pub use predicates::{IntPredicate, FPPredicate};
pub mod terminator;
pub use terminator::Terminator;
pub mod types;
pub use types::{Type, Typed};

mod iterators;
mod from_llvm;
