use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use common::read_lines;

fn part1() {
    let lines = read_lines("./day12/input")
        .unwrap()
        .collect::<Vec<_>>()
        .join("\n");

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    struct Point(i32, i32);

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    struct PlantPlot {
        location: Point,
        plant: char,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Region {
        plots: HashSet<Point>, // sorted such that the top-most plot is first
        plant: char,
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Gardens {
        plots: Vec<char>,
        width: i32,
    }

    #[derive(Debug)]
    struct ParseError;

    impl FromStr for Gardens {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let lines: Vec<_> = s.split('\n').collect();
            let width = lines.len() as i32;
            let plots = lines.concat().chars().collect();
            Ok(Self { plots, width })
        }
    }

    impl Gardens {
        fn get(&self, location: &Point) -> char {
            let idx = location.0 + location.1 * self.width;
            return self.plots[idx as usize];
        }

        fn get_neighbor_plant_plots(&self, location: &Point) -> VecDeque<PlantPlot> {
            let mut neighbors = vec![];
            if (location.0 > 0) {
                neighbors.push(Point(location.0 - 1, location.1));
            }

            if (location.0 < self.width - 1) {
                neighbors.push(Point(location.0 + 1, location.1));
            }

            if (location.1 > 0) {
                neighbors.push(Point(location.0, location.1 - 1));
            }

            if (location.1 < self.width - 1) {
                neighbors.push(Point(location.0, location.1 + 1));
            }

            return neighbors
                .iter()
                .map(|p| PlantPlot {
                    location: *p,
                    plant: self.get(p),
                })
                .collect();
        }

        fn get_region(&self, location: &Point) -> Region {
            let plant = self.get(&location);
            let mut plot = HashSet::<Point>::new();

            let mut candidates = HashSet::new();

            // add starting node to the candidate list
            candidates.insert(PlantPlot {
                location: *location,
                plant,
            });

            while !candidates.is_empty() {
                // pop the next neighbor

                let neighbor = candidates.iter().next().cloned().unwrap();
                candidates.remove(&neighbor);
                plot.insert(neighbor.location);

                // find its neighbors, and add them to the candidate list
                let neighbor_locations = self
                    .get_neighbor_plant_plots(&neighbor.location)
                    .into_iter()
                    .filter(|x| x.plant == plant && !plot.contains(&x.location));
                candidates.extend(neighbor_locations);
            }

            return Region {
                plant: plant,
                plots: plot,
            };
        }
    }

    impl Region {
        fn area(&self) -> usize {
            return self.plots.len();
        }

        fn perimeter(&self, garden: &Gardens) -> usize {
            return self.num_neighbors_out_of_region(garden);
        }

        fn num_neighbors_out_of_region(&self, garden: &Gardens) -> usize {
            let mut num_neighbors_out = 0;
            for p in self.plots.iter() {
                let neighbors = garden.get_neighbor_plant_plots(p);
                num_neighbors_out += 4 - neighbors.len();
                num_neighbors_out += neighbors
                    .iter()
                    .filter(|x| x.plant != self.plant)
                    .collect::<Vec<_>>()
                    .len();
            }

            return num_neighbors_out;
        }
    }
    let garden: Gardens = lines.parse().unwrap();
    let mut nodes_in_regions: HashSet<Point> = HashSet::new();
    let mut total_price = 0;
    for col in 0..garden.width {
        for row in 0..garden.width {
            let p = Point(col, row);
            if !nodes_in_regions.contains(&p) {
                let region = garden.get_region(&p);
                total_price += region.perimeter(&garden) * region.area();
                for n in region.plots {
                    nodes_in_regions.insert(n);
                }
            }
        }
    }
    println!("total price {}", total_price);
}

fn part2() {
    let lines = read_lines("./day12/input")
        .unwrap()
        .collect::<Vec<_>>()
        .join("\n");

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    struct Point(i32, i32);

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    struct PlantPlot {
        location: Point,
        plant: char,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Region {
        plots: HashSet<Point>, // sorted such that the top-most plot is first
        plant: char,
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Gardens {
        plots: Vec<char>,
        width: i32,
    }

    #[derive(Debug)]
    struct ParseError;

    impl FromStr for Gardens {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let lines: Vec<_> = s.split('\n').collect();
            let width = lines.len() as i32;
            let plots = lines.concat().chars().collect();
            Ok(Self { plots, width })
        }
    }

    type NeighborMap = HashSet<(i32, i32)>;
    const DELTAS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    impl Gardens {
        fn get(&self, location: &Point) -> char {
            let idx = location.0 + location.1 * self.width;
            return self.plots[idx as usize];
        }

        fn get_neighbor_plant_plots(&self, location: &Point) -> VecDeque<PlantPlot> {
            let mut neighbors = vec![];
            if (location.0 > 0) {
                neighbors.push(Point(location.0 - 1, location.1));
            }

            if (location.0 < self.width - 1) {
                neighbors.push(Point(location.0 + 1, location.1));
            }

            if (location.1 > 0) {
                neighbors.push(Point(location.0, location.1 - 1));
            }

            if (location.1 < self.width - 1) {
                neighbors.push(Point(location.0, location.1 + 1));
            }

            return neighbors
                .iter()
                .map(|p| PlantPlot {
                    location: *p,
                    plant: self.get(p),
                })
                .collect();
        }

        fn get_neighbors_in_region(&self, location: &Point) -> NeighborMap {
            let mut neighbors = NeighborMap::new();
            for delta in DELTAS {
                let loc_x = location.0 + delta.0;
                let loc_y = location.1 + delta.1;

                if loc_x >= 0 && loc_x < self.width && loc_y >= 0 && loc_y < self.width {
                    let np = Point(loc_x, loc_y);
                    if self.get(&np) == self.get(&location) {
                        neighbors.insert(delta);
                    }
                }
            }
            return neighbors;
        }

        fn get_region(&self, location: &Point) -> Region {
            let plant = self.get(&location);
            let mut plot = HashSet::<Point>::new();

            let mut candidates = HashSet::new();

            // add starting node to the candidate list
            candidates.insert(PlantPlot {
                location: *location,
                plant,
            });

            while !candidates.is_empty() {
                // pop the next neighbor

                let neighbor = candidates.iter().next().cloned().unwrap();
                candidates.remove(&neighbor);
                plot.insert(neighbor.location);

                // find its neighbors, and add them to the candidate list
                let neighbor_locations = self
                    .get_neighbor_plant_plots(&neighbor.location)
                    .into_iter()
                    .filter(|x| x.plant == plant && !plot.contains(&x.location));
                candidates.extend(neighbor_locations);
            }

            return Region {
                plant: plant,
                plots: plot,
            };
        }
    }

    impl Region {
        fn area(&self) -> usize {
            return self.plots.len();
        }

        fn perimeter(&self, garden: &Gardens) -> usize {
            return self.num_sides(garden);
        }

        fn num_sides(&self, garden: &Gardens) -> usize {
            let mut num_corners = 0;
            for p in self.plots.iter() {
                let neighbors = garden.get_neighbors_in_region(p);
                if !neighbors.contains(&(0, -1)) && !neighbors.contains(&(-1, 0)) {
                    // U:-, L-
                    num_corners += 1;
                }

                if !neighbors.contains(&(0, -1)) && !neighbors.contains(&(1, 0)) {
                    // U:-, R-
                    num_corners += 1;
                }

                if !neighbors.contains(&(0, 1)) && !neighbors.contains(&(-1, 0)) {
                    // D:-, L-
                    num_corners += 1;
                }

                if !neighbors.contains(&(0, 1)) && !neighbors.contains(&(1, 0)) {
                    // D:-, R-
                    num_corners += 1;
                }

                // D:+, R+, DR-
                if neighbors.contains(&(0, 1))
                    && neighbors.contains(&(1, 0))
                    && !neighbors.contains(&(1, 1))
                {
                    num_corners += 1;
                }

                // D:+, L+, DL-
                if neighbors.contains(&(0, 1))
                    && neighbors.contains(&(-1, 0))
                    && !neighbors.contains(&(-1, 1))
                {
                    num_corners += 1;
                }

                // U:+, R+, UR-
                if neighbors.contains(&(0, -1))
                    && neighbors.contains(&(1, 0))
                    && !neighbors.contains(&(1, -1))
                {
                    num_corners += 1;
                }

                // U:+, L+, UL-
                if neighbors.contains(&(0, -1))
                    && neighbors.contains(&(-1, 0))
                    && !neighbors.contains(&(-1, -1))
                {
                    num_corners += 1;
                }
            }

            return num_corners;
        }
    }
    let garden: Gardens = lines.parse().unwrap();
    let mut nodes_in_regions: HashSet<Point> = HashSet::new();
    let mut total_price = 0;
    for col in 0..garden.width {
        for row in 0..garden.width {
            let p = Point(col, row);
            if !nodes_in_regions.contains(&p) {
                let region = garden.get_region(&p);
                total_price += region.perimeter(&garden) * region.area();
                for n in region.plots {
                    nodes_in_regions.insert(n);
                }
            }
        }
    }
    println!("total price {}", total_price);
}
fn main() {
    part1();
    part2()
}
