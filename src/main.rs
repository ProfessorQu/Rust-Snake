use raylib::prelude::*;

mod snake;
use snake::snake::*;

mod astar;
use astar::astar::*;

mod ham_cycle;
use ham_cycle::ham_cycle::*;

mod gui;
use gui::gui::*;

const CELL_SIZE: usize = 18;
const CELL_SIZE_I: i32 = CELL_SIZE as i32;

const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;

const SCREEN_WIDTH: i32 = GRID_WIDTH as i32 * CELL_SIZE_I;
const SCREEN_HEIGHT: i32 = GRID_HEIGHT as i32 * CELL_SIZE_I;

const GAME_SPEED: usize = 10;
const SEARCH_EVERY: usize = 10;

const START_LEN: usize = 3;

const FPS: u32 = 60;

const FONT_SIZE: i32 = 40;

const BUTTON_WIDTH: f32 = SCREEN_WIDTH as f32 / 2.5;
const BUTTON_HEIGHT: f32 = SCREEN_HEIGHT as f32 / 6.0;
const BUTTON_FONT_SIZE: i32 = SCREEN_HEIGHT / 15;

fn draw(d: &mut RaylibDrawHandle, snake: &Snake, food: &Food, frame_count: &usize, score: &i32) {
    d.clear_background(Color::LIGHTGRAY);

    food.draw(d);
    snake.draw(d);

    let score_text = &format!("Score: {}", score);
    let score_text_length = measure_text(score_text, FONT_SIZE);

    let frame_text = &format!("Frames: {}", frame_count);
    let frame_text_length = measure_text(frame_text, 20);

    d.draw_text(score_text, SCREEN_WIDTH / 2 - score_text_length / 2, 10, FONT_SIZE, Color::YELLOW);
    d.draw_text(frame_text, SCREEN_WIDTH / 2 - frame_text_length / 2, SCREEN_HEIGHT - 30, 20, Color::BROWN);

    d.draw_fps(10, 10);

    if snake.game_over {
        let text = "GAME OVER";
        let text_length = measure_text(text, FONT_SIZE);
        d.draw_text(text, SCREEN_WIDTH / 2 - text_length / 2, SCREEN_WIDTH / 2 - text_length / 2, FONT_SIZE, Color::RED);
    }
}

fn main() {
    // ==================================
    // Initialize variables
    // ==================================
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Snake")
        .build();

    let mut astar_random = false;

    let mode = mode_menu(&mut rl, &thread);

    if mode == "" {
        return;
    }
    else if mode == "self" {
        rl.set_target_fps(FPS);
    }
    else if mode == "a*" {
        astar_random = match astar_random_menu(&mut rl, &thread) {
            Some(x) => x,
            None => return,
        }
    }

    let mut frame_count = 0;
    
    let mut snake = Snake::new();
    let mut food = Food::new();

    let mut score = 0;

    // ==================================
    // Play self
    // ==================================
    if mode == "self" {
        while !rl.window_should_close() {
            snake.get_inputs(&rl);

            if !snake.game_ended() && frame_count % GAME_SPEED == 0 {
                snake.update(&mut food, &mut score);
            }
    
            if rl.is_key_pressed(consts::KeyboardKey::KEY_R) {
                snake.reset();
                food.respawn(&snake);
    
                frame_count = 0;
                score = 0;
            }
            
            frame_count += 1;
    
            let mut d = rl.begin_drawing(&thread);
            draw(&mut d, &snake, &food, &frame_count, &score);
        }
    }
    // ==================================
    // A* algorithm
    // ==================================
    else if mode == "a*" {
        let mut astar = AStar::new(astar_random);
        astar.search(&snake, &food);

        while !rl.window_should_close() {
            if !snake.game_ended() {
                astar.update(&mut snake, &mut food, &frame_count, &mut score);
                
                frame_count += 1;
            }
    
            if rl.is_key_pressed(consts::KeyboardKey::KEY_R) {
                snake.reset();
                food.respawn(&snake);

                astar.reset();
                astar.search(&snake, &food);
    
                frame_count = 0;
                score = 0;
            }
    
            let mut d = rl.begin_drawing(&thread);
            astar.draw_path(&mut d, &snake);
            draw(&mut d, &snake, &food, &frame_count, &score);
        }
    }
    // ==================================
    // Hamiltonian cycle
    // ==================================
    else if mode == "ham" {
        let mut ham = HamiltonianCycle::new();
        ham.generate(&snake);

        while !rl.window_should_close() {
            if !snake.game_ended() {
                ham.update(&mut snake, &mut food, &mut score);
                
                frame_count += 1;
            }
    
            if rl.is_key_pressed(consts::KeyboardKey::KEY_R) {
                snake.reset();
                food.respawn(&snake);

                ham.reset(&snake);
    
                frame_count = 0;
                score = 0;
            }
    
            let mut d = rl.begin_drawing(&thread);
            draw(&mut d, &snake, &food, &frame_count, &score);
        }
    }
}