use macroquad::prelude::*;
use crate::GameState;
use crate::{STREET_WIDTH, STREET_HEIGHT, APT_HEIGHT, APT_WIDTH};
use crate::assets::Assets;

// Структура игрока
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub rotation: f32,
}

impl Player {
    // Управление
    pub fn handle_input(&mut self, delta_time: f32) { 
        if is_key_down(KeyCode::W) { self.y -= self.speed * delta_time; }
        if is_key_down(KeyCode::S) { self.y += self.speed * delta_time; }
        if is_key_down(KeyCode::A) { self.x -= self.speed * delta_time; }
        if is_key_down(KeyCode::D) { self.x += self.speed * delta_time; }
    }

    // Обновление вращения
    pub fn update_rotation(&mut self, camera: &Camera2D) {
        // Позиция курсора
        let mouse_screen = mouse_position();

        // Позиция в мире
        let mouse_world = camera.screen_to_world(vec2(mouse_screen.0, mouse_screen.1));

        // Вычисление вектора от игрока к курсору
        let direction = mouse_world - vec2(self.x, self.y);

        // Нахождение угла
        self.rotation = direction.y.atan2(direction.x);
    }

    // Ограничение локаций
    pub fn location_restriction(&mut self, state: &GameState) {
        match state {
            GameState::InApartment => {
                self.x = self.x.clamp(0.0, APT_WIDTH);
                self.y = self.y.clamp(0.0, APT_HEIGHT);
            }
            GameState::OnStreet => {
                self.x = self.x.clamp(0.0, STREET_WIDTH);
                self.y = self.y.clamp(0.0, STREET_HEIGHT);
            }
        }
    }

    // Отрисовка игрока
    pub fn draw(&mut self, assets: &Assets) {
        draw_texture_ex(
            &assets.player,
            self.x - 16.0, 
            self.y - 32.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 64.0)),
                rotation: self.rotation,
                pivot: Some(vec2(self.x, self.y)),
                ..Default::default()
            },
        );
    }
}