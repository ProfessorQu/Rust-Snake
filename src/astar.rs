pub mod astar {
    use raylib::prelude::*;

    use crate::snake::snake::*;
    use crate::{GRID_WIDTH, GRID_HEIGHT, CELL_SIZE, CELL_SIZE_I};

    #[derive(Clone, Copy)]
    pub struct Node {
        parent: Pos,
        f: i32,
        g: i32,
        h: i32,
    }

    pub struct AStar {
        path: Vec<Pos>,
        end: Option<Node>,
        end_pos: Pos,
        nodes: Vec<Vec<Node>>,
        open: Vec<Pos>,
        closed: Vec<Vec<bool>>,
    }

    impl AStar {
        pub fn new() -> Self {
            Self {
                path: Vec::new(),
                end: None,
                end_pos: Pos::new(GRID_WIDTH, GRID_HEIGHT),
                nodes: Vec::new(),
                open: Vec::new(),
                closed: Vec::new()
            }
        }

        pub fn draw_path(&self, draw: &mut RaylibDrawHandle) {
            if self.path_found() {
                for piece in self.path.iter() {
                    let x = (piece.x * CELL_SIZE) as i32;
                    let y = (piece.y * CELL_SIZE) as i32;
                    draw.draw_rectangle(x, y, CELL_SIZE_I, CELL_SIZE_I, Color::GOLD);
                }
            }
        }

        pub fn get_next_move(&self, path_index: usize, snake: &Snake) -> Direction {
            let head = snake.body[0];
            let next = self.path[path_index];

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

        pub fn path_found(&self) -> bool {
            match self.end {
                Some(_) => true,
                None => false
            }
        }

        fn is_valid(&self, x: usize, y: usize) -> bool {
            x < GRID_WIDTH && y < GRID_HEIGHT
        }
    
        fn is_unblocked(&self, snake: &Snake, x: usize, y: usize) -> bool {
            !snake.body[1..].contains(&Pos::new(x, y))
        }
    
        fn is_destination(&self, food: &Food, x: usize, y: usize) -> bool {
            food.pos.x == x && food.pos.y == y
        }
    
        fn calculate_h_value(&self, food: &Food, x: usize, y: usize) -> i32 {
            (x as i32 - food.pos.x as i32).abs() + (y as i32 - food.pos.y as i32).abs()
        }
    
        fn test_neighbors(&mut self, snake: &Snake, food: &Food, x: usize, y: usize, check_x: bool) -> bool {
            let offsets = vec![-1 as i32, 1];

            // ====================
            // Test offsets
            // ====================
            for offset in offsets.iter() {
                let x_mult = check_x as i32;
                let y_mult = 1 - x_mult;

                let new_x = (x as i32 + (*offset * x_mult)) as usize;
                let new_y = (y as i32 + (*offset * y_mult)) as usize;

                if self.is_valid(new_x, new_y) {
                    if self.is_destination(&food, new_x, new_y) {
                        self.nodes[new_x][new_y].parent.x = x;
                        self.nodes[new_x][new_y].parent.y = y;
    
                        self.end = Some(self.nodes[new_x][new_y]);
                        self.end_pos = Pos::new(new_x, new_y);

                        return true
                    }
                    else if !self.closed[new_x][new_y] && self.is_unblocked(&snake, new_x, new_y) {
                        let new_g = self.nodes[x][y].g + 1;
                        let new_h = self.calculate_h_value(&food, new_x, new_y);
                        let new_f = new_g + new_h;

                        if self.nodes[new_x][new_y].f == i32::MAX || self.nodes[new_x][new_y].f > new_f {
                            self.open.push(Pos::new(new_x, new_y));
    
                            self.nodes[new_x][new_y].g = new_g;
                            self.nodes[new_x][new_y].h = new_h;
                            self.nodes[new_x][new_y].f = new_f;
                            self.nodes[new_x][new_y].parent.x = x;
                            self.nodes[new_x][new_y].parent.y = y;
                        }
                    }
                }
            }

            return false
        }

        fn test_pos(&mut self, snake: &Snake, food: &Food, x: usize, y: usize) -> bool {
            self.test_neighbors(&snake, &food, x, y, true) || self.test_neighbors(&snake, &food, x, y, false)
        }

        fn get_path(&mut self, node: &Node) -> bool {
            if node.parent.x == GRID_WIDTH || node.parent.y == GRID_HEIGHT {
                return false;
            }

            let parent = self.nodes[node.parent.x as usize][node.parent.y as usize];
            if self.get_path(&parent)
            {
                self.path.push(node.parent);
            }

            true
        }

        pub fn search(&mut self, snake: &Snake, food: &Food) {
            self.path = Vec::new();

            let head = snake.body[0];

            self.closed = vec![vec![false; GRID_HEIGHT]; GRID_WIDTH];
            self.nodes = Vec::new();

            self.end = None;
    
            if head.x == GRID_WIDTH || head.y == GRID_HEIGHT || head == food.pos {
                return;
            }
            
            for x in 0..GRID_WIDTH {
                self.nodes.push(Vec::new());
                for _ in 0..GRID_HEIGHT {
                    self.nodes[x].push(Node {
                        parent: Pos::new(GRID_WIDTH, GRID_HEIGHT),
                        f: i32::MAX,
                        g: i32::MAX,
                        h: i32::MAX
                    })
                }
            }
    
            self.open = Vec::new();
            self.open.push(head);

            self.nodes[head.x][head.y].f = 0;
            self.nodes[head.x][head.y].g = 0;
            self.nodes[head.x][head.y].h = 0;
            self.nodes[head.x][head.y].parent.x = GRID_WIDTH;
            self.nodes[head.x][head.y].parent.y = GRID_HEIGHT;

            let mut found = false;
    
            while !found && !self.open.is_empty() {
                let cur_pos = self.open.remove(0);
    
                let x = cur_pos.x as usize;
                let y = cur_pos.y as usize;
    
                self.closed[x][y] = true;
    
                found = self.test_pos(&snake, &food, x, y);
            }

            match self.end {
                Some(node) => { self.get_path(&node); self.path.push(self.end_pos) },
                None => {}
            };
        }
    }
    
}