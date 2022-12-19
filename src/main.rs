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
    
    let mut player = Snake {
        body: vec![
            rvec2(0, 0),
            rvec2(1, 0),
            rvec2(2, 0),
        ],
        direction: Direction::Down,
        next_direction: Direction::Down,
    };

    let mut food = Food::new();

    while !rl.window_should_close() {
        player.get_inputs(&rl);

        if frame_count % GAME_SPEED == 0 {
            player.update(&mut food);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::LIGHTGRAY);

        player.draw(&mut d);
        food.draw(&mut d);

        d.draw_fps(10, 10);

        frame_count += 1;
        frame_count %= 60;
    }
}