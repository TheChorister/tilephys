
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum AbilityType {
    Ivulnerability,
    Flying,
}

pub fn ability_name(ability: AbilityType) -> &'static str {
    match ability {
        AbilityType::Ivulnerability => "invulnerability",
        AbilityType::Flying => "flying",
    }
}