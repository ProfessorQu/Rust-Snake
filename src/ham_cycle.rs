
pub mod ham_cycle {
    use crate::{snake::snake::*, GRID_WIDTH, GRID_HEIGHT};

    pub struct HamiltonianCycle {
        path: Vec<Direction>,
        path_index: usize
    }
    
    impl HamiltonianCycle {
        pub fn new() -> Self {
            Self {
                path: Vec::new(),
                path_index: 0
            }
        }

        pub fn update(&mut self, snake: &mut Snake, food: &mut Food, score: &mut i32) {
            snake.set_next_direction(self.get_next_move());

            snake.update(food, score);

            self.path_index += 1;
            self.path_index %= self.path.len() - 1;
        }

        fn get_next_move(&self) -> Direction {
            self.path[self.path_index]
        }

        pub fn reset(&mut self, snake: &Snake) {
            self.path_index = snake.len() - 1;
        }

        pub fn generate(&mut self, snake: &Snake) {
            // Go all the way to the right
            for _ in 1..GRID_WIDTH {
                self.path.push(Direction::Right);
            }
            // Go down one
            self.path.push(Direction::Down);

            // Loop back and forth while leaving one gap
            let mut reverse = true;
            for _y in 1..GRID_HEIGHT {
                for _x in 2..GRID_WIDTH {
                    if reverse {
                        self.path.push(Direction::Left);
                    }
                    else {
                        self.path.push(Direction::Right);
                    }
                }

                self.path.push(Direction::Down);
                reverse = !reverse;
            }

            // Go to the edge
            self.path.pop();
            self.path.push(Direction::Left);

            // Go back up
            for _y in 0..GRID_HEIGHT {
                self.path.push(Direction::Up);
            }

            self.path_index = snake.len() - 1;
        }
    }
}