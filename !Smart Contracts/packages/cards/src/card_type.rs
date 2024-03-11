use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ability::{Ability, AbilityEffect};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum CardType {
    Captain(CaptainData),
    Ship(ShipData),
    Crew(CrewData),
    Skill(SkillData),
}

impl CardType {
    pub fn to_suffix(&self) -> &[u8] {
        match self {
            CardType::Captain(_) => b"captain",
            CardType::Ship(_) => b"ship",
            CardType::Crew(_) => b"crew",
            CardType::Skill(_) => b"skill",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CaptainData {
    pub damage: u8,
    pub health: u8,
    pub luck: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ShipData {
    pub durability: u8,
    pub speed: u8,
    pub capacity: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CrewData {
    pub damage: u8,
    pub health: u8,
    pub abilities: Vec<Ability>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct SkillData {
    pub effect: AbilityEffect,
}
