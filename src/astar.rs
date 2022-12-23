pub mod astar {
    use raylib::prelude::*;

    use crate::snake::snake::*;
    use crate::{GRID_WIDTH, GRID_HEIGHT, CELL_SIZE, CELL_SIZE_I, SEARCH_EVERY};

    #[derive(Clone, Copy)]
    pub struct Node {
        pos: Pos,
        parent: Pos,
        f: i32,
        g: i32,
        h: i32,
    }

    pub struct AStar {
        pub path: Vec<Direction>,
        path_index: usize,
        end: Option<Node>,
        end_pos: Pos,
        nodes: Vec<Vec<Node>>,
        open: Vec<Pos>,
        closed: Vec<Vec<bool>>,
        random_dir: bool
    }

    impl AStar {
        pub fn new(random_dir: bool) -> Self {
            Self {
                path: Vec::new(),
                path_index: 0,
                end: None,
                end_pos: Pos::new(GRID_WIDTH, GRID_HEIGHT),
                nodes: Vec::new(),
                open: Vec::new(),
                closed: Vec::new(),
                random_dir
            }
        }

        pub fn reset(&mut self) {
            self.path = Vec::new();
            self.path_index = 0;
            self.end = None;
            self.end_pos = Pos::new(GRID_WIDTH, GRID_HEIGHT);
            self.nodes = Vec::new();
            self.open = Vec::new();
            self.closed = Vec::new();
        }

        pub fn update(&mut self, snake: &mut Snake, food: &mut Food, frame_count: &usize, score: &mut i32) {
            let score_before = *score;
            let path_found = self.path_found();

            if path_found {
                snake.set_next_direction(self.get_next_move());
                self.path_index += 1;
            }
            else if !path_found && snake.would_collide() {
                if self.random_dir {
                    let dir = snake.get_random_free_dir();
                    snake.set_next_direction(dir);
                }
                else {
                    let dir = snake.get_dir_of_free_space();
                    snake.set_next_direction(dir);
                }
            }

            snake.update(food, score);
    
            if score_before - *score != 0 || (!path_found && frame_count % SEARCH_EVERY == 0) {
                self.search(&snake, &food);
                self.path_index = 0;
            }
        }

        pub fn draw_path(&self, draw: &mut RaylibDrawHandle, snake: &Snake) {
            if self.path_found() {
                let mut cur = snake.head();
                for dir in self.path[self.path_index..].iter() {
                    cur = cur.transform(dir);
                    let x = (cur.x * CELL_SIZE) as i32;
                    let y = (cur.y * CELL_SIZE) as i32;
                    draw.draw_rectangle(x, y, CELL_SIZE_I, CELL_SIZE_I, Color::GRAY);
                }
            }
        }

        pub fn get_next_move(&self) -> Direction {
            self.path[self.path_index]
        }

        pub fn path_found(&self) -> bool {
            match self.end {
                Some(_) => true,
                None => false
            }
        }

        fn is_unblocked(&self, obstacles: &Vec<Pos>, x: usize, y: usize) -> bool {
            !obstacles.contains(&Pos::new(x, y))
        }
    
        fn is_destination(&self, end: &Pos, x: usize, y: usize) -> bool {
            end.x == x && end.y == y
        }
    
        fn calculate_h_value(&self, end: &Pos, x: usize, y: usize) -> i32 {
            (x as i32 - end.x as i32).abs() + (y as i32 - end.y as i32).abs()
        }
    
        fn test_neighbors(&mut self, end: &Pos, obstacles: &Vec<Pos>, x: usize, y: usize, check_x: bool) -> bool {
            let offsets = vec![-1 as i32, 1];

            // ====================
            // Test offsets
            // ====================
            for offset in offsets.iter() {
                let x_mult = check_x as i32;
                let y_mult = 1 - x_mult;

                let new_x = (x as i32 + (*offset * x_mult)) as usize;
                let new_y = (y as i32 + (*offset * y_mult)) as usize;

                if in_bounds(new_x, new_y) {
                    if self.is_destination(end, new_x, new_y) {
                        self.nodes[new_x][new_y].parent.x = x;
                        self.nodes[new_x][new_y].parent.y = y;
    
                        self.end = Some(self.nodes[new_x][new_y]);
                        self.end_pos = Pos::new(new_x, new_y);

                        return true
                    }
                    else if !self.closed[new_x][new_y] && self.is_unblocked(&obstacles, new_x, new_y) {
                        let new_g = self.nodes[x][y].g + 1;
                        let new_h = self.calculate_h_value(end, new_x, new_y);
                        let new_f = new_g + new_h;

                        if self.nodes[new_x][new_y].f == i32::MAX || self.nodes[new_x][new_y].f < new_f {
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

        fn test_pos(&mut self, end: &Pos, obstacles: &Vec<Pos>, x: usize, y: usize) -> bool {
            self.test_neighbors(end, obstacles, x, y, true) || self.test_neighbors(end, obstacles, x, y, false)
        }

        fn get_path(&mut self, node: &Node) {
            if node.parent.x == GRID_WIDTH || node.parent.y == GRID_HEIGHT {
                return;
            }

            let parent = self.nodes[node.parent.x as usize][node.parent.y as usize];
            self.get_path(&parent);
            self.path.push(parent.pos.get_dir_to(&node.pos).unwrap());
        }

        pub fn shortest_path(&mut self, start: &Pos, end: &Pos, obstacles: &Vec<Pos>) {
            self.path = Vec::new();

            self.closed = vec![vec![false; GRID_HEIGHT]; GRID_WIDTH];
            self.nodes = Vec::new();

            self.end = None;
    
            if start.x == GRID_WIDTH || start.y == GRID_HEIGHT || start == end {
                return
            }
            
            for x in 0..GRID_WIDTH {
                self.nodes.push(Vec::new());
                for y in 0..GRID_HEIGHT {
                    self.nodes[x].push(Node {
                        pos: Pos::new(x, y),
                        parent: Pos::new(GRID_WIDTH, GRID_HEIGHT),
                        f: i32::MAX,
                        g: i32::MAX,
                        h: i32::MAX
                    })
                }
            }
    
            self.open = Vec::new();
            self.open.push(*start);

            self.nodes[start.x][start.y].f = 0;
            self.nodes[start.x][start.y].g = 0;
            self.nodes[start.x][start.y].h = 0;
            self.nodes[start.x][start.y].parent.x = GRID_WIDTH;
            self.nodes[start.x][start.y].parent.y = GRID_HEIGHT;

            let mut found = false;
    
            while !found && !self.open.is_empty() {
                let cur_pos = self.open.remove(0);
    
                let x = cur_pos.x as usize;
                let y = cur_pos.y as usize;
    
                self.closed[x][y] = true;
    
                found = self.test_pos(end, &obstacles, x, y);
            }

            match self.end {
                Some(node) => { self.get_path(&node); },
                None => {}
            };
        }

        pub fn search(&mut self, snake: &Snake, food: &Food) {
            self.shortest_path(&snake.head(), &food.pos, &snake.exclude_head())
        }
    }
    
}