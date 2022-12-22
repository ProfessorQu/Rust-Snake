use raylib::prelude::*;

mod snake;
use snake::snake::*;

mod astar;
use astar::astar::*;

mod ham_cycle;
use ham_cycle::ham_cycle::*;


const CELL_SIZE: usize = 10;
const CELL_SIZE_I: i32 = CELL_SIZE as i32;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 80;

const SCREEN_WIDTH: i32 = GRID_WIDTH as i32 * CELL_SIZE_I;
const SCREEN_HEIGHT: i32 = GRID_HEIGHT as i32 * CELL_SIZE_I;

const GAME_SPEED: usize = 1;
const SEARCH_EVERY: usize = 10;
const FPS: u32 = 240;

const FONT_SIZE: i32 = 40;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    let mut frame_count = 0;
    // rl.set_target_fps(FPS);
    
    let mut snake = Snake::new(3);
    let mut food = Food::new();

    let mut astar = AStar::new();

    astar.search(&snake, &food);

    let mut score = 0;

    while !rl.window_should_close() {
        if !snake.game_ended() {
            astar.update(&mut snake, &mut food, &frame_count, &mut score);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::LIGHTGRAY);

        astar.draw_path(&mut d);

        food.draw(&mut d);
        snake.draw(&mut d);

        let score_text = &format!("Score: {}", score);
        let text_length = measure_text(score_text, FONT_SIZE);

        d.draw_text(score_text, SCREEN_WIDTH / 2 - text_length / 2, 10, FONT_SIZE, Color::YELLOW);

        d.draw_fps(10, 10);

        if snake.game_over {
            let text = "GAME OVER";
            let text_length = measure_text(text, FONT_SIZE);
            d.draw_text(text, SCREEN_WIDTH / 2 - text_length / 2, SCREEN_WIDTH / 2 - text_length / 2, FONT_SIZE, Color::RED);
        }

        frame_count += 1;
    }
}