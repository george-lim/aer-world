mod allegiance;
mod armor;
mod health;
mod position;
mod reaction;

pub use allegiance::*;
pub use armor::*;
pub use health::*;
pub use position::*;
pub use reaction::*;

pub mod components {
    pub use super::Allegiance;
    pub use super::Armor;
    pub use super::Health;
    pub use super::Position;
    pub use super::Reaction;
}
