use std::{collections::HashMap, str::FromStr};

use common::read_lines;
use regex::Regex;
use std::sync::LazyLock;

type Point = (i32, i32);
type Velocity = (i32, i32);

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

// const WIDTH: i32 = 11;
// const HEIGHT: i32 = 7;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Robot {
    curr_pos: Point,
    velocity: Velocity,
}

#[derive(Debug)]
struct RobotParseErr;

impl Robot {
    fn new(p: Point, v: Velocity) -> Self {
        return Self {
            curr_pos: p,
            velocity: v,
        };
    }

    fn do_move(&mut self) {
        let new_pos = (
            (self.curr_pos.0 + self.velocity.0).rem_euclid(WIDTH),
            (self.curr_pos.1 + self.velocity.1).rem_euclid(HEIGHT),
        );
        self.curr_pos = new_pos;
    }

    fn get_quad(&self) -> i32 {
        let x_first_half = self.curr_pos.0 < WIDTH / 2;
        let x_second_half = self.curr_pos.0 > WIDTH / 2;
        let y_first_half = self.curr_pos.1 < HEIGHT / 2;
        let y_second_half = self.curr_pos.1 > HEIGHT / 2;

        match (x_first_half, x_second_half, y_first_half, y_second_half) {
            (true, _, true, _) => 0,
            (_, true, true, _) => 1,
            (true, _, _, true) => 2,
            (_, true, _, true) => 3,
            _ => -1,
        }
    }
}

impl FromStr for Robot {
    type Err = RobotParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap());
        let m = RE.captures(s).unwrap();
        let [px, py, vx, vy] = m
            .extract::<4>()
            .1
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        return Ok(Self::new((px, py), (vx, vy)));
    }
}

fn part1() {
    let lines = read_lines("./day14/input").unwrap().collect::<Vec<_>>();
    let mut robots: Vec<Robot> = lines.iter().map(|l| l.parse().unwrap()).collect();
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.do_move();
        }
    }
    let mut robots_in_quads = HashMap::<i32, i32>::new();
    for robot in robots.iter() {
        *robots_in_quads.entry(robot.get_quad()).or_insert(0) += 1;
    }

    let mut safety_factor = 1;
    for quad_count in robots_in_quads.iter().filter(|(quad, _)| quad >= &&0) {
        safety_factor *= quad_count.1;
    }

    println!("safety factor: {}", safety_factor);
}

fn part2() {
    let lines = read_lines("./day14/input").unwrap().collect::<Vec<_>>();
}
fn main() {
    part1();
    part2()
}
