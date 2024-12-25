use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let strings = io::BufReader::new(file).lines().map(|x| x.unwrap());
    Ok(strings)
}

pub trait IdTrait: Clone + Debug + Ord + PartialOrd + Eq + PartialEq + Hash {}

pub trait Node: Debug + Clone {
    type NodeIdType: IdTrait;

    fn node_id(&self) -> Self::NodeIdType;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Edge<T>
where
    T: Node,
{
    node: T::NodeIdType,
    cost: usize, // must be positive
}

pub trait Graphable {
    type AdjacencyList;
    type Nodes;
    type Edges;
}

#[derive(Debug, Clone)]
pub struct Graph<T>
where
    T: Node,
{
    nodes: <Self as Graphable>::Nodes,
    adjacency_list: <Self as Graphable>::AdjacencyList,
}

impl<T> Graphable for Graph<T>
where
    T: Node,
{
    type AdjacencyList = HashMap<T::NodeIdType, Self::Edges>;
    type Nodes = HashMap<T::NodeIdType, T>;
    type Edges = Vec<Edge<T>>;
}

#[derive(Debug)]
pub struct Dijkstra<NodeId>
where
    NodeId: IdTrait,
{
    dist: HashMap<NodeId, usize>,
    prev: HashMap<NodeId, Option<NodeId>>,
}

impl<T> Graph<T>
where
    T: Node,
{
    pub fn new(
        nodes: <Graph<T> as Graphable>::Nodes,
        adjacency_list: <Graph<T> as Graphable>::AdjacencyList,
    ) -> Self {
        Self {
            nodes,
            adjacency_list,
        }
    }

    pub fn dijkstra(
        &self,
        start: &T::NodeIdType,
        end: Option<&T::NodeIdType>,
    ) -> Dijkstra<T::NodeIdType> {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct QueueEntry<N>
        where
            N: IdTrait,
        {
            distance: Reverse<usize>,
            node_id: N,
        }

        let mut d_res: Dijkstra<T::NodeIdType> = Dijkstra {
            dist: HashMap::new(),
            prev: HashMap::new(),
        };

        let mut q = BinaryHeap::<QueueEntry<T::NodeIdType>>::new();

        // initialize the internal data structures
        for v in self.nodes.keys() {
            d_res.prev.insert(v.clone(), None);
            d_res.dist.insert(v.clone(), usize::MAX);
        }

        d_res.dist.insert(start.clone(), 0);
        q.push(QueueEntry {
            node_id: start.clone(),
            distance: Reverse(0),
        });

        while let Some(QueueEntry { distance, node_id }) = q.pop() {
            let u_node_id = node_id;

            if end.is_some_and(|end_node| *end_node == u_node_id) {
                return d_res;
            }

            let dist_u = d_res.dist.get(&u_node_id).unwrap().clone();

            if distance.0 > dist_u {
                continue;
            }

            for e in self.adjacency_list.get(&u_node_id).unwrap() {
                let alt = dist_u + e.cost;
                let v_node = e.node.clone();
                if alt < *d_res.dist.get(&v_node).unwrap() {
                    d_res.prev.entry(v_node.clone()).and_modify(|x| {
                        x.replace(u_node_id.clone());
                    });

                    d_res.dist.entry(v_node.clone()).and_modify(|x| {
                        *x = alt;
                    });
                    q.push(QueueEntry {
                        node_id: v_node,
                        distance: Reverse(alt),
                    });
                }
            }
        }

        return d_res;
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fmt::Display};

    use super::*;

    #[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
    struct TestNodeId(usize);

    impl Display for TestNodeId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{}", self.0))
        }
    }

    impl IdTrait for TestNodeId {}

    #[derive(Debug, Clone)]
    struct TestNode {
        id: TestNodeId,
    }

    impl Node for TestNode {
        type NodeIdType = TestNodeId;

        fn node_id(&self) -> Self::NodeIdType {
            self.id.clone()
        }
    }

    fn create_test_graph(edges: Vec<(usize, usize, usize)>) -> Graph<TestNode> {
        let mut nodes = HashMap::new();
        let mut adjacency_list: HashMap<TestNodeId, Vec<Edge<TestNode>>> = HashMap::new();

        // Collect all unique node IDs
        let mut node_ids: Vec<usize> = edges
            .iter()
            .flat_map(|(from, to, _)| vec![*from, *to])
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        node_ids.sort();

        // Create nodes
        for id in node_ids {
            let node_id = TestNodeId(id);
            nodes.insert(node_id.clone(), TestNode {
                id: node_id.clone(),
            });
            adjacency_list.insert(node_id, Vec::new());
        }

        // Add edges
        for (from, to, cost) in edges {
            let from_id = TestNodeId(from);
            let to_id = TestNodeId(to);
            adjacency_list
                .get_mut(&from_id)
                .unwrap()
                .push(Edge { node: to_id, cost });
        }

        Graph::new(nodes, adjacency_list)
    }

    fn get_path(dijkstra: &Dijkstra<TestNodeId>, end: &TestNodeId) -> Vec<TestNodeId> {
        let mut path = Vec::new();
        let mut current = Some(end.clone());

        while let Some(node) = current {
            path.push(node.clone());
            current = dijkstra.prev.get(&node).unwrap().clone();
        }

        path.reverse();
        path
    }

    #[test]
    fn test_simple_path() {
        let graph = create_test_graph(vec![(0, 1, 4), (1, 2, 3), (0, 2, 8)]);

        let start = TestNodeId(0);
        let end = TestNodeId(2);
        let result = graph.dijkstra(&start, Some(&end));

        assert_eq!(*result.dist.get(&end).unwrap(), 7); // 0->1->2 (4+3)
        let path = get_path(&result, &end);
        assert_eq!(path, vec![TestNodeId(0), TestNodeId(1), TestNodeId(2)]);
    }

    #[test]
    fn test_disconnected_graph() {
        let graph = create_test_graph(vec![
            (0, 1, 4),
            (2, 3, 5), // Disconnected component
        ]);

        let start = TestNodeId(0);
        let end = TestNodeId(3);
        let result = graph.dijkstra(&start, Some(&end));

        assert_eq!(*result.dist.get(&end).unwrap(), usize::MAX);
    }

    #[test]
    fn test_single_node() {
        let graph = create_test_graph(vec![(0, 0, 100)]);
        let node = TestNodeId(0);
        let result = graph.dijkstra(&node, Some(&node));

        assert_eq!(*result.dist.get(&node).unwrap(), 0);
    }

    #[test]
    fn test_cycle() {
        let graph = create_test_graph(vec![(0, 1, 1), (1, 2, 2), (2, 0, 3)]);

        let start = TestNodeId(0);
        let end = TestNodeId(2);
        let result = graph.dijkstra(&start, Some(&end));

        assert_eq!(*result.dist.get(&end).unwrap(), 3); // 0->1->2 (1+2)
    }

    #[test]
    fn test_multiple_paths() {
        let graph = create_test_graph(vec![(0, 1, 2), (1, 3, 2), (0, 2, 3), (2, 3, 1)]);
        println!("{}", visualize_graph(&graph));

        let start = TestNodeId(0);
        let end = TestNodeId(3);
        let result = graph.dijkstra(&start, Some(&end));

        assert_eq!(*result.dist.get(&end).unwrap(), 4); // 0->1->3 (2+2)
        let path = get_path(&result, &end);
        assert_eq!(
            HashSet::<TestNodeId>::from_iter(path),
            HashSet::from_iter(vec![TestNodeId(0), TestNodeId(1), TestNodeId(3)])
        );
    }

    #[test]
    fn test_self_loop() {
        let graph = create_test_graph(vec![(0, 0, 1), (0, 1, 2)]);

        let start = TestNodeId(0);
        let end = TestNodeId(1);
        let result = graph.dijkstra(&start, Some(&end));

        assert_eq!(*result.dist.get(&end).unwrap(), 2);
    }

    pub fn visualize_graph<T>(graph: &Graph<T>) -> String
    where
        T: Node,
        T::NodeIdType: std::fmt::Display,
    {
        let mut mermaid = String::from("```mermaid\ngraph LR\n");

        // Add all edges with their weights
        for (from_id, edges) in &graph.adjacency_list {
            for edge in edges {
                // Use Mermaid's edge label syntax: A---|text|B
                mermaid.push_str(&format!(
                    "    {from_id}---|{edge_cost}|{to_id}\n",
                    from_id = from_id,
                    edge_cost = edge.cost,
                    to_id = edge.node
                ));
            }
        }

        // Add any isolated nodes (nodes with no edges)
        for node_id in graph.nodes.keys() {
            if !graph.adjacency_list.contains_key(node_id) {
                mermaid.push_str(&format!("    {node_id}((Node {node_id}))\n"));
            }
        }

        mermaid.push_str("```");
        mermaid
    }

    #[test]
    fn test_complex_path_selection() {
        // Create a more complex graph with multiple possible paths
        //
        //     1 --2--> 3
        //    /         |  \
        // 0 --3--> 2  1   4
        //    \     |  |  /
        //     2--> 5 -1->
        //
        let graph = create_test_graph(vec![
            (0, 1, 1), // Start -> Upper path
            (1, 3, 2),
            (3, 4, 1),
            (0, 2, 3), // Start -> Middle path
            (2, 4, 2),
            (0, 5, 2), // Start -> Lower path
            (5, 4, 1),
            (2, 5, 1), // Cross connection
            (3, 4, 1), // Alternative from 3
            (2, 6, 2), // add another node for testing
            (4, 7, 5), // add another node to check stopping early
        ]);

        println!("{}", visualize_graph(&graph));

        let start = TestNodeId(0);
        let end = TestNodeId(4);
        let result = graph.dijkstra(&start, None);

        // Multiple paths exist to node 4:
        // 1. 0->1->3->4 (cost: 1+2+1 = 4)
        // 2. 0->2->4   (cost: 3+2 = 5)
        // 3. 0->5->4   (cost: 2+1 = 3)
        // 4. 0->2->5->4 (cost: 3+1+1 = 5)

        // Verify shortest distance
        assert_eq!(*result.dist.get(&end).unwrap(), 3);

        // Your implementation should choose 0->5->4 because:
        // 1. From node 0:
        //    - Edge to 1 costs 1
        //    - Edge to 2 costs 3
        //    - Edge to 5 costs 2
        // 2. After visiting node 1, the next shortest edge is to 5 (cost 2)
        // 3. From 5, the edge to 4 costs 1
        let path = get_path(&result, &end);
        assert_eq!(path, vec![TestNodeId(0), TestNodeId(5), TestNodeId(4)]);

        // check other  paths as well
        let path_to_3 = get_path(&result, &TestNodeId(3));
        assert_eq!(path_to_3, vec![TestNodeId(0), TestNodeId(1), TestNodeId(3)]);

        let path_to_6 = get_path(&result, &TestNodeId(6));
        assert_eq!(path_to_6, vec![TestNodeId(0), TestNodeId(2), TestNodeId(6)]);

        let path_to_7 = get_path(&result, &TestNodeId(7));
        assert_eq!(path_to_7, vec![
            TestNodeId(0),
            TestNodeId(5),
            TestNodeId(4),
            TestNodeId(7)
        ]);
    }
}
