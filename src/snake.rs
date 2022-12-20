pub mod snake {
    use raylib::prelude::*;
    use raylib::prelude::consts::KeyboardKey::*;

    use rand::{*, seq::SliceRandom};

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
        body: Vec<Vector2>,
        direction: Direction,
        next_direction: Direction,
        game_over: bool,
        game_win: bool,
    }

    impl Snake {
        pub fn new(len: i32) -> Self {
            let mut body = Vec::new();
            for i in (0..len).rev() {
                body.push(rvec2(i, 0));
            }

            Self {
                body: body,
                direction: Direction::Right,
                next_direction: Direction::Right,
                game_over: false,
                game_win: false,
            }
        }

        pub fn game_ended(&self) -> bool {
            self.game_over || self.game_win
        }

        fn out_of_bounds(&self, x: f32, y: f32) -> bool {
            x < 0.0 || x >= GRID_SIZE as f32 || y < 0.0 || y >= GRID_SIZE as f32
        }

        fn transform(&self, vec: &Vector2, direction: &Direction) -> Vector2 {
            match direction {
                Direction::Up => rvec2(vec.x, vec.y - 1.0),
                Direction::Down => rvec2(vec.x, vec.y + 1.0),
                Direction::Left => rvec2(vec.x - 1.0, vec.y),
                Direction::Right => rvec2(vec.x + 1.0, vec.y)
            }
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

        fn eat_food(&mut self, food: &mut Food) -> bool{
            if self.body[0] == food.pos {
                self.game_win = food.respawn(&self);
                let tmp = self.body[self.body.len() - 1].clone();
                self.body.push(tmp);

                return true
            }

            false
        }

        fn collide(&mut self) {
            let head = &self.body[0];
            match self.game_over {
                true => { },
                false => match self.body[1..].contains(head) {
                    true => self.game_over = true,
                    false => self.game_over = self.out_of_bounds(head.x, head.y)
                }
            }
        }

        pub fn update(&mut self, food: &mut Food, score: &mut i32) {
            self.direction = self.next_direction.clone();
            let head = self.body.first().clone().expect("Failed to get the head");

            self.body.insert(0, self.transform(&head, &self.direction));
            self.body.remove(self.body.len() - 1);

            *score = match self.eat_food(food) {
                true => *score + 1,
                false => *score,
            };
            self.collide();
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

        fn get_free_spaces(&self, snake: &Snake) -> Vec<Vector2> {
            let mut spaces = Vec::new();

            for x in 0..GRID_SIZE {
                for y in 0..GRID_SIZE {
                    let vector = rvec2(x, y);
                    if !snake.body.contains(&vector) {
                        spaces.push(vector);
                    }
                }
            }

            spaces
        }

        pub fn respawn(&mut self, snake: &Snake) -> bool {
            self.pos = rvec2(
                get_random_value::<i32>(0, GRID_SIZE - 1),
                get_random_value::<i32>(0, GRID_SIZE - 1),
            );

            let mut rng = thread_rng();
            self.pos = match self.get_free_spaces(snake).choose(&mut rng) {
                Some(x) => *x,
                None => return true
            };

            false
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