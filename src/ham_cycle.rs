
#[allow(dead_code)]
pub mod ham_cycle {
    use crate::{snake::snake::*, astar::astar::AStar};

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
        astar: AStar,
    }
    
    impl HamiltonianCycle {
        pub fn new() -> Self {
            Self {
                astar: AStar::new()
            }
        }

        fn shortest_path(&mut self, snake: &Snake) {
            self.astar.shortest_path(&snake.head(), &snake.tail(), Vec::new());
        }
    }
}