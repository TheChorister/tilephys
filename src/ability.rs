use crate::resources::SceneResources;
use macroquad::time::get_time;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum AbilityTypeRaw {
    Invulnerability,
    Flight,
}

pub enum AbilityType {
    Invulnerability(f64, Option<f64>),
    Flight(f64, Option<f64>),
}

pub fn ability_name(ability: AbilityTypeRaw) -> &'static str {
    match ability {
        AbilityTypeRaw::Invulnerability => "invulnerability",
        AbilityTypeRaw::Flight => "flight",
    }
}

pub fn ability_name_adj(ability: AbilityTypeRaw) -> &'static str {
    match ability {
        AbilityTypeRaw::Invulnerability => "invulnerable",
        AbilityTypeRaw::Flight => "flying",
    }
}

impl AbilityType {

    fn get_remaining_time_from_args(start_time: f64, time_allowed: Option<f64>) -> f64 {
        match time_allowed {
            Some(t) => t + start_time - get_time(),
            None => 1.,
        }
    }

    fn get_remaining_time(&self) -> f64 {
        match self {
            AbilityType::Invulnerability(start_time, time_allowed) => Self::get_remaining_time_from_args(*start_time, *time_allowed),
            AbilityType::Flight(start_time, time_allowed) => Self::get_remaining_time_from_args(*start_time, *time_allowed),
        }
        
    }

    fn update(&self) -> bool {
        self.get_remaining_time() <= 0.
    }
}

pub struct Abilities {
    _abilities: Vec<AbilityType>,
}

impl Abilities {
    pub fn new() -> Self {
        Self {
            _abilities: Vec::new(),
        }
    }

    pub fn new_ability(&mut self, ability: AbilityType) {
        self._abilities.push(ability);
    }

    pub fn has_ability(&self, typ: AbilityType) -> bool {
        for ability in self._abilities.iter() {
            if ability.get_type() == typ {
                return true;
            }
        }
        false
    }

    pub fn lose_ability(&mut self, i: usize) {
        self._abilities.remove(i);
    }

    pub fn update(&mut self) {
        for (i, ability) in self._abilities.iter_mut().enumerate() {
            if !ability.update() {
                self._abilities.remove(i);
            }
        }
    }
}