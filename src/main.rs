use raylib::prelude::*;

mod snake;
use snake::snake::*;

mod astar;
use astar::astar::*;

const SCREEN_SIZE: i32 = 800;

const CELL_SIZE: f32 = 50.0;
const CELL_SIZE_I: i32 = CELL_SIZE as i32;

const GRID_SIZE: i32 = SCREEN_SIZE / CELL_SIZE_I;

const GAME_SPEED: i32 = 1;
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
    let mut astar = AStar::new();

    astar.search(&snake, &food);
    let mut path_index: usize = 1;

    let mut score = 0;
    let font_size = 40;

    while !rl.window_should_close() && !snake.game_ended() {
        let score_before = score;

        if frame_count % GAME_SPEED == 0 {
            snake.set_next_direction(astar.get_next_move(path_index, &snake));
            snake.update(&mut food, &mut score);
            path_index += 1;
        }

        if score_before - score != 0 {
            astar.search(&snake, &food);
            path_index = 1;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::LIGHTGRAY);

        astar.draw_path(&mut d);

        food.draw(&mut d);
        snake.draw(&mut d);

        let score_text = &format!("Score: {}", score);
        let text_length = measure_text(score_text, font_size);

        d.draw_text(score_text, SCREEN_SIZE / 2 - text_length / 2, 10, font_size, Color::YELLOW);

        d.draw_fps(10, 10);

        frame_count += 1;
        frame_count %= 60;
    }
}