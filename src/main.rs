use macroquad::prelude::*;
use crate::world::{APT_HEIGHT, APT_WIDTH, STREET_HEIGHT, STREET_WIDTH, GameState};

// Игрок
mod player;

// Мир
mod world;

// Интерфейс 
mod ui;

// Текстуры
mod assets;

// ОбЪекты
mod objects;

// NPC
mod npc;

#[macroquad::main("Dolbaeb simulator")]
async fn main() {
    // Загрузка текстур
    let assets = assets::Assets::load().await;

    // Камера
    let mut camera = Camera2D::default();
    camera.zoom = vec2(2.0 / screen_width(), 2.0 / screen_height());
    
    // Инициализация игрока
    let mut player = player::Player {
        x: APT_WIDTH / 2.0,
        y: APT_HEIGHT / 2.0,
        speed: 200.0,
        rotation: 0.0,
    };

    // Спавн телефона
    let mut phone = objects::Phone {
        charge: 100.0,
        is_get: false,
    };

    // Спавн врага
    let mut enemy = npc::Enemy {
        x: 200.0,
        y: 400.0,
        speed_patrol: 80.0,
        speed_chase: 160.0,
        rotation: 0.0,
        state: npc::EnemyState::Patrol,
        patrol_min_x: 0.0,
        patrol_max_x: 400.0,
        direction: 1.0,
        vision_radius: 250.0,
    };

    // Текущие состояние (дома или на улице)
    let mut state = GameState::InApartment;

    // Главный игровой цикл
    loop {
        // Дельта времени (надо для одиннаковой работы при разном фпс)
        let delta_time = get_frame_time();

        // Управление
        player.handle_input(delta_time);

        // Вращение
        player.update_rotation(&camera);

        // Ограничение локации
        player.location_restriction(&state);

        // Обновление телефона
        phone.update(delta_time);

        // Обновление врага
        enemy.update(&player, delta_time);

        // Смена локации
        world::handle_location_switch(&mut state, &mut player);

        // Камера центрируется на игроке с учетом размеров экрана
        camera.target = vec2(player.x, player.y);

        // Отчистка фона
        clear_background(world::get_bg_color(&state));

        // Включение камеры (всё что ниже будет двигаться вместе с миром)
        set_camera(&camera);

        // Отрисовка мира
        world::draw_world(&state);

        // Отрисовка врагов
        enemy.draw(&state);

        // Отрисовка игрока
        player.draw(&assets);

        // Отрисовка UI
        ui::draw_ui();

        phone.draw(&assets);

        next_frame().await
    }
}
