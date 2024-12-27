use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
    hash::Hash,
    ops::Index,
    str::FromStr,
};

use common::{get_path, read_lines, Edge, Graph, Graphable};

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

    // from S to *all* nodes
    let dijkstra_s = m.graph.dijkstra(&m.start, None);
    let min_target = m
        .end
        .iter()
        .map(|n_id| (n_id, dijkstra_s.dist.get(n_id).unwrap()))
        .min_by(|x, y| x.1.cmp(y.1))
        .unwrap();

    let target_score = min_target.1;

    fn flip_direction(node_id: &MazeNodeId) -> MazeNodeId {
        let e = match node_id.0 .2 {
            'N' => 'S',
            'E' => 'W',
            'S' => 'N',
            'W' => 'E',
            _ => panic!("What direction was I heading??"),
        };
        MazeNodeId((node_id.0 .0, node_id.0 .1, e))
    }

    // need to swap directions since we're traveling backwards
    let flipped_target = flip_direction(&min_target.0);

    // from E to *all* nodes
    println!(
        "Going to look for paths to node {:?} {}",
        min_target.0, min_target.1
    );
    let dijkstra_e = m.graph.dijkstra(&flipped_target, None);

    // if a node is on the shortest-path, that means that the sum of the path from start to the node, and the path
    // from the node  to the end is equal to the target.
    // check which of the scores from E-N meet that condition
    // need to flip the direction of the nodes, since we're heading in opposite directions
    let matching_nodes = dijkstra_e
        .dist
        .keys()
        .map(|node_id| (node_id, flip_direction(&node_id)))
        .map(|(node_id, flipped_id)| {
            (
                node_id,
                dijkstra_s.dist.get(&flipped_id).unwrap(),
                dijkstra_e.dist.get(node_id).unwrap(),
            )
        })
        .filter(|(_, s_to_n, n_to_e)| *s_to_n + *n_to_e == *target_score)
        .map(|(node_id, _, _)| (node_id.0 .0, node_id.0 .1))
        .collect::<HashSet<_>>();

    println!("found {:?} nodes", matching_nodes.len());

    //
}
fn main() {
    //part1();
    part2()
}
