use std::collections::HashSet;

use crate::input::KeyState;
use crate::physics::{Actor, IntRect};
use hecs::CommandBuffer;
use crate::draw::PlayerSprite;
use macroquad::time::get_time;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum AbilityType {
    Invulnerability,
    Flying,
}

pub fn ability_name(ability: AbilityType) -> &'static str {
    match ability {
        AbilityType::Invulnerability => "invulnerability",
        AbilityType::Flying => "flying",
    }
}

pub trait Ability {
    fn get_type(&self) -> AbilityType;
    fn update(&mut self);
}


struct Invulnerability {
    time_allowed: Option<f64>,
    time_remaining: f64,
    start_time: f64,
    enabled: bool
}


impl Invulnerability {
    fn new(time_allowed: Option<f64>) -> Self {
        Self {
            time_allowed,
            time_remaining: time_allowed.unwrap_or(get_time() + 1.),
            start_time: get_time(),
            enabled: true
        }
    }
}

impl Ability for Invulnerability {
    fn get_type(&self) -> AbilityType {
        AbilityType::Invulnerability
    }
    fn update(&mut self) {
        self.time_remaining = self.time_allowed.unwrap_or(self.start_time + 1.) - self.start_time;
        if self.time_remaining <= 0. {
            self.enabled = false;
        }
    }
}

pub fn new_ability(typ: AbilityType, time_allowed: Option<f64>) -> Box<dyn Ability> {
    match typ {
        AbilityType::Invulnerability => Box::new(Invulnerability::new(time_allowed)),
        AbilityType::Flying => Box::new(Invulnerability::new(time_allowed)),
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

    pub fn update(&self) {}
}