use crate::player::Player;
use macroquad::prelude::*;
use crate::objects::{Work, Stash};
use crate::assets::Assets;

// Состояние игры (где сейчас находится игрок)
#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    InApartment,
    OnStreet,
}

// Настройки размеров локаций (Границы мира)
pub const APT_WIDTH: f32 = 800.0;
pub const APT_HEIGHT: f32 = 600.0;
pub const STREET_WIDTH: f32 = 2500.0;
pub const STREET_HEIGHT: f32 = 2500.0;

// Переключение локаций (при смене переносит игрока в центр новой локации)
pub fn handle_location_switch(state: &mut GameState, player: &mut Player) {
    if is_key_pressed(KeyCode::Space) {
        *state = match *state {
            GameState::InApartment => {
                player.x = STREET_WIDTH / 2.0;
                player.y = STREET_HEIGHT / 2.0;
                GameState::OnStreet
            }
            GameState::OnStreet => {
                player.x = APT_WIDTH / 2.0;
                player.y = APT_HEIGHT / 2.0;
                GameState::InApartment
            }
        };
    }
}

// Цвет заднего фона
pub fn get_bg_color(player: &Player, state: &GameState) -> Color  {
    
    let bg_color = if player.paranoia > 70.0 {
        Color::new(0.3, 0.1, 0.1, 1.0)
    } else {
        match state {
            GameState::InApartment => DARKGRAY,
            GameState::OnStreet => Color::new(0.1, 0.12, 0.1, 1.0),
        }
    };

    bg_color
}

// Отрисовка мира и границ
pub fn draw_world(state: &GameState, work: &Work, stash: &Stash, assets: &Assets) {
    match state {
        GameState::InApartment => {
            // Стены квартиры (просто рамка)
            draw_rectangle_lines(0.0, 0.0, APT_WIDTH, APT_HEIGHT, 5.0, WHITE);
            // Рисует ковер 
            draw_texture_ex (
                &assets.carpet,
                work.x - 64.0,
                work.y - 48.0,
                WHITE,
                DrawTextureParams {
                        dest_size: Some(vec2(128.0, 96.0)),
                        ..Default::default()
                },
            );
            draw_text("Carpet (Press E)", work.x - 35.0, work.y - 50.0, 16.0, LIGHTGRAY);
        }
        GameState::OnStreet => {
            // Границы улицы
            draw_rectangle_lines(0.0, 0.0, STREET_WIDTH, STREET_HEIGHT, 10.0, RED);
                
            // Кусты для теста
            draw_circle(500.0, 500.0, 30.0, GREEN);
            draw_circle(1500.0, 800.0, 40.0, GREEN);

            // Рисует клад
            if !stash.is_found {
                draw_texture_ex(
                    &assets.stash,
                    stash.x - 16.0,
                    stash.y - 16.0, 
                    WHITE,          
                    DrawTextureParams {
                        dest_size: Some(vec2(32.0, 32.0)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}