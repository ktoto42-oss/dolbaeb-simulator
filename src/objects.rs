use crate::player::Player;
use crate::GameState;
use macroquad::prelude::*;
use crate::{STREET_WIDTH, STREET_HEIGHT};

// Структура клада
pub struct Stash {
    pub x: f32,
    pub y: f32,
    pub is_found: bool,
}

// Структура работы
pub struct Work {
    pub x: f32,
    pub y: f32,
    pub is_complete: bool, 
}

impl Work {
    // Проверка работы (ковра)
    pub fn update(&mut self, player: &mut Player, state: &GameState) {
        if *state == GameState::InApartment && !self.is_complete {
            let distance = ((player.x - self.x).powi(2) + (player.y - self.y).powi(2)).sqrt();
            if distance < 60.0 && is_key_pressed(KeyCode::E) && player.high > 30.0 {
                player.paranoia = (player.paranoia - 30.0).max(0.0);
                player.energy = (player.energy - 20.0).max(0.0);
                self.is_complete = true;
            }
        }
    }
}

impl Stash {
    // Проверка клада
    pub fn update(&mut self, player: &mut Player, state: &GameState) {
        if *state == GameState::OnStreet && !self.is_found {
            let distance = ((player.x - self.x).powi(2) + (player.y - self.y).powi(2)).sqrt();
            if distance < 25.0 {
                self.is_found = true;
                player.high = 100.0;
                player.energy = 100.0;
            }
        }
    }
}

// Обновление КД (раз в 5 секунд)
pub fn update_cooldowns(timer: &mut f32, stash: &mut Stash, work: &mut Work) {
    if *timer > 5.0 {
        if stash.is_found {
            stash.x = rand::gen_range(100.0, STREET_WIDTH - 100.0);
            stash.y = rand::gen_range(100.0, STREET_HEIGHT - 100.0);
            stash.is_found = false;
        }
        work.is_complete = false;
        *timer = 0.0;
    }
}