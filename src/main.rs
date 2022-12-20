use raylib::prelude::*;

mod snake;
use snake::snake::*;

const SCREEN_SIZE: i32 = 800;

const CELL_SIZE: f32 = 50.0;
const CELL_SIZE_I: i32 = CELL_SIZE as i32;

const GRID_SIZE: i32 = SCREEN_SIZE / CELL_SIZE_I;

const GAME_SPEED: i32 = 10;
const FPS: u32 = 60;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_SIZE, SCREEN_SIZE)
        .title("Hello, World")
        .build();

    let mut frame_count = 0;
    rl.set_target_fps(FPS);
    
    let mut snake = Snake::new(3);
    let mut food = Food::new();

    let mut score = 0;
    let font_size = 40;

    while !rl.window_should_close() && !snake.game_ended() {
        snake.get_inputs(&rl);

        if frame_count % GAME_SPEED == 0 {
            snake.update(&mut food, &mut score);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::LIGHTGRAY);

        snake.draw(&mut d);
        food.draw(&mut d);

        let score_text = &format!("Score: {}", score);
        let text_length = measure_text(score_text, font_size);

        d.draw_text(score_text, SCREEN_SIZE / 2 - text_length / 2, 10, font_size, Color::YELLOW);

        d.draw_fps(10, 10);

        frame_count += 1;
        frame_count %= 60;
    }
}