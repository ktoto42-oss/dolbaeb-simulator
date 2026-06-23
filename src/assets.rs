use macroquad::prelude::*;

// Структура текстур
pub struct Assets {
    pub player: Texture2D,
    pub phone: Texture2D,
}

impl Assets {
    // Загрузка текстур
    pub async fn load() -> Self {
        let player = load_texture("assets/player.png").await.unwrap();
        let phone = load_texture("assets/phone.png").await.unwrap();

        // Отключение размытия для пиксель арта
        player.set_filter(FilterMode::Nearest);
        phone.set_filter(FilterMode::Nearest);

        Self {
            player,
            phone,
        }
    }
}