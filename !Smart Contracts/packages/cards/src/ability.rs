use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Ability {
    pub name: String,
    pub description: String,
    pub definition: AbilityDefinition,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AbilityDefinition {
    pub target: AbilityTarget,
    pub effect: AbilityEffect,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum AbilityTarget {
    User,
    Opponent(AbilityTargetSpecifier),
    Ally(AbilityTargetSpecifier),
    Either(AbilityTargetSpecifier),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum AbilityTargetSpecifier {
    All,
    Specific,
    Random,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AbilityEffect {
    pub effect_type: AbilityEffectType,
    pub cost: u8,
    pub target_animation: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum AbilityEffectType {
    Single(Action),
    Composite(Vec<Action>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum Action {
    Damage(Expression),
    Heal(Expression),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum Expression {
    Add(ExpressionValue, ExpressionValue),
    Subtract(ExpressionValue, ExpressionValue),
    Multiply(ExpressionValue, ExpressionValue),
    Divide(ExpressionValue, ExpressionValue),
    Constant(u8),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub enum ExpressionValue {
    Constant(u8),
    CaptainHealth,
    CaptainDamage,
    CaptainLuck,
    ShipDurability,
    ShipSpeed,
}
