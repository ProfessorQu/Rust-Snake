pub mod snake {
    use std::ops::Sub;

    use raylib::prelude::*;
    use raylib::prelude::consts::KeyboardKey::*;

    use rand::{*, seq::SliceRandom};

    use crate::{CELL_SIZE, CELL_SIZE_I, GRID_WIDTH, GRID_HEIGHT, START_LEN};

    pub fn in_bounds(x: usize, y: usize) -> bool {
        x < GRID_WIDTH && y < GRID_HEIGHT
    }

    pub fn in_bounds_i32(x: i32, y: i32) -> bool {
        x >= 0 && x < GRID_WIDTH as i32 && y >= 0 && y < GRID_HEIGHT as i32
    }

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

        pub fn get_neighbors(&self) -> Vec<Self> {
            let mut vec = Vec::new();
            if in_bounds_i32(self.x as i32 + 1, self.y as i32) {
                vec.push(Self { x: self.x + 1, y: self.y })
            }

            if in_bounds_i32(self.x as i32 - 1, self.y as i32) {
                vec.push(Self { x: self.x - 1, y: self.y })
            }

            if in_bounds_i32(self.x as i32, self.y as i32 + 1) {
                vec.push(Self { x: self.x, y: self.y + 1 })
            }

            if in_bounds_i32(self.x as i32, self.y as i32 - 1) {
                vec.push(Self { x: self.x, y: self.y - 1 })
            }

            vec
        }

        pub fn adjacent(&self, dir: &Direction) -> Option<Self> {
            match dir {
                Direction::Right => if in_bounds_i32(self.x as i32 + 1, self.y as i32) {
                    Some(Self { x: self.x + 1, y: self.y})
                } else { None },

                Direction::Left => if in_bounds_i32(self.x as i32 - 1, self.y as i32) {
                    Some(Self { x: self.x - 1, y: self.y })
                } else { None },
                
                Direction::Up => if in_bounds_i32(self.x as i32, self.y as i32 - 1) {
                    Some(Self { x: self.x, y: self.y - 1 })
                } else { None },

                Direction::Down => if in_bounds_i32(self.x as i32, self.y as i32 + 1) {
                    Some(Self { x: self.x, y: self.y + 1 })
                } else { None },
            }
        }

        pub fn get_dir_to(&self, other: &Pos) -> Option<Direction> {
            let x: i32 = other.x as i32 - self.x as i32;
            let y: i32 = other.y as i32 - self.y as i32;

            match x {
                0 => match y {
                    0 => None,
                    1 => Some(Direction::Down),
                    -1 => Some(Direction::Up),
                    _ => None
                }
                1 => if y == 0 { Some(Direction::Right) } else { None },
                -1 => if y == 0 { Some(Direction::Left) } else { None },
                _ => None
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
        pub fn opposite(&self) -> Self {
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
        pub game_over: bool,
        pub game_win: bool,
    }

    impl Snake {
        pub fn new() -> Self {
            let mut body = Vec::new();
            for i in (0..START_LEN).rev() {
                body.push(Pos::new(i % GRID_WIDTH, i / GRID_WIDTH));
            }

            Self {
                body: body,
                direction: Direction::Right,
                next_direction: Direction::Right,
                game_over: false,
                game_win: false,
            }
        }

        pub fn reset(&mut self) {
            let mut body = Vec::new();
            for i in (0..START_LEN).rev() {
                body.push(Pos::new(i % GRID_WIDTH, i / GRID_WIDTH));
            }

            self.body = body;
            self.direction = Direction::Right;
            self.next_direction = Direction::Right;
            self.game_over = false;
            self.game_win = false;
        }

        pub fn len(&self) -> usize {
            self.body.len()
        }

        pub fn head(&self) -> Pos {
            self.body[0]
        }

        pub fn exclude_head(&self) -> Vec<Pos> {
            self.body[1..].to_vec()
        }

        pub fn game_ended(&self) -> bool {
            self.game_over || self.game_win
        }

        fn transform(&self, vec: &Pos, direction: &Direction) -> Option<Pos> {
            let x = vec.x as i32;
            let y = vec.y as i32;

            match direction {
                Direction::Up => match in_bounds_i32(x, y - 1) {
                    true => Some(Pos::new(vec.x, (y - 1) as usize)),
                    false => None,
                },
                Direction::Down => match in_bounds_i32(x, y + 1) {
                    true => Some(Pos::new(vec.x, (y + 1) as usize)),
                    false => None,
                },

                Direction::Left => match in_bounds_i32(x - 1, y) {
                    true => Some(Pos::new((x - 1) as usize, vec.y)),
                    false => None,
                },
                Direction::Right => match in_bounds_i32(x + 1, y) {
                    true => Some(Pos::new((x + 1) as usize, vec.y)),
                    false => None,
                },
            }
        }

        fn get_free_spaces(&self, positions: Vec<Pos>) -> Vec<Pos> {
            let mut free = Vec::new();

            for pos in positions.iter() {
                if !self.exclude_head().contains(pos) {
                    free.push(*pos);
                }
            }

            free
        }

        pub fn get_random_free_dir(&mut self) -> Direction {
            let head = self.head();

            let neighbors = head.get_neighbors();
            let free = self.get_free_spaces(neighbors);

            if free.len() > 0 {
                let mut rng = thread_rng();
                let next = free.choose(&mut rng).expect("Failed to get a random position from free");
                
                head.get_dir_to(&next).expect("Failed to get the direction to the next position")
            }
            else {
                Direction::Right
            }
        }

        pub fn get_dir_of_free_space(&mut self) -> Direction {
            let head = self.head();

            let neighbors = head.get_neighbors();
            let free = self.get_free_spaces(neighbors);

            if free.len() == 1 {
                return head.get_dir_to(&free[0]).expect("Failed to get the direction to the next position");
            }

            let mut max_space = 0;
            let mut max_pos = Pos::new(GRID_WIDTH, GRID_HEIGHT);

            for free_pos in free.iter() {
                let space = self.get_space(free_pos);

                if space > max_space {
                    max_pos = *free_pos;
                    max_space = space;
                }
            }

            if max_pos.x == GRID_WIDTH || max_pos.y == GRID_HEIGHT {
                Direction::Right
            }
            else {
                head.get_dir_to(&max_pos).expect("Failed to get the direction to the pos with the largest open")
            }
        }

        pub fn get_space(&self, pos: &Pos) -> usize {
            let mut open = Vec::new();
            open.push(*pos);

            let mut idx = 0;
            
            while idx < open.len() {
                let neighbors = open[idx].get_neighbors();
                let free = self.get_free_spaces(neighbors);
    
                for free_pos in free.iter() {
                    if !open.contains(free_pos) {
                        open.push(*free_pos);
                    }
                }

                idx += 1;
            }

            open.len()
        }

        pub fn would_collide(&self) -> bool {
            match self.head().adjacent(&self.direction){
                Some(next) => self.exclude_head().contains(&next),
                None => true
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
            if self.head() == food.pos {
                self.game_win = food.respawn(&self);
                let tmp = self.body[self.body.len() - 1].clone();
                self.body.push(tmp);

                return true
            }

            false
        }

        fn collide(&mut self) {
            let head = self.head();
            if !self.game_over {
                let pos = match self.transform(&head, &self.next_direction) {
                    Some(x) => x,
                    None => { self.game_over = true; return; }
                };

                match self.exclude_head().contains(&pos) {
                    true => self.game_over = true,
                    false => self.game_over = !in_bounds(head.x, head.y)
                }
            }
        }

        pub fn update(&mut self, food: &mut Food, score: &mut i32) {
            self.collide();

            if self.game_over {
                return;
            }

            self.direction = self.next_direction.clone();
            let head = self.body.first().clone().expect("Failed to get the snake head");

            let next = self.transform(&head, &self.direction).expect("Failed to transform head to the next direction");
            self.body.insert(0, next);
            self.body.remove(self.body.len() - 1);

            *score = match self.eat_food(food) {
                true => *score + 1,
                false => *score,
            };
        }

        pub fn draw(&self, draw: &mut RaylibDrawHandle) {
            let mut color = Color::SKYBLUE;
            for part in self.body.iter() {
                let x = (part.x * CELL_SIZE) as i32;
                let y = (part.y * CELL_SIZE) as i32;
                
                draw.draw_rectangle(x, y, CELL_SIZE_I, CELL_SIZE_I, color);

                if color == Color::DARKGREEN {
                    color = Color::LIME;
                } else {
                    color = Color::DARKGREEN;
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