use cards::card_template::CardTemplate;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub message: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    NewCard { card: CardTemplate, supply: u32 },
    ReceiveWelcomePack {},
    BuyBoosterPack {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetOwner returns the current owner of the contract
    #[returns(QualificationResponse)]
    QualifiedForWelcomePack { address: Addr },
    #[returns(CardsResponse)]
    OwnedCards { address: Addr },
    #[returns(DeckResponse)]
    Deck { address: Addr },
}

#[cw_serde]
pub struct QualificationResponse {
    pub qualified: bool,
}

// We define a custom struct for each query response
#[cw_serde]
pub struct CardsResponse {
    pub cards: Vec<CardTemplate>,
}

#[cw_serde]
pub struct DeckResponse {
    pub ship: CardTemplate,
    pub captain: CardTemplate,
    pub crew: Vec<CardTemplate>,
}

#[cw_serde]
pub struct MigrateMsg {}
