use raylib::prelude::*;

mod snake;
use snake::snake::*;

mod astar;
use astar::astar::*;


const CELL_SIZE: usize = 15;
const CELL_SIZE_I: i32 = CELL_SIZE as i32;

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;

const SCREEN_WIDTH: i32 = GRID_WIDTH as i32 * CELL_SIZE_I;
const SCREEN_HEIGHT: i32 = GRID_HEIGHT as i32 * CELL_SIZE_I;

const GAME_SPEED: usize = 1;
const FPS: u32 = 120;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Hello, World")
        .build();

    let mut frame_count = 0;
    rl.set_target_fps(FPS);
    
    let mut snake = Snake::new(3);
    let mut food = Food::new();
    let mut astar = AStar::new();

    astar.search(&snake, &food);
    let mut path_index: usize = 0;

    let mut score = 0;
    let font_size = 40;

    while !rl.window_should_close() && !snake.game_ended() {
        let score_before = score;

        if frame_count % GAME_SPEED == 0 {
            if !astar.path_found() {
                let dir = snake.get_dir_of_free_space();
                snake.set_next_direction(dir);
            }
            else {
                snake.set_next_direction(astar.get_next_move(path_index, &snake));
            }
            snake.update(&mut food, &mut score);
            path_index += 1;
        }

        if (score_before - score != 0) || !astar.path_found() {
            astar.search(&snake, &food);
            path_index = 0;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::LIGHTGRAY);

        astar.draw_path(&mut d);

        food.draw(&mut d);
        snake.draw(&mut d);

        let score_text = &format!("Score: {}", score);
        let text_length = measure_text(score_text, font_size);

        d.draw_text(score_text, SCREEN_WIDTH / 2 - text_length / 2, 10, font_size, Color::YELLOW);

        d.draw_fps(10, 10);

        frame_count += 1;
        frame_count %= 60;
    }
}