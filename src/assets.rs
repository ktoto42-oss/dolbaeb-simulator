use macroquad::prelude::*;

// Структура текстур
pub struct Assets {
    pub player: Texture2D,
    pub phone: Texture2D,
    pub font: Font,
}

impl Assets {
    // Загрузка текстур
    pub async fn load() -> Self {
        let player = load_texture("assets/player.png").await.unwrap();
        let phone = load_texture("assets/phone.png").await.unwrap();

        let font = load_ttf_font("assets/times.ttf").await.unwrap();

        // Отключение размытия для пиксель арта
        player.set_filter(FilterMode::Nearest);
        phone.set_filter(FilterMode::Nearest);

        Self {
            player,
            phone,
            font,
        }
    }
}