pub mod ham_cycle {
    use rand::Rng;

    use crate::{GRID_SIZE, GRID_WIDTH, GRID_HEIGHT, snake::snake::Direction};

    #[derive(Clone, Copy)]
    struct Node {
        visited: bool,
        can_go_right: bool,
        can_go_down: bool,
    }

    impl Node {
        fn new() -> Self {
            Self {
                visited: false,
                can_go_right: false,
                can_go_down: false
            }
        }
    }

    struct Maze {
        tour_to_number: [i32; GRID_SIZE],
        nodes: [Node; GRID_SIZE],
    }

    impl Maze {
        pub fn new() -> Self {
            Self {
                tour_to_number: [0; GRID_SIZE],
                nodes: [Node::new(); GRID_SIZE]
            }
        }

        fn get_path_number(&self, x: usize, y: usize) -> i32 {
            self.tour_to_number[x + y * GRID_WIDTH]
        }

        fn mark_visited(&mut self, x: i32, y: i32) {
            self.nodes[x as usize + y as usize * GRID_WIDTH / 2].visited = true;
        }

        fn mark_can_go_right(&mut self, x: i32, y: i32) {
            self.nodes[x as usize + y as usize * GRID_WIDTH / 2].can_go_right = true;
        }

        fn mark_can_go_down(&mut self, x: i32, y: i32) {
            self.nodes[x as usize + y as usize * GRID_WIDTH / 2].can_go_down = true;
        }

        fn can_go_right(&self, x: i32, y: i32) -> bool {
            self.nodes[x as usize + y as usize * GRID_WIDTH / 2].can_go_right
        }

        fn can_go_down(&self, x: i32, y: i32) -> bool {
            self.nodes[x as usize + y as usize * GRID_WIDTH / 2].can_go_down
        }

        fn can_go_left(&self, x: i32, y: i32) -> bool {
            if x == 0 { return false }
            self.nodes[x as usize + y as usize * GRID_WIDTH / 2].can_go_right
        }

        fn can_go_up(&self, x: i32, y: i32) -> bool {
            if y == 0 { return false }
            self.nodes[x as usize + y as usize * GRID_WIDTH / 2].can_go_down
        }

        fn is_visited(&self, x: i32, y: i32) -> bool {
            self.nodes[x as usize + y as usize * GRID_WIDTH / 2].visited
        }

        fn generate_r(&mut self, fromx: i32, fromy: i32, x: i32, y: i32) {
            if x < 0 || y < 0 || x >= GRID_WIDTH as i32 / 2 || y >= GRID_HEIGHT as i32 / 2 {
                return;
            }
            if self.is_visited(x, y) {
                return;
            }

            self.mark_visited(x, y);

            if fromx != -1 {
                if fromx < x {
                    self.mark_can_go_right(fromx, fromy);
                }
                else if fromx > x {
                    self.mark_can_go_right(x, y);
                }
                else if fromy < y {
                    self.mark_can_go_down(fromx, fromy);
                }
                else if fromy > y {
                    self.mark_can_go_down(x, y);
                }
            }

            for _ in 0..2 {
                match rand::thread_rng().gen_range(0..4) {
                    0 => self.generate_r(x, y, x-1, y),
                    1 => self.generate_r(x, y, x+1, y),
                    2 => self.generate_r(x, y, x, y-1),
                    3 => self.generate_r(x, y, x, y+1),
                    _ => {}
                }
            }
            self.generate_r(x, y, x-1, y);
            self.generate_r(x, y, x+1, y);
            self.generate_r(x, y, x, y-1);
            self.generate_r(x, y, x, y+1);
        }

        fn find_next_dir(&self, x: i32, y: i32, snake_dir: Direction) -> Direction {
            match snake_dir {
                Direction::Right => {
                    if self.can_go_up(x, y)     { return Direction::Up    }
                    if self.can_go_right(x, y)  { return Direction::Right }
                    if self.can_go_down(x, y)   { return Direction::Down  }
                    return Direction::Left
                },
                Direction::Down => {
                    if self.can_go_right(x, y)  { return Direction::Right }
                    if self.can_go_down(x, y)   { return Direction::Down  }
                    if self.can_go_left(x, y)   { return Direction::Left  }
                    return Direction::Up
                },
                Direction::Left => {
                    if self.can_go_down(x, y)   { return Direction::Down  }
                    if self.can_go_left(x, y)   { return Direction::Left  }
                    if self.can_go_up(x, y)     { return Direction::Up    }
                    return Direction::Right
                },
                Direction::Up => {
                    if self.can_go_left(x, y)   { return Direction::Left  }
                    if self.can_go_up(x, y)     { return Direction::Up    }
                    if self.can_go_right(x, y)  { return Direction::Right }
                    return Direction::Down
                }
            }
        }

        fn set_tour_number(&mut self, x: usize, y: usize, num: i32) {
            if self.get_path_number(x, y) != 0 {
                return;
            }

            self.tour_to_number[x + y * GRID_WIDTH] = num;
        }

        fn generate_tour_number(&mut self) {
            const START_X: i32 = 0;
            const START_Y: i32 = 0;

            let x = START_X;
            let y = START_Y;

            let start_dir: Direction = match self.can_go_down(x, y) {
                true => Direction::Up,
                false => Direction::Left
            };

            let dir = start_dir;

            let mut number = 0;

            while {
                let next_dir = self.find_next_dir(x, y, dir);
                let x_u = x as usize;
                let y_u = y as usize;

                match dir {
                    Direction::Right => {
                        self.set_tour_number(x_u * 2, y_u * 2, number);
                        number += 1;

                        match next_dir {
                            Direction::Left => {
                                self.set_tour_number(x_u * 2 + 1,   y_u * 2,        number);
                                number += 1;
                                self.set_tour_number(x_u * 2 + 1,   y_u * 2 + 1,    number);
                                number += 1;
                                self.set_tour_number(x_u * 2,       y_u * 2 + 1,    number);
                                number += 1;
                            },
                            Direction::Down => {
                                self.set_tour_number(x_u * 2 + 1,   y_u * 2,        number);
                                number += 1;
                                self.set_tour_number(x_u * 2 + 1,   y_u * 2 + 1,    number);
                                number += 1;
                            },
                            _ => {
                                if next_dir == dir {
                                    self.set_tour_number(x_u * 2 + 1,   y_u * 2,        number);
                                    number += 1;
                                }
                            }
                        }
                    },
                    Direction::Down => {
                        self.set_tour_number(x_u * 2 + 1, y_u * 2, number);
                        number += 1;

                        match next_dir {
                            Direction::Up => {
                                self.set_tour_number(x_u * 2 + 1,   y_u * 2 + 1,    number);
                                number += 1;
                                self.set_tour_number(x_u * 2,       y_u * 2 + 1,    number);
                                number += 1;
                                self.set_tour_number(x_u * 2,       y_u * 2,        number);
                                number += 1;
                            },
                            Direction::Down => {
                                self.set_tour_number(x_u * 2 + 1,   y_u * 2,        number);
                                number += 1;
                                self.set_tour_number(x_u * 2 + 1,   y_u * 2 + 1,    number);
                                number += 1;
                            },
                            _ => {
                                if next_dir == dir {
                                    self.set_tour_number(x_u * 2 + 1,   y_u * 2,        number);
                                    number += 1;
                                }
                            }
                        }
                    },
                    Direction::Left => {},
                    Direction::Up => {},
                }


                number as usize != GRID_SIZE
            } {}
        }

        fn generate(&mut self) {
            self.generate_r(-1, -1, 0, 0);
            // self.generateTourNumber();
        }
    }
}