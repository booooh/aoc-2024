use common::read_lines;

use regex::Regex;

fn part1() {
    let instructions = read_lines("./day03/input").unwrap().collect::<String>();
    let mul_pattern = r"mul\(([0-9]+),([0-9]+)\)";
    let mul_re = Regex::new(&mul_pattern).unwrap();
    let mul_iter = mul_re.captures_iter(&instructions).map(|m| m.extract());
    let mut total = 0;

    for (_, [a, b]) in mul_iter {
        total += (a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap());
    }

    println!("{:?}", total);
}

fn part2() {}

fn main() {
    part1();
    part2();
}
