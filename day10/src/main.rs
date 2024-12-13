use std::{collections::VecDeque, iter::zip, str::FromStr};

use common::read_lines;

#[derive(Debug, Clone, Copy)]
struct Node {
    col: i32,
    row: i32,
    height: i32,
}

#[derive(Debug, Clone)]
struct TopoMap {
    nodes: Vec<Node>, // all nodes
    height: i32,
    width: i32,
}

const STEP: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

impl TopoMap {
    fn get_node(&self, col: i32, row: i32) -> Option<&Node> {
        if col < 0 || row < 0 || col >= self.width || row >= self.height {
            return None;
        }
        let idx: usize = (self.width * row + col) as usize;
        Some(&self.nodes[idx])
    }

    fn get_neighbors(&self, node: &Node) -> Vec<&Node> {
        // let mut neighbors = vec![];

        let neighbors: Vec<_> = STEP
            .iter()
            .map(|(dx, dy)| self.get_node(node.col + dx, node.row + dy))
            .filter(|x| x.is_some())
            .map(|n| n.unwrap())
            .collect();

        return neighbors;
    }

    fn count_trails(&self, node: &Node, counts: &mut Vec<i32>) {
        let idx = (node.col + node.row * self.width) as usize;
        if node.height == 9 {
            counts[idx] = 1;
            println!("counts[{:?}]=1", idx);
        } else {
            // number of trails that pass through this node
            counts[idx] = self
                .get_neighbors(node)
                .iter()
                .filter(|n| n.height == node.height + 1)
                .map(|n| counts[(n.col + n.row * self.width) as usize])
                .sum();
            println!("counts[{:?}]={:?}", idx, counts[idx]);
        }
    }
    fn count_all_trails(&self) -> Vec<i32> {
        let mut counts = vec![];
        counts.resize((self.height * self.width) as usize, 0);
        for height in (0..10).rev() {
            println!("counting for height: {}", height);
            let height_nodes = self.nodes.iter().filter(|n| n.height == height);
            for node in height_nodes {
                self.count_trails(node, &mut counts);
            }
        }
        return counts;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseMapError;

impl FromStr for TopoMap {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.split("\n").collect();
        let height = lines.len() as i32;
        let width = lines[0].len() as i32;
        let mut nodes = vec![];
        let lines = lines.iter().enumerate();

        for (row_id, row) in lines {
            for (col_id, height) in row.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
                nodes.push(Node {
                    col: col_id as i32,
                    row: row_id as i32,
                    height: height as i32,
                });
            }
        }

        return Ok(TopoMap {
            nodes,
            height,
            width,
        });
    }
}

fn part1() {
    let grid_lines: String = read_lines("./day10/example")
        .unwrap()
        .collect::<Vec<_>>()
        .join("\n");
    let topo_map: TopoMap = grid_lines.parse().unwrap();
    let counts = topo_map.count_all_trails();
    println!("{:?}", counts);
    // for (node, count) in
    let num_trails: i32 = zip(topo_map.nodes, counts)
        .filter(|(n, _)| n.height == 0)
        .map(|(_, count)| count)
        .sum();
    println!("{}", num_trails);
}

fn part2() {
    let lines = read_lines("./day10/input").unwrap().collect::<Vec<_>>();
}
fn main() {
    part1();
    part2()
}
