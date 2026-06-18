use macroquad::prelude::*;

// Состояние игры (где сейчас находится игрок)
#[derive(PartialEq, Clone, Copy)]
enum GameState {
    InApartment,
    OnStreet,
}

// Структура игрока
struct Player {
    x: f32,
    y: f32,
    speed: f32,
    high: f32,
    paranoia: f32,
    energy: f32,
}

// Структура клада
struct Stash {
    x: f32,
    y: f32,
    is_found: bool,
}

// Структура работы
struct Work {
    x: f32,
    y: f32,
    is_complete: bool, 
}

#[macroquad::main("Dolbaeb simulator")]
async fn main() {
    // Настройки размеров локаций (Границы мира)
    let apt_width = 800.0;
    let apt_height = 600.0;
    let street_width = 2500.0;
    let street_height = 2500.0;

    // Инициализирует игрока в центре экрана
    let mut player = Player {
        x: apt_width / 2.0,
        y: apt_height / 2.0,
        speed: 200.0,
        high: 50.0,
        paranoia: 10.0,
        energy: 80.0,
    };

    // Спавн первого клада на улице
    let mut stash = Stash {
        x: rand::gen_range(100.0, street_width - 100.0),
        y: rand::gen_range(100.0, street_height - 100.0),
        is_found: false,
    };

    // Спавн работы (ковра) в центре квартиры
    let mut work = Work {
        x: apt_width / 2.0,
        y: apt_height / 2.0,
        is_complete: false,
    };

    let mut state = GameState::InApartment;
    let mut timer: f32 = 0.0;

    // Камера
    let mut camera = Camera2D::default();
    camera.zoom = vec2(2.0 / screen_width(), 2.0 / screen_height());

    // Текстуры 
    let player_texture: Texture2D = load_texture("assets/player.png").await.unwrap();
    let stash_texture: Texture2D = load_texture("assets/zip.png").await.unwrap();
    let carpet_texture: Texture2D = load_texture("assets/carpet.png").await.unwrap();

    // Отключение размытия т.к пиксель арт
    player_texture.set_filter(FilterMode::Nearest);
    stash_texture.set_filter(FilterMode::Nearest);
    carpet_texture.set_filter(FilterMode::Nearest);

    // Главный игровой цикл
    loop {
        let delta_time = get_frame_time();
        timer += delta_time;

        // Координаты курсора
        let mouse_screen = mouse_position();
        // Позиция в мире
        let mouse_world = camera.screen_to_world(vec2(mouse_screen.0, mouse_screen.1));
        // Вычисляем вектор от игрока к мыши
        let direction = mouse_world - vec2(player.x, player.y);
        // Находим угол в радианах.
        let player_rotation = direction.y.atan2(direction.x);

        // Время идет - кайф и энергия падают
        player.high = (player.high - 2.0 * delta_time).max(0.0);
        player.energy = (player.energy - 1.0 * delta_time).max(0.0);

        if player.high > 70.0 {
            player.paranoia = (player.paranoia + 5.0 * delta_time).min(100.0);
        } else if player.high < 15.0 {
            player.paranoia = (player.paranoia + 3.0 * delta_time).min(100.0);
        } else {
            player.paranoia = (player.paranoia - 1.0 * delta_time).max(0.0);
        }

        // Замедление персонажа при малом количестве энергии
        if player.energy < 20.0 {
            player.speed = 100.0;
        } else {
            player.speed = 220.0;
        }

        // Управление
        if is_key_down(KeyCode::W) { player.y -= player.speed * delta_time; }
        if is_key_down(KeyCode::S) { player.y += player.speed * delta_time; }
        if is_key_down(KeyCode::A) { player.x -= player.speed * delta_time; }
        if is_key_down(KeyCode::D) { player.x += player.speed * delta_time; }

        // Ограничение локаций
        match state {
            GameState::InApartment => {
                player.x = player.x.clamp(0.0, apt_width);
                player.y = player.y.clamp(0.0, apt_height);
            }
            GameState::OnStreet => {
                player.x = player.x.clamp(0.0, street_width);
                player.y = player.y.clamp(0.0, street_height);
            }
        }

        // Переключение локаций (при смене переносит игрока в центр новой локации)
        if is_key_pressed(KeyCode::Space) {
            state = match state {
                GameState::InApartment => {
                    player.x = street_width / 2.0;
                    player.y = street_height / 2.0;
                    GameState::OnStreet
                }
                GameState::OnStreet => {
                    player.x = apt_width / 2.0;
                    player.y = apt_height / 2.0;
                    GameState::InApartment
                }
            };
        }

        // Проверка работы (ковра)
        if state == GameState::InApartment && !work.is_complete {
            let distance = ((player.x - work.x).powi(2) + (player.y - work.y).powi(2)).sqrt();
            if distance < 60.0 && is_key_pressed(KeyCode::E) && player.high > 30.0 {
                player.paranoia = (player.paranoia - 30.0).max(0.0);
                player.energy = (player.energy - 20.0).max(0.0);
                work.is_complete = true;
            }
        }

        // Проверка клада
        if state == GameState::OnStreet && !stash.is_found {
            let distance = ((player.x - stash.x).powi(2) + (player.y - stash.y).powi(2)).sqrt();
            if distance < 25.0 {
                stash.is_found = true;
                player.high = 100.0;
                player.energy = 100.0;
            }
        }

        // КД для активностей (5 секунд)
        if timer > 5.0 {
            if stash.is_found {
                stash.x = rand::gen_range(100.0, street_width - 100.0);
                stash.y = rand::gen_range(100.0, street_height - 100.0);
                stash.is_found = false;
            }
            work.is_complete = false;
            timer = 0.0;
        }

        // Камера центрируется на игроке с учетом размеров экрана
        camera.target = vec2(player.x, player.y);

        // Отрисовка графики
        let bg_color = if player.paranoia > 70.0 {
            Color::new(0.3, 0.1, 0.1, 1.0)
        } else {
            match state {
                GameState::InApartment => DARKGRAY,
                GameState::OnStreet => Color::new(0.1, 0.12, 0.1, 1.0),
            }
        };

        clear_background(bg_color);

        // Включение камеры всё что ниже будет двигаться вместе с миром
        set_camera(&camera);

        // Отрисовка мира и границ
        match state {
            GameState::InApartment => {
                // Стены квартиры (просто рамка)
                draw_rectangle_lines(0.0, 0.0, apt_width, apt_height, 5.0, WHITE);
                
                // Рисует ковер 
                draw_texture_ex (
                    &carpet_texture,
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
                draw_rectangle_lines(0.0, 0.0, street_width, street_height, 10.0, RED);
                
                // Кусты для теста
                draw_circle(500.0, 500.0, 30.0, GREEN);
                draw_circle(1500.0, 800.0, 40.0, GREEN);

                // Рисует клад
                if !stash.is_found {
                    draw_texture_ex(
                        &stash_texture,
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

        // Рисует игрока внутри игрового мира
        draw_texture_ex(
            &player_texture,
            player.x - 16.0, 
            player.y - 32.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 64.0)),
                rotation: player_rotation,
                ..Default::default()
            },
        );

        // Смена камеры всё что ниже будет статично
        set_default_camera();

        // Подсказки
        match state {
            GameState::InApartment => { draw_text("Home. Press SPACE to go outside", 20.0, 30.0, 24.0, WHITE); },
            GameState::OnStreet => { draw_text("Street. Find the blue zip! Press SPACE to go home", 20.0, 30.0, 24.0, WHITE); },
        }


        // Отрисовка интерфейса (полоски шкал)
        // Кайф (Красная полоса)
        draw_text(&format!("High: {:.0}%", player.high), 20.0, screen_height() - 80.0, 20.0, MAROON);
        draw_rectangle(120.0, screen_height() - 95.0, player.high * 1.5, 15.0, MAROON);

        // Паранойя (Фиолетовая полоса)
        draw_text(&format!("Paranoia: {:.0}%", player.paranoia), 20.0, screen_height() - 50.0, 20.0, PURPLE);
        draw_rectangle(120.0, screen_height() - 65.0, player.paranoia * 1.5, 15.0, PURPLE);

        // Энергия (Желтая полоса)
        draw_text(&format!("Energy: {:.0}%", player.energy), 20.0, screen_height() - 20.0, 20.0, GOLD);
        draw_rectangle(120.0, screen_height() - 35.0, player.energy * 1.5, 15.0, GOLD);

        next_frame().await
    }
}