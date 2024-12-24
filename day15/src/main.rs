use std::collections::{HashMap, HashSet};

use common::read_lines;

type Position = (i32, i32);

fn part1() {
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

        fn render(&self) {
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

    warehouse.render();

    for l in lines_iter {
        for c in l.chars() {
            warehouse.move_robot(c);
        }
    }
    warehouse.render();
    println!("gps: {}", warehouse.gps());
}

fn part2() {
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
            println!(
                "robot wants to move from {:?} to position {:?}",
                self.robot, new_pos
            );
            if self.walls.contains(&new_pos) {
                return;
            }

            if !self.boxes.contains(&new_pos) && !self.boxes.contains(&(new_pos.0 - 1, new_pos.1)) {
                self.robot = new_pos;
                return;
            }

            // there's a box, try to see if we can shift it
            let mut box_pos = new_pos;

            if delta.1 == 0 {
                let mut affected_boxes = HashSet::<Position>::new();
                while self.boxes.contains(&box_pos)
                    || self.boxes.contains(&(box_pos.0 + delta.0, box_pos.1))
                {
                    affected_boxes.insert(box_pos);
                    box_pos = (box_pos.0 + delta.0, box_pos.1);
                }

                // println!("affected boxes before intersection: {:?}", affected_boxes);

                // this includes positions that are not box starts so make sure we narrow down the set
                affected_boxes = affected_boxes.intersection(&self.boxes).copied().collect();

                // println!("affected boxes are: {:?}", affected_boxes);

                // reached the end of the box-sequence, if box_pos is empty, move the box from the new position to this location
                // otherwise do nothing
                if self.walls.contains(&box_pos) || self.walls.contains(&(box_pos.0 + 1, box_pos.1))
                {
                    return;
                }

                // remove all of the boxes
                for b in affected_boxes.iter() {
                    self.boxes.remove(&b);
                }

                // update all of the boxes's locations
                for b in affected_boxes.iter() {
                    self.boxes.insert((b.0 + delta.0, b.1));
                }

                self.robot = new_pos;
            } else {
                // moving up or down, we may impact multiple boxes on each row, keep a set of boxes that are impacted in each row
                let mut affected_boxes = HashMap::<i32, HashSet<Position>>::new();

                let mut tmp = HashSet::new();
                if self.boxes.contains(&new_pos) {
                    tmp.insert(new_pos);
                } else {
                    tmp.insert((new_pos.0 - 1, new_pos.1));
                }
                affected_boxes.insert(new_pos.1, tmp);

                // until we reach a wall, or no boxes in the current row
                let mut curr_row = new_pos.1;
                // println!(
                //     "before loop: curr_row={}, affected_boxes={:?}",
                //     curr_row, affected_boxes
                // );
                loop {
                    let mut tmp = HashSet::new();
                    let curr_boxes = affected_boxes.get(&curr_row).unwrap();
                    for b in curr_boxes {
                        // see if this box is blocked by a wall
                        // println!("  checking neighboring boxes for box at {:?}", b);
                        if self.walls.contains(&(b.0, b.1 + &delta.1))
                            || self.walls.contains(&(b.0 + 1, b.1 + &delta.1))
                        {
                            return;
                        }

                        // see if moving this box would impact other boxes
                        if self.boxes.contains(&(b.0, b.1 + delta.1)) {
                            tmp.insert((b.0, b.1 + delta.1));
                        }

                        if self.boxes.contains(&(b.0 - 1, b.1 + delta.1)) {
                            tmp.insert((b.0 - 1, b.1 + delta.1));
                        }

                        if self.boxes.contains(&(b.0 + 1, b.1 + delta.1)) {
                            tmp.insert((b.0 + 1, b.1 + delta.1));
                        }
                    }

                    if tmp.is_empty() {
                        break;
                    } else {
                        curr_row += delta.1;
                        // println!("  boxes found for row {} {:?}", curr_row, tmp);
                        affected_boxes.insert(curr_row, tmp);
                    }
                }

                // reached the end of the box-sequence, i
                // remove all of the boxes
                for r in affected_boxes.values() {
                    for b in r.iter() {
                        self.boxes.remove(b);
                    }
                }

                // update all of the boxes's locations
                for r in affected_boxes.values() {
                    for b in r.iter() {
                        self.boxes.insert((b.0, b.1 + delta.1));
                    }
                }

                self.robot = new_pos;
            }
        }

        fn render(&self) {
            for r in 0..self.height {
                for c in 0..self.width {
                    let coord = (c, r);
                    if self.walls.contains(&coord) {
                        print!("#");
                    } else if self.boxes.contains(&coord) {
                        print!("[]");
                    } else if self.robot == coord {
                        print!("@");
                    } else if !self.boxes.contains(&(c - 1, r)) {
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

    let lines = read_lines("./day15/input").unwrap().collect::<Vec<_>>();
    let mut warehouse = Warehouse::default();
    let mut lines_iter = lines.iter();
    warehouse.width = (lines[0].len() * 2) as i32;

    for (row, line) in lines_iter.by_ref().enumerate() {
        if line.is_empty() {
            break;
        }

        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    warehouse.walls.insert((2 * col as i32, row as i32));
                    warehouse.walls.insert(((2 * col + 1) as i32, row as i32));
                }
                '.' => {}
                'O' => {
                    warehouse.boxes.insert((2 * col as i32, row as i32)); // only add the starting location
                }
                '@' => warehouse.robot = (2 * col as i32, row as i32),
                _ => panic!("unexpected char in warehouse"),
            };
        }
        warehouse.height += 1;
    }

    warehouse.render();
    for l in lines_iter {
        for c in l.chars() {
            warehouse.move_robot(c);
        }
    }
    warehouse.render();
    println!("gps score: {}", warehouse.gps());
}
fn main() {
    part1();
    part2();
}
