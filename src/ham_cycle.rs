
#[allow(dead_code)]
pub mod ham_cycle {
    use raylib::prelude::*;

    use crate::{snake::snake::*, GRID_WIDTH, GRID_HEIGHT, CELL_SIZE, GAME_SPEED};

    struct Node {
        // parent: Box<Node>,
        dist: i32,
        visit: bool,
    }

    impl Node {
        fn new() -> Self {
            Self {
                // parent: Box::<Node>::new(),
                dist: i32::MAX,
                visit: false
            }
        }
    }

    pub struct HamiltonianCycle {
        path: Vec<Pos>,
        path_index: usize,
    }
    
    impl HamiltonianCycle {
        pub fn new() -> Self {
            Self {
                path: Vec::new(),
                path_index: 0
            }
        }

        pub fn update(&mut self, snake: &mut Snake, food: &mut Food, frame_count: &usize, score: &mut i32) {
            if frame_count % GAME_SPEED == 0 {
                snake.set_next_direction(self.get_next_move(&snake.head()));
    
                snake.update(food, score);

                self.path_index += 1;
                self.path_index %= self.path.len() - 1;
            }
        }

        fn get_next_move(&self, head: &Pos) -> Direction {
            head.get_dir_to(&self.path[self.path_index]).unwrap()
        }

        pub fn draw_path(&self, draw: &mut RaylibDrawHandle) {
            for (idx, piece) in self.path.iter().enumerate() {
                let x = (piece.x * CELL_SIZE) as i32;
                let y = (piece.y * CELL_SIZE) as i32;
                draw.draw_text(&idx.to_string(), x, y, 1, Color::BLACK)
            }
        }

        pub fn reset(&mut self, snake: &Snake) {
            self.path_index = snake.len();
        }

        pub fn generate(&mut self, snake: &Snake) {
            self.path.push(Pos::new(0, 0));

            let mut reverse = true;
            for y in 0..GRID_HEIGHT {
                for x in 1..GRID_WIDTH {
                    if reverse {
                        self.path.push(Pos::new(x, y));
                    }
                    else {
                        self.path.push(Pos::new(GRID_WIDTH - x, y));
                    }
                }

                reverse = !reverse;
            }

            for y in (0..GRID_HEIGHT).rev() {
                self.path.push(Pos::new(0, y));
            }

            self.path_index = snake.len();
        }
    }
}