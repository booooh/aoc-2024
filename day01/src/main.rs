use std::{collections::HashMap, iter::zip};

use common::read_lines;

fn part1() {
    let pairs = read_lines("./day01/input").unwrap().map(|x| {
        x.split_whitespace()
            .map(|s| s.to_owned().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    let mut first_list = Vec::<i32>::new();
    let mut second_list = Vec::<i32>::new();
    for p in pairs {
        first_list.push(p[0]);
        second_list.push(p[1]);
    }
    first_list.sort();
    second_list.sort();
    let sum_diff = zip(first_list, second_list)
        .map(|p| p.0.abs_diff(p.1))
        .sum::<u32>();

    println!("{:?}", sum_diff);
}

fn part2() {
    let pairs = read_lines("./day01/input").unwrap().map(|x| {
        x.split_whitespace()
            .map(|s| s.to_owned().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    let mut first_list = Vec::<i32>::new();
    let mut second_map = HashMap::<i32, i32>::new();
    for p in pairs {
        first_list.push(p[0]);
        *second_map.entry(p[1]).or_insert(0) += 1;
    }
    // first_list.sort();
    // second_list.sort();
    let mut similarity_score = 0;
    for loc in first_list {
        similarity_score += *second_map.entry(loc).or_insert(0) * loc;
    }
    println!("{:?}", similarity_score);
}

fn main() {
    part1();
    part2();
}
