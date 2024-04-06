use serde::{Deserialize, Serialize};

use crate::prelude::Ability;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Spell {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub cost: u8,
    pub lvl: u8,
    pub abilities: Vec<Ability>,
}
