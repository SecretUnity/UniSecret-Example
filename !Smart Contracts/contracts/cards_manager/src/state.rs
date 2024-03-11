use schemars::JsonSchema;
use secret_toolkit::{
    serialization::Json,
    storage::{AppendStore, Item, Keymap},
};
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, StdResult, Storage};
use shared::{cards::card_template::CardTemplate, cards_manager::msg::DeckResponse};

pub static CONFIG: Item<Config> = Item::new(b"config");
pub static CARDS: AppendStore<CardBlueprint, Json> = AppendStore::new(b"cards");
pub static WELCOME_PACK: Keymap<Addr, bool> = Keymap::new(b"welcome_pack");
pub static USER_CARDS: AppendStore<CardTemplate, Json> = AppendStore::new(b"user_cards");
pub static USER_DECK: Keymap<Addr, DeckResponse, Json> = Keymap::new(b"user_deck");

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

impl Config {
    pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
        CONFIG.load(storage)
    }

    pub fn save_config(&self, storage: &mut dyn Storage) -> StdResult<()> {
        CONFIG.save(storage, &self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CardBlueprint {
    pub template: CardTemplate,
    pub supply: u32,
    pub in_circulation: u32,
}

impl CardBlueprint {
    pub fn card_type_to_suffix(&self) -> &[u8] {
        self.template.card_type.to_suffix()
    }
}

pub fn load_caards_of_type(
    storage: &dyn Storage,
    card_type: &[u8],
) -> StdResult<Vec<CardTemplate>> {
    Ok(CARDS
        .add_suffix(card_type)
        .iter(storage)?
        .map(|x| x.unwrap().template)
        .collect::<Vec<_>>())
}
