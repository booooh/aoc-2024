use itertools::Itertools;
use std::{
    cmp,
    collections::{HashMap, HashSet},
    str::FromStr,
};

use common::read_lines;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    col: i64,
    row: i64,
}

#[derive(Debug, Clone)]
struct Grid {
    map: HashMap<char, Vec<Position>>,
    width: i64,
    height: i64,
}

impl Grid {
    fn antinodes(&self) -> HashSet<Position> {
        let mut antinodes = HashSet::new();
        for antena_type in self.map.keys() {
            let antena_positions = self.map.get(antena_type).unwrap();
            let combinations = antena_positions.iter().combinations(2).collect::<Vec<_>>();
            for pair in combinations {
                let dx: i64 = pair[1].col - pair[0].col;
                let dy: i64 = pair[1].row - pair[0].row;
                antinodes.insert(Position {
                    col: pair[1].col + dx,
                    row: pair[1].row + dy,
                });
                antinodes.insert(Position {
                    col: pair[0].col - dx,
                    row: pair[0].row - dy,
                });
            }
        }
        antinodes.retain(|p| p.col >= 0 && p.col < self.width && p.row >= 0 && p.row < self.height);

        antinodes
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseLineError;

impl FromStr for Grid {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let height = (s.len() as f64).sqrt() as i64;
        let width = height;
        for (row, grid_row) in s.split("\n").enumerate() {
            let antenas_in_row =
                grid_row
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(|(col, antena)| {
                        (
                            antena,
                            Position {
                                col: col as i64,
                                row: row as i64,
                            },
                        )
                    });

            for (antena, pos) in antenas_in_row {
                map.entry(antena).or_insert(vec![]).push(pos);
            }
        }

        Ok(Self { map, height, width })
    }
}

fn part1() {
    let grid_lines: String = read_lines("./day08/input")
        .unwrap()
        .collect::<Vec<_>>()
        .join("\n");
    let grid: Grid = grid_lines.parse().unwrap();
    println!("{:?}", grid.antinodes().len());
}

fn part2() {
    let lines = read_lines("./day08/input").unwrap().collect::<Vec<_>>();
}
fn main() {
    part1();
    part2()
}
