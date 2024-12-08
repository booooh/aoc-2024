use std::collections::HashSet;

use common::read_lines;

fn part1() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Position {
        col: i32,
        row: i32,
    }

    #[derive(Debug, Clone, Copy)]
    enum Direction {
        UP = 0,
        RIGHT = 1,
        DOWN = 2,
        LEFT = 3,
    }

    const directions: &'static [Direction] = &[
        Direction::UP,
        Direction::RIGHT,
        Direction::DOWN,
        Direction::LEFT,
    ];

    impl Direction {
        fn attempt_dirs(curr_direction_index: usize) -> Vec<Direction> {
            (0..4)
                .map(|d| directions[(d + curr_direction_index) % 4])
                .collect()
        }
    }

    #[derive(Debug, Clone)]
    struct Guard {
        positions_visted: HashSet<Position>,
        curr_position: Position,
        curr_direction_index: usize,
    }

    #[derive(Debug)]
    struct Map {
        obstacles: HashSet<Position>,
        width: i32,
        height: i32,
    }

    impl Map {
        fn new(input: &Vec<String>) -> (Self, Position) {
            let height = input.len() as i32;
            let width = input[0].len() as i32;

            let obstacles = input
                .iter()
                .flat_map(|s| s.chars())
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(i, _)| Position {
                    col: (i as i32 % width),
                    row: (i as i32 / height),
                })
                .collect::<HashSet<_>>();

            let guard = input
                .iter()
                .flat_map(|s| s.chars())
                .enumerate()
                .filter(|(_, c)| *c == '^')
                .map(|(i, _)| Position {
                    col: (i as i32 % width),
                    row: (i as i32 / height),
                })
                .next()
                .unwrap();

            return (
                Map {
                    obstacles,
                    width,
                    height,
                },
                guard,
            );
        }
    }

    impl Guard {
        // find what the guard's next position will be, but don't move
        fn next_pos(&self, map: &Map) -> Option<Position> {
            let mut d_col = 0;
            let mut d_row = 0;
            match directions[self.curr_direction_index] {
                Direction::UP => {
                    d_row = -1;
                }
                Direction::RIGHT => {
                    d_col = 1;
                }
                Direction::DOWN => {
                    d_row = 1;
                }
                Direction::LEFT => {
                    d_col = -1;
                }
            };

            let next_pos = Position {
                col: self.curr_position.col + d_col,
                row: self.curr_position.row + d_row,
            };

            if next_pos.col < 0
                || next_pos.row < 0
                || next_pos.col >= map.width
                || next_pos.row >= map.height
            {
                return None;
            }

            return Some(next_pos);
        }
        fn next_move(&mut self, map: &Map) -> bool {
            for _ in 0..4 {
                let possible_next = self.next_pos(map);
                if possible_next.is_none() {
                    return false;
                }

                // check if the next position is an obstacle
                if !map.obstacles.contains(&possible_next.unwrap()) {
                    self.curr_position = possible_next.unwrap();
                    self.positions_visted.insert(self.curr_position);
                    return true;
                } else {
                    self.curr_direction_index = (self.curr_direction_index + 1) % 4;
                }
            }
            panic!("We've gone through all directions, and have nowhere to go!")
        }

        fn new(start_loc: Position) -> Self {
            let mut guard = Guard {
                curr_direction_index: 0,
                curr_position: start_loc,
                positions_visted: HashSet::new(),
            };
            guard.positions_visted.insert(start_loc);
            guard
        }
    }

    let lines = read_lines("./day06/input").unwrap().collect::<Vec<_>>();
    let (map, guard_loc) = Map::new(&lines);

    let mut guard = Guard::new(guard_loc);

    while guard.next_move(&map) {
        println!("Guard moved!")
    }

    println!("num positions {}", guard.positions_visted.len());
}

fn part2() {
    let lines = read_lines("./day06/input").unwrap().collect::<Vec<_>>();
}
fn main() {
    part1();
    part2()
}
