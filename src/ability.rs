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
    fn update(
        &mut self,
        buffer: &mut CommandBuffer,
        player: &mut Actor,
        player_rect: &IntRect,
        key_state: KeyState,
    ) -> bool;
}


struct Invulnerability {
    time_allowed: f64,
    time_remaining: f64,
    start_time: f64,
    enabled: bool
}


impl Invulnerability {
    fn new(time_allowed: f64) -> Self {
        Self {
            time_allowed,
            time_remaining: time_allowed,
            start_time: get_time(),
            enabled: true
        }
    }
}

impl Ability for Invulnerability {
    fn get_type(&self) -> AbilityType {
        AbilityType::Invulnerability
    }
    fn update(
        &mut self,
        buffer: &mut CommandBuffer,
        player: &mut Actor,
        player_rect: &IntRect,
        key_state: KeyState,
    ) -> bool {
        self.time_remaining = self.time_allowed - self.start_time;
        if self.time_remaining <= 0. {
            self.enabled = false;
        }
        false
    }
}