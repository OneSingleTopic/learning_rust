pub mod associated_traits;
pub mod method_overload;
mod newtype_pattern;
pub mod operator_overload;
pub mod supertraits;

pub use associated_traits::{Counter, CustomIterator};
pub use method_overload::{Animal, Dog, Human, Pilot, Wizard};
pub use newtype_pattern::Wrapper;
pub use operator_overload::Point;
pub use supertraits::{Couple, OutlinePrint};
