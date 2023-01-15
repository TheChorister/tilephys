use crate::draw::PlayerSprite;
use crate::input::{Input, VirtualKey};
use crate::physics::{Actor, IntRect, Secrecy, TriggerZone};
use crate::projectile::make_player_projectile;
use crate::resources::SceneResources;
use crate::switch::Switch;
use crate::vfx::create_explosion;
use hecs::CommandBuffer;
use macroquad::prelude::{is_key_down, KeyCode};
use std::collections::HashSet;

pub struct Controller {
    jump_frames: u32,
    triggers: HashSet<String>,
    facing: i8,
    fire_timer: u32,
    hurt_timer: u8,
    pub hp: u8,
    god_mode: bool,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            jump_frames: 0,
            triggers: HashSet::new(),
            facing: 1,
            fire_timer: 100000,
            hurt_timer: 0,
            hp: 3,
            god_mode: false,
        }
    }

    pub fn update(resources: &mut SceneResources, buffer: &mut CommandBuffer, input: &Input) {
        let world = resources.world_ref.lock().unwrap();
        let mut q = world.query::<(&mut Actor, &IntRect, &mut PlayerSprite, &mut Controller)>();
        for (id, (player, p_rect, sprite, controller)) in q.iter() {
            let mut new_triggers: HashSet<String> = HashSet::new();
            for (_, (trigger, t_rect)) in world.query::<(&mut TriggerZone, &IntRect)>().iter() {
                if p_rect.intersects(t_rect) {
                    let name = format!("{}_enter", trigger.name).to_owned();
                    if !controller.triggers.contains(&name) {
                        resources.triggers.insert(name.clone());
                        if trigger.secrecy == Secrecy::Hidden {
                            trigger.secrecy = Secrecy::Found;
                            resources.stats.secrets += 1;
                            resources.messages.add("Found a secret area!".to_owned());
                        }
                    }
                    new_triggers.insert(name);
                }
            }
            controller.triggers = new_triggers;
            if input.is_down(VirtualKey::Left) {
                player.vx -= 3.0;
                controller.facing = -1;
                sprite.flipped = false;
            }
            if input.is_down(VirtualKey::Right) {
                player.vx += 3.0;
                controller.facing = 1;
                sprite.flipped = true;
            }
            if input.is_pressed(VirtualKey::Interact) {
                let mut q = world.query::<(&Actor, &IntRect, &mut Switch)>();
                for (_, (_, s_rect, s)) in q.iter() {
                    if p_rect.intersects(s_rect) && s.enabled {
                        resources
                            .triggers
                            .insert(format!("{}_interact", s.name).to_owned());
                        s.enabled = false;
                    }
                }
            }
            if input.is_pressed(VirtualKey::Fire) {
                let new_x = p_rect.x + 3 + controller.facing as i32 * 9;
                let rect = IntRect::new(new_x, p_rect.y + 11, 8, 5);
                make_player_projectile(buffer, rect, controller.facing as f32 * 10.0);
                player.vx -= controller.facing as f32 * 10.0;
                controller.fire_timer = 0;
                sprite.firing = true;
            }
            if player.grounded && input.is_pressed(VirtualKey::Jump) {
                player.vy = -6.0;
                controller.jump_frames = 5;
            } else if controller.jump_frames > 0
                && input.is_down(VirtualKey::Jump)
                && player.vy < 0.0
            {
                player.vy = -10.0;
                controller.jump_frames -= 1;
            } else {
                controller.jump_frames = 0;
            }
            if player.grounded {
                sprite.n += player.vx.abs() as i32;
            }
            controller.fire_timer += 1;
            sprite.muzzle_flash = controller.fire_timer.min(100) as u8;
            if controller.fire_timer > 5 {
                sprite.firing = false;
            }
            if controller.hurt_timer > 0 {
                controller.hurt_timer -= 1;
                sprite.blink = (controller.hurt_timer / 3) % 2 == 0;
            } else {
                sprite.blink = false;
            }
            if controller.hp == 0 || (player.crushed && !controller.god_mode) {
                buffer.remove_one::<PlayerSprite>(id);
                buffer.remove_one::<Controller>(id);
                let (px, py) = p_rect.centre_int();
                create_explosion(buffer, px, py);
                resources.messages.add("You have died.".to_owned());
            }
            if is_key_down(KeyCode::Q) && is_key_down(KeyCode::D) && !controller.god_mode {
                controller.god_mode = true;
                resources.messages.add("God mode enabled!".to_owned());
            }
        }
    }

    pub fn hurt(&mut self) {
        if self.hurt_timer == 0 && self.hp > 0 && !self.god_mode {
            self.hp -= 1;
            self.hurt_timer = 24;
        }
    }

    pub fn was_hurt(&self) -> bool {
        self.hurt_timer >= 23
    }

    pub fn can_heal(&self) -> bool {
        self.hp < 3
    }

    pub fn heal(&mut self) {
        self.hp += 1;
    }
}
