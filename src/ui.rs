use macroquad::prelude::*;
use crate::GameState;
use crate::player::Player;

// Отрисовка интерфейса
pub fn draw_ui(state: &GameState, player: &Player) {
    // Фиксация камеры (всё что нихе будет статично)
    set_default_camera();

    // Подсказки
    match state {
        GameState::InApartment => { draw_text("Home. Press SPACE to go outside", 20.0, 30.0, 24.0, WHITE); },
        GameState::OnStreet => { draw_text("Street. Find the blue zip! Press SPACE to go home", 20.0, 30.0, 24.0, WHITE); },
    }

    // Кайф (Красная полоса)
    draw_text(&format!("High: {:.0}%", player.high), 20.0, screen_height() - 80.0, 20.0, MAROON);
    draw_rectangle(120.0, screen_height() - 95.0, player.high * 1.5, 15.0, MAROON);

    // Паранойя (Фиолетовая полоса)
    draw_text(&format!("Paranoia: {:.0}%", player.paranoia), 20.0, screen_height() - 50.0, 20.0, PURPLE);
    draw_rectangle(120.0, screen_height() - 65.0, player.paranoia * 1.5, 15.0, PURPLE);

    // Энергия (Желтая полоса)
    draw_text(&format!("Energy: {:.0}%", player.energy), 20.0, screen_height() - 20.0, 20.0, GOLD);
    draw_rectangle(120.0, screen_height() - 35.0, player.energy * 1.5, 15.0, GOLD);
}