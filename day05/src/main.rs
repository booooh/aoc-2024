use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

use common::read_lines;

#[derive(Debug, Clone, Copy)]
struct Edge<NodeID>(NodeID, NodeID);

fn topological_sort<NodeID>(edges: &Vec<Edge<NodeID>>) -> Vec<NodeID>
where
    NodeID: Hash + Eq + Copy + Debug,
{
    let mut sorted_list = Vec::new(); // the return value
    let mut edges_by_source = HashMap::<NodeID, HashSet<NodeID>>::new(); // map from source -> all detinations
    let mut edges_by_dest = HashMap::<NodeID, HashSet<NodeID>>::new();

    /*
       L ← Empty list that will contain the sorted elements
       S ← Set of all nodes with no incoming edge

       while S is not empty do
           remove a node n from S
           add n to L
           for each node m with an edge e from n to m do
               remove edge e from the graph
               if m has no other incoming edges then
                   insert m into S

       if graph has edges then
           return error   (graph has at least one cycle)
       else
           return L   (a topologically sorted order)
    */
    for edge in edges {
        (*edges_by_source.entry(edge.0).or_insert(HashSet::default())).insert(edge.1);
        edges_by_source.entry(edge.1).or_insert(HashSet::default());
        (*edges_by_dest.entry(edge.1).or_insert(HashSet::default())).insert(edge.0);
        edges_by_dest.entry(edge.0).or_insert(HashSet::default());
    }

    let mut no_incoming_edges_q = edges_by_dest
        .iter()
        .filter(|(_, edges)| edges.len() == 0)
        .map(|(&dest, _)| dest)
        .collect::<VecDeque<_>>();

    // println!("{:?}", edges_by_dest.keys());

    if no_incoming_edges_q.is_empty() {
        panic!("Could not find nodes without incoming edges");
    }

    while let Some(node_n) = no_incoming_edges_q.pop_back() {
        sorted_list.push(node_n);

        let dest_nodes = edges_by_source.remove(&node_n).unwrap();
        for node_m in dest_nodes {
            // remove the edge (n,m) from the map n
            let edges_to_m = edges_by_dest.get_mut(&node_m).unwrap();
            edges_to_m.remove(&node_n);

            // if this was the last incoming edge for node_m, add it to the set of no incoming edges
            if edges_to_m.is_empty() {
                no_incoming_edges_q.push_front(node_m);
                edges_by_dest.remove(&node_m);
            }
        }
    }

    if edges_by_source.len() > 0 {
        panic!("There were still edges in the graph!");
    }

    return sorted_list;
}

fn part1() {
    let day_input = read_lines("./day05/input").unwrap().collect::<Vec<_>>();
    let page_ordering_rules = day_input
        .iter()
        .take_while(|&line| line.len() > 0)
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| Edge(a.parse::<i32>().unwrap(), b.parse().unwrap()))
        .collect::<Vec<_>>();

    // println!("{:?}", page_ordering_rules);

    let updates = day_input
        .iter()
        .skip_while(|line| line.len() > 0)
        .skip(1)
        .map(|s| {
            s.split(",")
                .map(|p| p.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn is_correct_order(update: &Vec<i32>, ordering: &Vec<i32>) -> bool {
        let applicable_pages = update
            .iter()
            .filter(|page| ordering.contains(page))
            .map(|p| *p)
            .collect::<Vec<_>>();
        // println!(
        //     "{:?}, {:?} == {}",
        //     update,
        //     ordering,
        //     &applicable_pages == ordering
        // );
        return &applicable_pages == ordering;
    }

    let mut res = 0i32;
    for update in updates {
        let applicable_edges = page_ordering_rules
            .clone()
            .into_iter()
            .filter(|e| update.contains(&e.0) && update.contains(&e.1))
            .collect::<Vec<Edge<i32>>>();
        let page_order = topological_sort(&applicable_edges);
        if is_correct_order(&update, &page_order) {
            res += update[update.len() / 2];
        }
    }
    println!("{:?}", res);
}

fn part2() {
    let day_input = read_lines("./day05/input").unwrap().collect::<Vec<_>>();
    let page_ordering_rules = day_input
        .iter()
        .take_while(|&line| line.len() > 0)
        .map(|line| line.split_once("|").unwrap())
        .map(|(a, b)| Edge(a.parse::<i32>().unwrap(), b.parse().unwrap()))
        .collect::<Vec<_>>();

    // println!("{:?}", page_ordering_rules);

    let updates = day_input
        .iter()
        .skip_while(|line| line.len() > 0)
        .skip(1)
        .map(|s| {
            s.split(",")
                .map(|p| p.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    fn is_correct_order(update: &Vec<i32>, ordering: &Vec<i32>) -> bool {
        let applicable_pages = update
            .iter()
            .filter(|page| ordering.contains(page))
            .map(|p| *p)
            .collect::<Vec<_>>();
        return &applicable_pages == ordering;
    }

    let mut res = 0i32;
    for update in updates {
        let applicable_edges = page_ordering_rules
            .clone()
            .into_iter()
            .filter(|e| update.contains(&e.0) && update.contains(&e.1))
            .collect::<Vec<Edge<i32>>>();
        let page_order = topological_sort(&applicable_edges);
        if !is_correct_order(&update, &page_order) {
            res += page_order[page_order.len() / 2];
        }
    }
    println!("{:?}", res);
}

fn main() {
    part1();
    part2();
}
