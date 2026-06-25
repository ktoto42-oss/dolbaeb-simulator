use macroquad::prelude::*;
use crate::assets::Assets;

// Структура телефона
pub struct Phone {
    pub charge: f32,
    pub is_get: bool,
}

impl Phone {
    // Обновление 
    pub fn update(&mut self, delta_time: f32) {
        self.charge = (self.charge - 0.2 * delta_time).max(0.0);

        if is_key_pressed(KeyCode::Q) {
            self.is_get = match self.is_get {
                true => false,
                false => true,
            };
        }
    }

    // Отрисовка
    pub fn draw(&self, assets: &Assets, font_idx: usize) {
        let current_font = assets.get_font(font_idx);
        if self.is_get {
            draw_texture_ex (
                &assets.phone,
                60.0,
                screen_height() - 363.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(128.0, 256.0)),
                    ..Default::default()
                },
            );
            draw_text_ex(
                &format!("{:.0}%", self.charge),
                155.0,
                screen_height() - 345.0,
                TextParams {
                    font: Some(current_font),
                    font_size: 16,
                    color: WHITE,
                    ..Default::default()
                }
            );
        }
    }
}
