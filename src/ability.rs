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

pub trait Ability {
    fn get_type(&self) -> AbilityType;
    fn update(&self) -> bool;
    fn get_remaining_time(&self) -> f64;
}


struct Invulnerability {
    time_allowed: Option<f64>,
    pub time_remaining: f64,
    start_time: f64,
}


impl Invulnerability {
    fn new(time_allowed: Option<f64>) -> Self {
        Self {
            time_allowed,
            time_remaining: time_allowed.unwrap_or(get_time() + 1.),
            start_time: get_time(),
        }
    }
}

impl Ability for Invulnerability {
    fn get_type(&self) -> AbilityType {
        AbilityType::Invulnerability
    }

    fn get_remaining_time(&self) -> f64 {
        match self.time_allowed {
            Some(t) => t + self.start_time - get_time(),
            None => 1.,
        }
    }

    fn update(&self) -> bool {
        self.get_remaining_time() <= 0.
    }
}

pub fn new_ability(typ: AbilityType, time_allowed: Option<f64>) -> Box<dyn Ability> {
    match typ {
        AbilityType::Invulnerability => Box::new(Invulnerability::new(time_allowed)),
        AbilityType::Flight => Box::new(Invulnerability::new(time_allowed)),
    }
}

pub struct Abilities {
    _abilities: Vec<Box<dyn Ability>>,
}

impl Abilities {
    pub fn new() -> Self {
        Self {
            _abilities: Vec::new(),
        }
    }

    pub fn new_ability(&mut self, typ: AbilityType, time_allowed: Option<f64>) {
        self._abilities.push(new_ability(typ, time_allowed));
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