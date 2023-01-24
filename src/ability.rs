use std::collections::HashSet;

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

pub fn ability_name_verb(ability: AbilityType) -> &'static str {
    match ability {
        AbilityType::Invulnerability => "be invulnerable",
        AbilityType::Flight => "fly",
    }
}

pub struct Abilities {
    _abilities: HashSet<AbilityType>,
    pub learn_queue: HashSet<AbilityType>,
    pub forget_queue: HashSet<AbilityType>,
}

impl Abilities {
    pub fn new() -> Self {
        Self {
            _abilities: HashSet::new(),
            learn_queue: HashSet::new(),
            forget_queue: HashSet::new(),
        }
    }

    pub fn learn(&mut self, ability: AbilityType) {
        self.learn_queue.insert(ability);
    }

    pub fn forget(&mut self, ability: AbilityType) {
        self.forget_queue.insert(ability);
    }

    pub fn can(&self, ability: AbilityType) -> bool {
        self._abilities.contains(&ability)
    }

    pub fn update(&mut self) {
        for ability in self.learn_queue.iter() {
            self._abilities.insert(*ability);
        }
        for ability in self.forget_queue.iter() {
            self._abilities.remove(ability);
        }
        self.learn_queue.clear();
        self.forget_queue.clear();
    }

}