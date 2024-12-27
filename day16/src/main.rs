use std::{
    collections::HashMap,
    fmt::{Display, Write},
    hash::Hash,
    str::FromStr,
};

use common::{read_lines, Edge, Graph, Graphable};

/**

###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
(0,0) is bottom-left
*/

fn part1() {
    /*
    // each node is represented by an index and a heading
    // (1,1,E) - 1 - (2,1,E)
    // (1,1,E) - 1000 - (1,1,N)
    // (1,1,E) - 1000 - (1,1,S)
     */

    #[derive(Debug, Hash, Clone, PartialEq, PartialOrd, Eq, Ord)]
    struct MazeNodeId((i32, i32, char));
    impl common::IdTrait for MazeNodeId {}

    impl Display for MazeNodeId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{},{},{}", self.0 .0, self.0 .1, self.0 .2))
        }
    }

    #[derive(Clone, Debug)]
    struct MazeNode {
        node_id: MazeNodeId,
        val: char,
    }

    impl common::Node for MazeNode {
        type NodeIdType = MazeNodeId;

        fn node_id(&self) -> Self::NodeIdType {
            return self.node_id.clone();
        }
    }

    struct Maze {
        start: MazeNodeId,
        end: Vec<MazeNodeId>, // there are multiple possible ways to reach the end node (based on the inbound direction)
        graph: Graph<MazeNode>,
    }

    #[derive(Debug)]
    struct MazeParseErr;

    impl FromStr for Maze {
        type Err = MazeParseErr;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut lines = s
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            lines.reverse(); // start from the bottom

            let mut nodes: <Graph<MazeNode> as Graphable>::Nodes = HashMap::new();
            let mut adjacency_list: <Graph<MazeNode> as Graphable>::AdjacencyList = HashMap::new();
            let mut start_node = MazeNodeId((0, 0, 'E'));
            let mut end_nodes = vec![];

            for (row, line) in lines.iter().enumerate() {
                for (col, ch) in line.iter().enumerate() {
                    if ch == &'#' {
                        continue;
                    }

                    // we found a start node
                    if ch == &'S' {
                        start_node = MazeNodeId((col as i32, row as i32, 'E'))
                    }

                    // we found end nodes
                    if ch == &'E' {
                        end_nodes = DIRECTIONS
                            .iter()
                            .map(|&d| MazeNodeId((col as i32, row as i32, d)))
                            .collect();
                    }

                    // the input has walls around the border, so can assume all neighbors are
                    let neighbors: Vec<_> = DELTAS
                        .iter()
                        .map(|d| {
                            lines[((row as i32) + d.1) as usize][((col as i32) + d.0) as usize]
                        })
                        .collect();
                    let node_edges = create_edges_for_node((col as i32, row as i32), neighbors);
                    for edge in node_edges {
                        let node = MazeNode {
                            node_id: edge.0.clone(),
                            val: *ch,
                        };

                        // add the source node to the nodes
                        nodes.insert(edge.0.clone(), node);

                        // add the edge
                        let edges_for_node = adjacency_list.entry(edge.0.clone()).or_default();
                        edges_for_node.push(Edge {
                            node: edge.1,
                            cost: edge.2 as usize,
                        });
                    }
                }
            }
            Ok(Maze {
                start: start_node,
                end: end_nodes,
                graph: Graph::new(nodes, adjacency_list),
            })
        }
    }

    const DELTAS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    const DIRECTIONS: [char; 4] = ['N', 'E', 'S', 'W'];

    fn create_edges_for_node(
        node: (i32, i32),
        neighbors: Vec<char>,
    ) -> Vec<(MazeNodeId, MazeNodeId, i32)> {
        // for each neighbor, check if it is reachable, if so, create an edge with the appropriate cost
        let mut edges = vec![];
        for (i, n) in neighbors.iter().enumerate() {
            // [NORTH,EAST, SOUTH, WEST]
            if *n != '#' {
                let edge = (
                    MazeNodeId((node.0, node.1, DIRECTIONS[i])),
                    MazeNodeId((node.0 + DELTAS[i].0, node.1 + DELTAS[i].1, DIRECTIONS[i])),
                    1,
                );
                edges.push(edge);
            }
        }

        // for each node and direction, add the 2 options to turn around
        for (i, dir) in DIRECTIONS.iter().enumerate() {
            // clockwise and counter-clockwise turn
            let start = MazeNodeId((node.0, node.1, *dir));
            let end_cw = MazeNodeId((node.0, node.1, DIRECTIONS[(i + 1) % 4]));
            let end_ccw = MazeNodeId((node.0, node.1, DIRECTIONS[(i + 3) % 4]));

            edges.push((start.clone(), end_cw, 1000));
            edges.push((start, end_ccw, 1000));
        }
        return edges;
    }

    let lines = read_lines("./day16/input").unwrap().collect::<Vec<_>>();
    let m: Maze = lines.join("\n").parse().unwrap();
    let res = m.graph.dijkstra(&m.start, Some(&m.end[0]));
    let path = common::get_path::<MazeNode>(&res, &m.end[0]);
    println!("{:?}", res.dist.get(&m.end[0]));
    println!("{:?}", res.dist.get(&m.end[1]));
    println!("{:?}", res.dist.get(&m.end[2]));
    println!("{:?}", res.dist.get(&m.end[3]));
}

fn part2() {
    let lines = read_lines("./day16/input").unwrap().collect::<Vec<_>>();
    let target = 107468_usize;
}
fn main() {
    part1();
    part2()
}
