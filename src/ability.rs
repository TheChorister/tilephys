use crate::physics::IntRect;
use crate::draw::PlayerSprite;
use macroquad::time::get_time;

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
    mut timeRemaining: i32,
    start_time: f64
}


impl Invulnerability {
    fn new(timeAllowed: f64) -> Self {
        Self {
            timeAllowed,
            timeRemaining: timeAllowed,
            start_time: get_time()
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
            player
        }
        false
    }
}