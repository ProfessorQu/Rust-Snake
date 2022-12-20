pub mod astar {
    use raylib::prelude::*;

    use crate::snake::snake::*;
    use crate::{GRID_SIZE, CELL_SIZE_I};

    #[derive(Clone, Copy, Debug)]
    pub struct Node {
        pub parent_x: usize,
        pub parent_y: usize,
        pub f: i32,
        pub g: i32,
        pub h: i32,
    }

    pub struct AStar {
        pub path: Vec<Vector2>,
        pub end: Option<Node>,
        end_pos: Vector2,
        nodes: Vec<Vec<Node>>,
        open: Vec<Vector2>,
        closed: Vec<Vec<bool>>,
    }

    impl AStar {
        pub fn new() -> Self {
            Self {
                path: Vec::new(),
                end: None,
                end_pos: rvec2(GRID_SIZE, GRID_SIZE),
                nodes: Vec::new(),
                open: Vec::new(),
                closed: Vec::new()
            }
        }

        pub fn draw_path(&self, draw: &mut RaylibDrawHandle) {
            for piece in self.path.iter() {
                let x = piece.x as i32 * CELL_SIZE_I;
                let y = piece.y  as i32* CELL_SIZE_I;
                draw.draw_rectangle(x, y, CELL_SIZE_I, CELL_SIZE_I, Color::GOLD);
            }
        }

        pub fn get_next_move(&self, path_index: usize, snake: &Snake) -> Direction {
            let head = snake.body[0];
            let next = self.path[path_index];

            match next - head {
                Vector2 { x, y } if x == 1.0  && y == 0.0 => Direction::Right,
                Vector2 { x, y } if x == -1.0 && y == 0.0 => Direction::Left,
                Vector2 { x, y } if y == 1.0  && x == 0.0 => Direction::Down,
                Vector2 { x, y } if y == -1.0 && x == 0.0 => Direction::Up,
                _ => Direction::Left
            }
        }

        fn is_valid(&self, x: usize, y: usize) -> bool {
            x < GRID_SIZE as usize && y < GRID_SIZE as usize
        }
    
        fn is_unblocked(&self, snake: &Snake, x: usize, y: usize) -> bool {
            !snake.body[1..].contains(&rvec2(x as i32, y as i32))
        }
    
        fn is_destination(&self, food: &Food, x: usize, y: usize) -> bool {
            food.pos.x == x as f32 && food.pos.y == y as f32
        }
    
        fn calculate_h_value(&self, food: &Food, x: usize, y: usize) -> i32 {
            (x as i32 - food.pos.x as i32).abs() + (y as i32 - food.pos.y as i32).abs()
        }
    
        fn test_pos(&mut self, snake: &Snake, food: &Food, x: usize, y: usize) -> bool {
            let offsets = vec![-1 as i32, 1];

            // ====================
            // Test x offsets
            // ====================
            for x_offset in offsets.iter() {
                let new_x = (x as i32 + *x_offset) as usize;
                if self.is_valid(new_x, y) {
                    if self.is_destination(&food, new_x, y) {
                        self.nodes[new_x][y].parent_x = x;
                        self.nodes[new_x][y].parent_y = y;
    
                        self.end = Some(self.nodes[new_x][y]);
                        self.end_pos = rvec2(new_x as i32, y as i32);

                        return true;
                    }
                    else if !self.closed[new_x][y] && self.is_unblocked(&snake, new_x, y) {
                        let new_g = self.nodes[x][y].g + 1;
                        let new_h = self.calculate_h_value(&food, new_x, y);
                        let new_f = new_g + new_h;

                        if self.nodes[new_x][y].f == i32::MAX || self.nodes[new_x][y].f > new_f {
                            self.open.push(rvec2(new_x as i32, y as i32));
    
                            self.nodes[new_x][y].g = new_g;
                            self.nodes[new_x][y].h = new_h;
                            self.nodes[new_x][y].f = new_f;
                            self.nodes[new_x][y].parent_x = x;
                            self.nodes[new_x][y].parent_y = y;
                        }
                    }
                }
            }

            // ====================
            // Test y offsets
            // ====================
            for y_offset in offsets.iter() {
                let new_y = (y as i32 + *y_offset) as usize;
                if self.is_valid(x, new_y) {
                    if self.is_destination(&food, x, new_y) {
                        self.nodes[x][new_y].parent_x = x;
                        self.nodes[x][new_y].parent_y = y;
                        
                        self.end = Some(self.nodes[x][new_y]);
                        self.end_pos = rvec2(x as i32, new_y as i32);

                        return true;
                    }
                    else if !self.closed[x][new_y] && self.is_unblocked(&snake, x, new_y) {
                        let new_g = self.nodes[x][y].g + 1;
                        let new_h = self.calculate_h_value(&food, x, new_y);
                        let new_f = new_g + new_h;
    
                        if self.nodes[x][new_y].f == i32::MAX || self.nodes[x][new_y].f > new_f {
                            self.open.push(rvec2(x as i32, new_y as i32));
    
                            self.nodes[x][new_y].g = new_g;
                            self.nodes[x][new_y].h = new_h;
                            self.nodes[x][new_y].f = new_f;
                            self.nodes[x][new_y].parent_x = x;
                            self.nodes[x][new_y].parent_y = y;
                        }
                    }
                }
            }

            return false;
        }

        fn get_path(&mut self, node: &Node) {
            if node.parent_x == GRID_SIZE as usize || node.parent_y == GRID_SIZE as usize {
                return;
            }

            let parent = self.nodes[node.parent_x][node.parent_y];
            self.get_path(&parent);
            self.path.push(rvec2(node.parent_x as i32, node.parent_y as i32));
        }

        pub fn search(&mut self, snake: &Snake, food: &Food) {
            self.path = Vec::new();

            let head = snake.body[0];
    
            self.closed = vec![vec![false; GRID_SIZE as usize]; GRID_SIZE as usize];
            self.nodes = Vec::new();
    
            if head == food.pos {
                self.end = None;
                return;
            }
            
            for x in 0..GRID_SIZE {
                self.nodes.push(Vec::new());
                for _ in 0..GRID_SIZE {
                    self.nodes[x as usize].push(Node {
                        parent_x: GRID_SIZE as usize,
                        parent_y: GRID_SIZE as usize,
                        f: i32::MAX,
                        g: i32::MAX,
                        h: i32::MAX
                    })
                }
            }
    
            self.open = Vec::new();
            self.open.push(head);

            let head_x = head.x as usize;
            let head_y = head.y as usize;

            self.nodes[head_x][head_y].f = 0;
            self.nodes[head_x][head_y].g = 0;
            self.nodes[head_x][head_y].h = 0;
            self.nodes[head_x][head_y].parent_x = food.pos.x as usize;
            self.nodes[head_x][head_y].parent_x = food.pos.y as usize;

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