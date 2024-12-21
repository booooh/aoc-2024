use common::read_lines;

use regex::Regex;

fn part1() {
    let instructions = read_lines("./day03/input").unwrap().collect::<String>();
    let mul_pattern = r"mul\(([0-9]+),([0-9]+)\)";
    let mul_re = Regex::new(mul_pattern).unwrap();
    let mul_iter = mul_re.captures_iter(&instructions).map(|m| m.extract());
    let mut total = 0;

    for (_, [a, b]) in mul_iter {
        total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }

    println!("{:?}", total);
}

fn part2() {
    let instructions = read_lines("./day03/input").unwrap().collect::<String>();
    let mul_pattern = r"don't|do|mul\(([0-9]+),([0-9]+)\)";
    let mul_re = Regex::new(mul_pattern).unwrap();
    let mul_iter = mul_re.captures_iter(&instructions);
    let mut total = 0;

    let mut enabled_factor = 1;
    for a in mul_iter {
        match a.get(0).unwrap().as_str() {
            "don't" => {
                enabled_factor = 0;
            }
            "do" => {
                enabled_factor = 1;
            }
            _ => {
                total += (a.get(1).unwrap().as_str().parse::<i32>().unwrap()
                    * a.get(2).unwrap().as_str().parse::<i32>().unwrap())
                    * enabled_factor;
            }
        };
    }
    println!("{:?}", total);
}

fn main() {
    part1();
    part2();
}
