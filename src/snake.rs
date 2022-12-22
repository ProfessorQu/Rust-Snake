pub mod snake {
    use std::ops::Sub;

    use raylib::prelude::*;
    use raylib::prelude::consts::KeyboardKey::*;

    use rand::{*, seq::SliceRandom};

    use crate::{CELL_SIZE, CELL_SIZE_I, GRID_WIDTH, GRID_HEIGHT};

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct Pos {
        pub x: usize,
        pub y: usize
    }

    impl Pos {
        pub fn new(x: usize, y: usize) -> Self {
            Self {
                x,
                y
            }
        }

        pub fn neighbor(&self, other: Pos) -> bool {
            let x: i32 = other.x as i32 - self.x as i32;
            let y: i32 = other.y as i32 - self.y as i32;

            match x {
                0 => match y {
                    0 => false,
                    -1 => true,
                    1 => true,
                    _ => false
                }
                -1 => y == 0,
                1 => y == 0,
                _ => false
            }
        }
    }

    impl Sub for Pos {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
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
        pub body: Vec<Pos>,
        direction: Direction,
        next_direction: Direction,
        game_over: bool,
        game_win: bool,
    }

    impl Snake {
        pub fn new(len: i32) -> Self {
            let mut body = Vec::new();
            for i in (0..len as usize).rev() {
                body.push(Pos::new(i, 0));
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

        fn out_of_bounds(&self, x: usize, y: usize) -> bool {
            x >= GRID_WIDTH || y >= GRID_HEIGHT
        }

        fn transform(&self, vec: &Pos, direction: &Direction) -> Pos {
            let x = vec.x as i32;
            let y = vec.y as i32;
            match direction {
                Direction::Up => Pos::new(x as usize, (y - 1) as usize),
                Direction::Down => Pos::new(x as usize, (y + 1) as usize),
                Direction::Left => Pos::new((x - 1) as usize, y as usize),
                Direction::Right => Pos::new((x + 1) as usize, y as usize)
            }
        }

        fn test_neighbors(&mut self, check_x: bool) -> Pos {
            let head = self.body[0];
            let offsets = vec![-1 as i32, 1];

            // ====================
            // Test offsets
            // ====================
            for offset in offsets.iter() {
                let x_mult = check_x as i32;
                let y_mult = 1 - x_mult;

                let new_x = (head.x as i32 + (*offset * x_mult)) as usize;
                let new_y = (head.y as i32 + (*offset * y_mult)) as usize;

                if !self.out_of_bounds(new_x, new_y) {
                    let cur_pos = Pos::new(new_x, new_y);

                    if !self.body[1..].contains(&cur_pos)
                    {
                        return cur_pos
                    }
                }
            }

            return Pos::new(GRID_WIDTH, GRID_HEIGHT)
        }

        fn test_pos(&mut self) -> Option<Pos> {
            let rand_bool = rand::random::<f32>() < 0.5;
            match self.test_neighbors(rand_bool) {
                Pos {x: GRID_WIDTH, y: GRID_HEIGHT } =>
                match self.test_neighbors(!rand_bool) {
                    Pos {x: GRID_WIDTH, y: GRID_HEIGHT } => None,
                    x => Some(x)
                },
                x => Some(x)
            } 
        }

        pub fn get_dir_of_free_space(&mut self) -> Direction {
            let head = self.body[0];
            
            match self.test_pos() {
                Some(next) => {
                    let x: i32 = next.x as i32 - head.x as i32;
                    let y: i32 = next.y as i32 - head.y as i32;
    
                    match x {
                        1 => Direction::Right,
                        -1 => Direction::Left,
                        0 => match y {
                            1 => Direction::Down,
                            -1 => Direction::Up,
                            _ => Direction::Right
                        },
                        _ => Direction::Right
                    
                    }
                }
                None => Direction::Right
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

        pub fn set_next_direction(&mut self, dir: Direction) {
            if self.direction.opposite() != dir {
                self.next_direction = dir;
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
        pub pos: Pos
    }

    impl Food {
        pub fn new() -> Self {
            Self {
                pos: Pos::new(
                    get_random_value::<i32>(0, GRID_WIDTH as i32 - 1) as usize,
                    get_random_value::<i32>(0, GRID_HEIGHT as i32- 1) as usize,
                )
            }
        }

        fn get_free_spaces(&self, snake: &Snake) -> Vec<Pos> {
            let mut spaces = Vec::new();

            for x in 0..GRID_WIDTH {
                for y in 0..GRID_HEIGHT {
                    let vector = Pos::new(x, y);
                    if !snake.body.contains(&vector) {
                        spaces.push(vector);
                    }
                }
            }

            spaces
        }

        pub fn respawn(&mut self, snake: &Snake) -> bool {
            self.pos = Pos::new(
                get_random_value::<i32>(0, GRID_WIDTH as i32 - 1) as usize,
                get_random_value::<i32>(0, GRID_HEIGHT as i32 - 1) as usize,
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