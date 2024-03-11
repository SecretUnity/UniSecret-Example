use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    error::ContractError,
    state::{
        load_caards_of_type, CardBlueprint, Config, CARDS, USER_CARDS, USER_DECK, WELCOME_PACK,
    },
};
use shared::{
    cards::card_template::CardTemplate,
    cards_manager::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QualificationResponse, QueryMsg},
};

type ContractResult = Result<Response, ContractError>;

fn prep_cards() -> Vec<CardTemplate> {
    vec![
        // Unique captains
        CardTemplate {
            name: "Trevor Ironhook".to_string(),
            description: "A fierce pirate captain with a hook for a hand.".to_string(),
            image: "trevor_ironhook.png".to_string(),
            card_type: shared::cards::card_type::CardType::Captain(
                shared::cards::card_type::CaptainData {
                    damage: 3,
                    health: 5,
                    luck: 92,
                },
            ),
        },
        CardTemplate {
            name: "Salty Dog".to_string(),
            description: "A salty old sea dog with a peg leg and a parrot.".to_string(),
            image: "salty_dog.png".to_string(),
            card_type: shared::cards::card_type::CardType::Captain(
                shared::cards::card_type::CaptainData {
                    damage: 6,
                    health: 3,
                    luck: 85,
                },
            ),
        },
        CardTemplate {
            name: "Blackbeard".to_string(),
            description: "The most feared pirate of all time, with a beard to match.".to_string(),
            image: "blackbeard.png".to_string(),
            card_type: shared::cards::card_type::CardType::Captain(
                shared::cards::card_type::CaptainData {
                    damage: 8,
                    health: 8,
                    luck: 200,
                },
            ),
        },
        CardTemplate {
            name: "Anne Bonny".to_string(),
            description: "A fierce pirate queen who will stop at nothing to get her way."
                .to_string(),
            image: "anne_bonny.png".to_string(),
            card_type: shared::cards::card_type::CardType::Captain(
                shared::cards::card_type::CaptainData {
                    damage: 5,
                    health: 4,
                    luck: 100,
                },
            ),
        },
        CardTemplate {
            name: "Calico Jack".to_string(),
            description: "A pirate captain with a love for fine clothes and fine wine.".to_string(),
            image: "calico_jack.png".to_string(),
            card_type: shared::cards::card_type::CardType::Captain(
                shared::cards::card_type::CaptainData {
                    damage: 4,
                    health: 6,
                    luck: 80,
                },
            ),
        },
        // Unique ships
        CardTemplate {
            name: "The Black Pearl".to_string(),
            description: "A fast and deadly ship with a crew of undead pirates.".to_string(),
            image: "black_pearl.png".to_string(),
            card_type: shared::cards::card_type::CardType::Ship(
                shared::cards::card_type::ShipData {
                    durability: 8,
                    speed: 10,
                    capacity: 8,
                },
            ),
        },
        CardTemplate {
            name: "The Flying Dutchman".to_string(),
            description: "A ghostly ship that can sail through the very fabric of reality."
                .to_string(),
            image: "flying_dutchman.png".to_string(),
            card_type: shared::cards::card_type::CardType::Ship(
                shared::cards::card_type::ShipData {
                    durability: 12,
                    speed: 6,
                    capacity: 10,
                },
            ),
        },
        CardTemplate {
            name: "The Jolly Roger".to_string(),
            description: "A ship with a fearsome reputation and a crew to match.".to_string(),
            image: "jolly_roger.png".to_string(),
            card_type: shared::cards::card_type::CardType::Ship(
                shared::cards::card_type::ShipData {
                    durability: 10,
                    speed: 8,
                    capacity: 12,
                },
            ),
        },
        CardTemplate {
            name: "The Queen Anne's Revenge".to_string(),
            description: "A ship with a fearsome reputation and a crew to match.".to_string(),
            image: "queen_annes_revenge.png".to_string(),
            card_type: shared::cards::card_type::CardType::Ship(
                shared::cards::card_type::ShipData {
                    durability: 8,
                    speed: 8,
                    capacity: 8,
                },
            ),
        },
        CardTemplate {
            name: "The Sea Witch".to_string(),
            description: "A ship with a fearsome reputation and a crew to match.".to_string(),
            image: "sea_witch.png".to_string(),
            card_type: shared::cards::card_type::CardType::Ship(
                shared::cards::card_type::ShipData {
                    durability: 6,
                    speed: 8,
                    capacity: 5,
                },
            ),
        },
        CardTemplate {
            name: "The Wicked Wench".to_string(),
            description: "A ship with a fearsome reputation and a crew to match.".to_string(),
            image: "wicked_wench.png".to_string(),
            card_type: shared::cards::card_type::CardType::Ship(
                shared::cards::card_type::ShipData {
                    durability: 8,
                    speed: 8,
                    capacity: 8,
                },
            ),
        },
        CardTemplate {
            name: "Cardboard Box".to_string(),
            description: "A ship with a fearsome reputation and a crew to match... or not."
                .to_string(),
            image: "cardboard_box.png".to_string(),
            card_type: shared::cards::card_type::CardType::Ship(
                shared::cards::card_type::ShipData {
                    durability: 1,
                    speed: 1,
                    capacity: 1,
                },
            ),
        },
        CardTemplate {
            name: "Poo boat (no fans)".to_string(),
            description: "You're not sure how this thing floats, but it does.".to_string(),
            image: "poo_boat.png".to_string(),
            card_type: shared::cards::card_type::CardType::Ship(
                shared::cards::card_type::ShipData {
                    durability: 2,
                    speed: 2,
                    capacity: 2,
                },
            ),
        },
        // Unique crew
        CardTemplate {
            name: "Peg Leg Pete".to_string(),
            description: "A pirate with a peg leg and a heart of gold.".to_string(),
            image: "peg_leg_pete.png".to_string(),
            card_type: shared::cards::card_type::CardType::Crew(
                shared::cards::card_type::CrewData {
                    damage: 3,
                    health: 3,
                    abilities: vec![shared::cards::ability::Ability {
                        name: "Swashbuckling".to_string(),
                        description: "This crew member can attack twice in one turn.".to_string(),
                        definition: shared::cards::ability::AbilityDefinition {
                            target: shared::cards::ability::AbilityTarget::Opponent(
                                shared::cards::ability::AbilityTargetSpecifier::Specific,
                            ),
                            effect: shared::cards::ability::AbilityEffect {
                                effect_type: shared::cards::ability::AbilityEffectType::Single(
                                    shared::cards::ability::Action::Damage(
                                        shared::cards::ability::Expression::Constant(2),
                                    ),
                                ),
                                cost: 2,
                                target_animation: "swashbuckling".to_string(),
                            },
                        },
                    }],
                },
            ),
        },
        CardTemplate {
            name: "One-eyed Thad".to_string(),
            description: "This pirate has only one eye, but he never misses. At least, not twice."
                .to_string(),
            image: "one_eyed_thad.png".to_string(),
            card_type: shared::cards::card_type::CardType::Crew(
                shared::cards::card_type::CrewData {
                    damage: 4,
                    health: 2,
                    abilities: vec![],
                },
            ),
        },
        CardTemplate {
            name: "Fisherman Fred".to_string(),
            description: "A fisherman who's never been to sea, but he's willing to learn."
                .to_string(),
            image: "fisherman_fred.png".to_string(),
            card_type: shared::cards::card_type::CardType::Crew(
                shared::cards::card_type::CrewData {
                    damage: 1,
                    health: 1,
                    abilities: vec![shared::cards::ability::Ability {
                        name: "Cast Net".to_string(),
                        description: "This crew member can feed the crew for a turn.".to_string(),
                        definition: shared::cards::ability::AbilityDefinition {
                            target: shared::cards::ability::AbilityTarget::Ally(
                                shared::cards::ability::AbilityTargetSpecifier::All,
                            ),
                            effect: shared::cards::ability::AbilityEffect {
                                effect_type: shared::cards::ability::AbilityEffectType::Single(
                                    shared::cards::ability::Action::Heal(
                                        shared::cards::ability::Expression::Constant(1),
                                    ),
                                ),
                                cost: 1,
                                target_animation: "cast_net".to_string(),
                            },
                        },
                    }],
                },
            ),
        },
        CardTemplate {
            name: "Scurvy Steve".to_string(),
            description: "A pirate with a bad case of scurvy, but he's still a good fighter."
                .to_string(),
            image: "scurvy_steve.png".to_string(),
            card_type: shared::cards::card_type::CardType::Crew(
                shared::cards::card_type::CrewData {
                    damage: 3,
                    health: 2,
                    abilities: vec![],
                },
            ),
        },
        CardTemplate {
            name: "Cannonball Carl".to_string(),
            description: "A pirate who's as round as a cannonball, but twice as deadly."
                .to_string(),
            image: "cannonball_carl.png".to_string(),
            card_type: shared::cards::card_type::CardType::Crew(
                shared::cards::card_type::CrewData {
                    damage: 5,
                    health: 3,
                    abilities: vec![],
                },
            ),
        },
        // Unique skills
        CardTemplate {
            name: "Walk the Plank".to_string(),
            description: "Send an enemy crew member to Davy Jones' locker.".to_string(),
            image: "walk_the_plank.png".to_string(),
            card_type: shared::cards::card_type::CardType::Skill(
                shared::cards::card_type::SkillData {
                    effect: shared::cards::ability::AbilityEffect {
                        effect_type: shared::cards::ability::AbilityEffectType::Single(
                            shared::cards::ability::Action::Damage(
                                shared::cards::ability::Expression::Constant(5),
                            ),
                        ),
                        cost: 5,
                        target_animation: "walk_the_plank".to_string(),
                    },
                },
            ),
        },
        CardTemplate {
            name: "Hoist the Colours".to_string(),
            description: "Boost your crew's morale and give them a second wind.".to_string(),
            image: "hoist_the_colours.png".to_string(),
            card_type: shared::cards::card_type::CardType::Skill(
                shared::cards::card_type::SkillData {
                    effect: shared::cards::ability::AbilityEffect {
                        effect_type: shared::cards::ability::AbilityEffectType::Composite(vec![
                            shared::cards::ability::Action::Heal(
                                shared::cards::ability::Expression::Constant(2),
                            ),
                        ]),
                        cost: 4,
                        target_animation: "hoist_the_colours".to_string(),
                    },
                },
            ),
        },
        CardTemplate {
            name: "Shiver Me Timbers".to_string(),
            description: "Strike fear into the hearts of your enemies.".to_string(),
            image: "shiver_me_tibers.png".to_string(),
            card_type: shared::cards::card_type::CardType::Skill(
                shared::cards::card_type::SkillData {
                    effect: shared::cards::ability::AbilityEffect {
                        effect_type: shared::cards::ability::AbilityEffectType::Single(
                            shared::cards::ability::Action::Damage(
                                shared::cards::ability::Expression::Constant(1),
                            ),
                        ),
                        cost: 3,
                        target_animation: "shiver_me_timbers".to_string(),
                    },
                },
            ),
        },
        CardTemplate {
            name: "Yo Ho Ho".to_string(),
            description: "Boost your crew's morale and give them a second wind.".to_string(),
            image: "yo_ho_ho.png".to_string(),
            card_type: shared::cards::card_type::CardType::Skill(
                shared::cards::card_type::SkillData {
                    effect: shared::cards::ability::AbilityEffect {
                        effect_type: shared::cards::ability::AbilityEffectType::Composite(vec![
                            shared::cards::ability::Action::Heal(
                                shared::cards::ability::Expression::Constant(1),
                            ),
                        ]),
                        cost: 2,
                        target_animation: "yo_ho_ho".to_string(),
                    },
                },
            ),
        },
    ]
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        owner: info.sender.clone(),
    };

    //deps.api
    //    .debug(format!("Contract was initialized by {}", info.sender).as_str());
    config.save_config(deps.storage)?;

    prep_cards().iter().for_each(|card| {
        CARDS
            .add_suffix(card.card_type.to_suffix())
            .push(
                deps.storage,
                &CardBlueprint {
                    template: card.clone(),
                    supply: 100,
                    in_circulation: 0,
                },
            )
            .unwrap();
    });
    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> ContractResult {
    match msg {
        ExecuteMsg::NewCard { card, supply } => new_card(deps, info, card, supply),
        ExecuteMsg::ReceiveWelcomePack {} => receive_welcome_pack(deps, env, info),
        ExecuteMsg::BuyBoosterPack {} => buy_booster_pack(deps, env, info),
    }
}

fn new_card(deps: DepsMut, info: MessageInfo, card: CardTemplate, supply: u32) -> ContractResult {
    let cfg = Config::read_config(deps.storage)?;
    if cfg.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let card_blueprint = CardBlueprint {
        template: card.clone(),
        supply,
        in_circulation: 0,
    };
    CARDS
        .add_suffix(card.card_type.to_suffix())
        .push(deps.storage, &card_blueprint)?;
    Ok(Response::default())
}

fn receive_welcome_pack(deps: DepsMut, env: Env, info: MessageInfo) -> ContractResult {
    if WELCOME_PACK.contains(deps.storage, &info.sender) {
        return Err(ContractError::Unauthorized {});
    }
    let rand = env.block.random.unwrap();
    let captains = load_caards_of_type(deps.storage, b"captain")?;
    let ships = load_caards_of_type(deps.storage, b"ship")?;
    let crews = load_caards_of_type(deps.storage, b"crew")?;
    let skills = load_caards_of_type(deps.storage, b"skill")?;

    let mut new_cards: Vec<CardTemplate> = vec![];
    let captain_picks = vec![
        captains[rand[0] as usize % captains.len()].clone(),
        captains[rand[1] as usize % captains.len()].clone(),
        captains[rand[2] as usize % captains.len()].clone(),
    ];
    let ship_picks = vec![
        ships[rand[3] as usize % ships.len()].clone(),
        ships[rand[4] as usize % ships.len()].clone(),
    ];
    let crew_picks = vec![
        crews[rand[5] as usize % crews.len()].clone(),
        crews[rand[6] as usize % crews.len()].clone(),
        crews[rand[7] as usize % crews.len()].clone(),
        crews[rand[8] as usize % crews.len()].clone(),
        crews[rand[9] as usize % crews.len()].clone(),
    ];
    let skill_picks = vec![
        skills[rand[10] as usize % skills.len()].clone(),
        skills[rand[11] as usize % skills.len()].clone(),
        skills[rand[12] as usize % skills.len()].clone(),
    ];

    new_cards.extend(captain_picks);
    new_cards.extend(ship_picks);
    new_cards.extend(crew_picks);
    new_cards.extend(skill_picks);

    for card in new_cards {
        USER_CARDS
            .add_suffix(info.sender.to_string().as_bytes())
            .push(deps.storage, &card)?;
    }

    WELCOME_PACK.insert(deps.storage, &info.sender, &true)?;
    Ok(Response::default())
}

fn buy_booster_pack(deps: DepsMut, env: Env, info: MessageInfo) -> ContractResult {
    let captains = load_caards_of_type(deps.storage, b"captain")?;
    let ships = load_caards_of_type(deps.storage, b"ship")?;
    let crews = load_caards_of_type(deps.storage, b"crew")?;
    let skills = load_caards_of_type(deps.storage, b"skill")?;
    let rand = env.block.random.unwrap();
    let mut all: Vec<&CardTemplate> = vec![];
    all.extend(captains.iter());
    all.extend(ships.iter());
    all.extend(crews.iter());
    all.extend(skills.iter());
    let picks: Vec<&CardTemplate> = vec![
        all[rand[0] as usize % all.len()],
        all[rand[1] as usize % all.len()],
        all[rand[2] as usize % all.len()],
        all[rand[3] as usize % all.len()],
        all[rand[4] as usize % all.len()],
    ];

    for card in picks {
        USER_CARDS
            .add_suffix(info.sender.to_string().as_bytes())
            .push(deps.storage, card)?;
    }
    Ok(Response::default())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QualifiedForWelcomePack { address } => to_binary(&QualificationResponse {
            qualified: !WELCOME_PACK.contains(deps.storage, &address),
        }),
        QueryMsg::OwnedCards { address } => to_binary(
            &USER_CARDS
                .add_suffix(address.to_string().as_bytes())
                .iter(deps.storage)?
                .map(|x| x.unwrap())
                .collect::<Vec<_>>(),
        ),
        QueryMsg::Deck { address } => to_binary(&USER_DECK.get(deps.storage, &address).unwrap()),
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::*;
    use cosmwasm_std::{from_binary, Coin, StdError, Uint128};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let result =
        try_parse_code(deps.as_mut(), "echo (a); echo (b); reverse(ab);".to_owned()).unwrap();
        let output = result.attributes.first().unwrap().value.clone();
        println!("{}", &output);
        assert_eq!("a\n", output);
    }
}

*/
