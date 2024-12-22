use std::collections::{HashMap, HashSet};

use common::read_lines;

type Position = (i32, i32);

#[derive(Debug, Default)]
struct Warehouse {
    width: i32,
    height: i32,
    boxes: HashSet<Position>,
    walls: HashSet<Position>,
    robot: Position,
}

impl Warehouse {
    fn move_robot(&mut self, direction: char) {
        let delta = match direction {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("unexpected direction"),
        };

        let new_pos = (self.robot.0 + delta.0, self.robot.1 + delta.1);

        if self.walls.contains(&new_pos) {
            return;
        }

        if !self.boxes.contains(&new_pos) {
            self.robot = new_pos;
            return;
        }

        // there's a box, try to see if we can shift it
        let mut box_pos = new_pos;
        while self.boxes.contains(&box_pos) {
            box_pos = (box_pos.0 + delta.0, box_pos.1 + delta.1);
        }

        // reached the end of the box-sequence, if box_pos is empty, move the box from the new position to this location
        // otherwise do nothing
        if self.walls.contains(&box_pos) {
            return;
        }

        self.boxes.insert(box_pos);
        self.boxes.remove(&new_pos);

        self.robot = new_pos;
    }

    fn redner(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                let coord = (c, r);
                if self.walls.contains(&coord) {
                    print!("#");
                } else if self.boxes.contains(&coord) {
                    print!("O");
                } else if self.robot == coord {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn gps(&self) -> i64 {
        let mut gps_score = 0_i64;

        for b in self.boxes.iter() {
            gps_score += b.1 as i64 * 100_i64 + b.0 as i64
        }
        return gps_score;
    }
}

fn part1() {
    let lines = read_lines("./day15/input").unwrap().collect::<Vec<_>>();

    let mut warehouse = Warehouse::default();
    let mut lines_iter = lines.iter();
    warehouse.width = lines[0].len() as i32;

    for (row, line) in lines_iter.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }

        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    warehouse.walls.insert((col as i32, row as i32));
                }
                '.' => {}
                'O' => {
                    warehouse.boxes.insert((col as i32, row as i32));
                }
                '@' => warehouse.robot = (col as i32, row as i32),
                _ => panic!("unexpected char in warehouse"),
            };
        }
        warehouse.height += 1;
    }

    warehouse.redner();

    for l in lines_iter {
        for c in l.chars() {
            warehouse.move_robot(c);
        }
    }
    warehouse.redner();
    println!("gps: {}", warehouse.gps());
}

fn part2() {
    let lines = read_lines("./day15/input").unwrap().collect::<Vec<_>>();
}
fn main() {
    part1();
    part2()
}
