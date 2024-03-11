use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::card_type::CardType;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CardTemplate {
    pub name: String,
    pub description: String,
    pub image: String,
    pub card_type: CardType,
}
