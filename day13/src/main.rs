use common::read_lines;
use regex::Regex;
use std::str::FromStr;

type Button = (i32, i32);

#[derive(Debug, PartialEq, PartialOrd)]
struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: (i32, i32),
}

impl ClawMachine {
    fn new(button_a: Button, button_b: Button, prize: (i32, i32)) -> Self {
        Self {
            button_a,
            button_b,
            prize,
        }
    }

    fn solve(&self) -> (i32, i32) {
        let det = (self.button_a.0 * self.button_b.1) - (self.button_a.1 * self.button_b.0);
        let m = ((self.button_b.1 * self.prize.0) - (self.button_b.0 * self.prize.1)) / det;
        let n = ((-self.button_a.1 * self.prize.0) + (self.button_a.0 * self.prize.1)) / det;

        // sanity check - since we used integers, not floating points
        if (m * self.button_a.0) + (n * self.button_b.0) != self.prize.0
            || (m * self.button_a.1) + (n * self.button_b.1) != self.prize.1
        {
            return (-1, -1);
        }

        return (m, n);
    }

    fn cost(&self) -> Option<i32> {
        match self.solve() {
            (-1, -1) => None,
            (m, n) => Some(3 * m + n),
        }
    }
}

#[derive(Debug)]
struct ClawMachineErr;

impl FromStr for ClawMachine {
    type Err = ClawMachineErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /*
            Button A: X+22, Y+88
            Button B: X+90, Y+28
            Prize: X=6496, Y=3076
        */

        let re = Regex::new(r"(?m)Button A: X\+(?<a_x>\d+), Y\+(?<a_y>\d+)\nButton B: X\+(?<b_x>\d+), Y\+(?<b_y>\d+)\nPrize: X=(?<prize_x>\d+), Y=(?<prize_y>\d+)\n?").unwrap();
        let m = re.captures(s).unwrap();
        let [a_x, a_y, b_x, b_y, prize_x, prize_y] = m
            .extract::<6>()
            .1
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        return Ok(ClawMachine::new((a_x, a_y), (b_x, b_y), (prize_x, prize_y)));
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<ClawMachine> {
    lines
        .chunks(4)
        .map(|c| c.join("\n").parse().unwrap())
        .collect()
}

fn part1() {
    let lines = read_lines("./day13/input").unwrap().collect::<Vec<_>>();
    let machines = parse_input(&lines);

    let mut total_cost = 0;
    for m in machines {
        if let Some(cost) = m.cost() {
            total_cost += cost;
        }
    }
    println!("{:?}", total_cost);
}

fn part2() {
    let lines = read_lines("./day13/input").unwrap().collect::<Vec<_>>();
}
fn main() {
    part1();
    part2()
}
