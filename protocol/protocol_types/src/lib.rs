pub mod character;
pub mod heros;
pub mod spell;

pub mod prelude {
    pub use crate::character::*;
    pub use crate::heros::*;
    pub use crate::spell::*;
}
