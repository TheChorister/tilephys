use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum AbilityType {
    Invulnerability,
    Flight,
    QuantumTunneling,
}

pub fn ability_name(ability: AbilityType) -> &'static str {
    match ability {
        AbilityType::Invulnerability => "invulnerability",
        AbilityType::Flight => "flight",
        AbilityType::QuantumTunneling => "quantum tunnel ability",
    }
}

pub fn ability_name_adj(ability: AbilityType) -> &'static str {
    match ability {
        AbilityType::Invulnerability => "invulnerable",
        AbilityType::Flight => "flying",
        AbilityType::QuantumTunneling => "quantum tunneling",
    }
}

pub struct Abilities {
    _abilities: HashSet<AbilityType>,
}

impl Abilities {
    pub fn new() -> Self {
        Self {
            _abilities: HashSet::new(),
        }
    }

    pub fn learn(&mut self, ability: AbilityType) {
        self._abilities.insert(ability);
    }

    pub fn forget(&mut self, ability: AbilityType) {
        self._abilities.remove(&ability);
    }

    pub fn can(&self, ability: AbilityType) -> bool {
        self._abilities.contains(&ability)
    }
}