use rand::Rng;
use raylib::prelude::*;
use std::time::SystemTime;

struct Square {
    x: i32,
    y: i32,
    size: i32,
    color: Color,
}

fn main() {
    // INIT

    let width : i32 = 800;
    let height: i32 = 600;
    let square_size: i32 = 50;

    let mut game_over:bool = false;

    let mut let_enemy_fall:bool = false;

    let mut score:i32 = 0;

    //let mut enemy_speed_multiplier:i32 = 1;

    let mut enemy:Square = Square {
        x: rand::thread_rng().gen_range(0..(width-square_size)),
        y: 0,
        size: square_size,
        color: Color::BLUE,
    };

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Bad Game")
        .build();

    let mut player: Square = Square {
        x: width / 2 - square_size / 2,
        y: height - square_size,
        size: square_size,
        color: Color::RED,
    };

    rl.set_target_fps(60);

    let start_time: SystemTime = SystemTime::now();

    // UPDATE
    while !rl.window_should_close() {
        let enemy_speed_multiplier = score / 10 + 1;
        if !game_over{
            let mut screen: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
            screen.clear_background(Color::WHITE);

            screen.draw_text(&format!("Score: {}", score), 10, 10, 20, Color::BLACK);

            if screen.is_key_down(KeyboardKey::KEY_RIGHT) && player.x < width - player.size {
                player.x += 15;
            }
            if screen.is_key_down(KeyboardKey::KEY_LEFT) && player.x > 0{
                player.x -= 15;
            }
            
            screen.draw_rectangle(player.x, player.y, player.size, player.size, player.color);

            if start_time.elapsed().unwrap().as_secs() % 4 == 2 {
                let_enemy_fall = true;
            }

            if let_enemy_fall {
                enemy.y += 10 * enemy_speed_multiplier;
                screen.draw_rectangle(enemy.x, enemy.y, enemy.size, enemy.size, enemy.color);

                if enemy.y > height {
                    enemy.y = 0;
                    enemy.x = rand::thread_rng().gen_range(0..(width-square_size));
                    score += 1;
                    let_enemy_fall = false;
                }
            }
            // Check for collision
            if enemy.y > player.y - enemy.size && enemy.x > player.x - enemy.size && enemy.x < player.x + player.size {
                game_over = true;
            }
        }
        else{
            let mut screen: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
            screen.clear_background(Color::WHITE);
            screen.draw_text("Game Over", 10, 10, 20, Color::BLACK);
            screen.draw_text(&format!("Score: {}", score), 10, 30, 20, Color::BLACK);
        }
    }
}