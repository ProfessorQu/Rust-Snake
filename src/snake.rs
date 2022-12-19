pub mod snake {
    use raylib::prelude::*;
    use raylib::prelude::consts::KeyboardKey::*;

    use crate::{CELL_SIZE, CELL_SIZE_I, GRID_SIZE};

    #[derive(Clone, Copy, PartialEq)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right
    }

    impl Direction {
        fn opposite(&self) -> Self {
            match &self {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            }
        }
    }

    pub struct Snake {
        pub body: Vec<Vector2>,
        pub direction: Direction,
        pub next_direction: Direction,
    }

    impl Snake {
        fn out_of_bounds(&self, x: f32, y: f32) -> bool {
            x < 0.0 || x >= GRID_SIZE as f32 || y < 0.0 || y >= GRID_SIZE as f32
        }

        fn transform(&self, vec: &Vector2, direction: &Direction) -> Vector2 {
            let next = match direction {
                Direction::Up => match self.out_of_bounds(vec.x, vec.y - 1.0) {
                    true => rvec2(vec.x, GRID_SIZE - 1),
                    false => rvec2(vec.x, vec.y - 1.0)
                },
                Direction::Down => match self.out_of_bounds(vec.x, vec.y + 1.0) {
                    true => rvec2(vec.x, 0),
                    false => rvec2(vec.x, vec.y + 1.0)
                },
                Direction::Left => match self.out_of_bounds(vec.x - 1.0, vec.y) {
                    true => rvec2(GRID_SIZE - 1, vec.y),
                    false => rvec2(vec.x - 1.0, vec.y)
                },
                Direction::Right => match self.out_of_bounds(vec.x + 1.0, vec.y) {
                    true => rvec2(0, vec.y),
                    false => rvec2(vec.x + 1.0, vec.y)
                }
            };

            next
        }

        pub fn get_inputs(&mut self, handle: &RaylibHandle) {
            if      handle.is_key_down(KEY_UP)    && self.direction.opposite() != Direction::Up {
                self.next_direction = Direction::Up;
            }
            else if handle.is_key_down(KEY_DOWN)  && self.direction.opposite() != Direction::Down {
                self.next_direction = Direction::Down;
            }
            else if handle.is_key_down(KEY_LEFT)  && self.direction.opposite() != Direction::Left {
                self.next_direction = Direction::Left;
            }
            else if handle.is_key_down(KEY_RIGHT) && self.direction.opposite() != Direction::Right {
                self.next_direction = Direction::Right;
            }
        }

        fn eat_food(&mut self, food: &mut Food) {
            if self.body[0] == food.pos {
                food.respawn(&self);
                let tmp = self.body[self.body.len() - 1].clone();
                self.body.push(tmp);
            }
        }

        pub fn update(&mut self, food: &mut Food) {
            self.direction = self.next_direction.clone();
            let head = self.body.first().clone().expect("Failed to get the head");
            self.body.insert(0, self.transform(&head, &self.direction));
            self.body.remove(self.body.len() - 1);

            self.eat_food(food);
        }

        pub fn draw(&self, draw: &mut RaylibDrawHandle) {
            let mut color = Color::LIME;
            for part in self.body.iter() {
                let x = (part.x * CELL_SIZE) as i32;
                let y = (part.y * CELL_SIZE) as i32;
                
                draw.draw_rectangle(x, y, CELL_SIZE_I, CELL_SIZE_I, color);

                if color == Color::LIME {
                    color = Color::DARKGREEN;
                } else {
                    color = Color::LIME;
                }
            }
        }
    }

    pub struct Food {
        pub pos: Vector2
    }

    impl Food {
        pub fn new() -> Self {
            Self {
                pos: rvec2(
                    get_random_value::<i32>(0, GRID_SIZE - 1),
                    get_random_value::<i32>(0, GRID_SIZE - 1),
                )
            }
        }

        pub fn respawn(&mut self, snake: &Snake) {
            self.pos = rvec2(
                get_random_value::<i32>(0, GRID_SIZE - 1),
                get_random_value::<i32>(0, GRID_SIZE - 1),
            );

            let mut collision = true;

            while collision {
                collision = false;

                for part in snake.body.iter() {
                    if *part == self.pos {
                        self.pos = rvec2(
                            get_random_value::<i32>(0, GRID_SIZE - 1),
                            get_random_value::<i32>(0, GRID_SIZE - 1),
                        );
                        collision = true;
                        break;
                    }
                }
            }
        }

        pub fn draw(&self, draw: &mut RaylibDrawHandle) {
            let x = (self.pos.x * CELL_SIZE) as i32;
            let y = (self.pos.y * CELL_SIZE) as i32;
            draw.draw_rectangle(
                x, y,
                CELL_SIZE_I, CELL_SIZE_I,
                Color::RED);
        }
    }
}