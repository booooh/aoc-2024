use std::{collections::HashSet, hash::Hash};

use common::read_lines;

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

const directions: &[Direction] = &[
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
    positions_visted: HashSet<(Position, usize)>,
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

        (
            Map {
                obstacles,
                width,
                height,
            },
            guard,
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum MoveResult {
    SimpleMove,
    CycleFound,
    OutOfBounds,
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

        Some(next_pos)
    }

    fn next_move(&mut self, map: &Map) -> MoveResult {
        for _ in 0..4 {
            let possible_next = self.next_pos(map);
            if possible_next.is_none() {
                return MoveResult::OutOfBounds;
            }

            // check if the next position is an obstacle
            if !map.obstacles.contains(&possible_next.unwrap()) {
                self.curr_position = possible_next.unwrap();
                if self
                    .positions_visted
                    .contains(&(self.curr_position, self.curr_direction_index))
                {
                    return MoveResult::CycleFound;
                }

                self.positions_visted
                    .insert((self.curr_position, self.curr_direction_index));
                return MoveResult::SimpleMove;
            } else {
                self.curr_direction_index = (self.curr_direction_index + 1) % 4;
            }
        }
        panic!("We've gone through all directions, and have nowhere to go!")
    }

    fn unique_positions(&self) -> HashSet<&Position> {
        self.positions_visted
            .iter()
            .map(|(p, _)| p)
            .collect::<HashSet<_>>()
    }

    fn new(start_loc: Position) -> Self {
        let mut guard = Guard {
            curr_direction_index: 0,
            curr_position: start_loc,
            positions_visted: HashSet::new(),
        };
        guard.positions_visted.insert((start_loc, 0));
        guard
    }
}

fn part1() {
    let lines = read_lines("./day06/input").unwrap().collect::<Vec<_>>();
    let (map, guard_loc) = Map::new(&lines);

    let mut guard = Guard::new(guard_loc);

    while guard.next_move(&map) == MoveResult::SimpleMove {
        println!("Guard moved!")
    }

    let num_positions = guard.unique_positions().len();
    println!("num positions {}", num_positions);
}

fn part2() {
    let lines = read_lines("./day06/input").unwrap().collect::<Vec<_>>();
    let (mut map, guard_loc) = Map::new(&lines);
    let mut guard = Guard::new(guard_loc);

    let mut num_cycles = 0;
    loop {
        guard.next_move(&map);
        if let Some(np) = guard.next_pos(&map) {
            if map.obstacles.contains(&np) || guard.unique_positions().contains(&np) {
                // not interesting, since I'm not adding an obstacle
                continue;
            } else {
                map.obstacles.insert(np);
                println!("adding position: {:?}", np);
                let mut new_guard = guard.clone();
                loop {
                    match new_guard.next_move(&map) {
                        MoveResult::SimpleMove => {
                            println!("doing a simple move {:?}", new_guard.curr_position)
                        }
                        MoveResult::CycleFound => {
                            println!("found cycle!");
                            num_cycles += 1;
                            break;
                        }
                        MoveResult::OutOfBounds => break,
                    }
                }
                map.obstacles.remove(&np);
            }
        } else {
            break;
        }
        println!("{}", num_cycles);
    }
}
fn main() {
    part1();
    part2()
}
