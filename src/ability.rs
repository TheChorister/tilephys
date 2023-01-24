use crate::resources::SceneResources;
use macroquad::time::get_time;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum AbilityType {
    Invulnerability,
    Flight,
}

pub fn ability_name(ability: AbilityType) -> &'static str {
    match ability {
        AbilityType::Invulnerability => "invulnerability",
        AbilityType::Flight => "flight",
    }
}

pub fn ability_name_adj(ability: AbilityType) -> &'static str {
    match ability {
        AbilityType::Invulnerability => "invulnerable",
        AbilityType::Flight => "flying",
    }
}