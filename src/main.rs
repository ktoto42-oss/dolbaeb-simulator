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
        high: 50.0,
        paranoia: 10.0,
        energy: 80.0,
        rotation: 0.0,
    };

    // Спавн первого клада на улице
    let mut stash = objects::Stash {
        x: rand::gen_range(100.0, STREET_WIDTH - 100.0),
        y: rand::gen_range(100.0, STREET_HEIGHT - 100.0),
        is_found: false,
    };

    // Спавн работы (ковра) в центре квартиры
    let mut work = objects::Work {
        x: APT_WIDTH / 2.0,
        y: APT_HEIGHT / 2.0,
        is_complete: false,
    };

    // Спавн телефона
    let mut phone = objects::Phone {
        charge: 100.0,
        is_get: false,
    };

    // Текущие состояние (дома или на улице)
    let mut state = GameState::InApartment;

    // Таймер для кд 
    let mut timer: f32 = 0.0;

    // Главный игровой цикл
    loop {
        // Дельта времени (надо для одиннаковой работы при разном фпс)
        let delta_time = get_frame_time();

        // Увеличение таймера (+1 в секунду)
        timer += delta_time;

        // Обновление статов
        player.update_stats(delta_time);

        // Управление
        player.handle_input(delta_time);

        // Вращение
        player.update_rotation(&camera);

        // Ограничение локации
        player.location_restriction(&state);

        // Обновление работы
        work.update(&mut player, &state);

        // Обновление клада
        stash.update(&mut player, &state);

        // Обновление телефона
        phone.update(delta_time);

        // Обновление КД 
        objects::update_cooldowns(&mut timer, &mut stash, &mut work);

        // Смена локации
        world::handle_location_switch(&mut state, &mut player);

        // Камера центрируется на игроке с учетом размеров экрана
        camera.target = vec2(player.x, player.y);

        // Отчистка фона
        clear_background(world::get_bg_color(&player, &state));

        // Включение камеры (всё что ниже будет двигаться вместе с миром)
        set_camera(&camera);

        // Отрисовка мира
        world::draw_world(&state, &work, &stash, &assets);

        // Отрисовка игрока
        player.draw(&assets);

        // Отрисовка UI
        ui::draw_ui(&state, &player);

        phone.draw(&assets);

        next_frame().await
    }
}
