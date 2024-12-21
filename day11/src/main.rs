use std::collections::HashMap;

use common::read_lines;

fn part1() {
    let lines = read_lines("./day11/input").unwrap().collect::<Vec<_>>();
    let mut stones: Vec<i64> = lines[0].split(" ").map(|s| s.parse().unwrap()).collect();

    fn rule_1(stones: &Vec<i64>, idx: usize) -> Option<Vec<i64>> {
        if stones[idx] == 0 {
            return Some(vec![1]);
        }
        None
    }

    fn rule_2(stones: &Vec<i64>, idx: usize) -> Option<Vec<i64>> {
        let curr_val = stones[idx];
        let num_digits = curr_val.ilog10() + 1;
        if num_digits % 2 == 0 {
            let factor = 10_u32.pow(num_digits / 2) as i64;
            return Some(vec![curr_val / factor, curr_val % factor]);
        }
        None
    }

    fn rule_3(stones: &Vec<i64>, idx: usize) -> Vec<i64> {
        vec![stones[idx] * 2024]
    }

    fn blink_stone(stones: &Vec<i64>, idx: usize) -> Vec<i64> {
        if let Some(stone) = rule_1(stones, idx) {
            stone
        } else if let Some(stone) = rule_2(stones, idx) {
            return stone;
        } else {
            return rule_3(stones, idx);
        }
    }

    fn blink(stones: &Vec<i64>) -> Vec<i64> {
        (0..stones.len())
            .flat_map(|idx| blink_stone(stones, idx))
            .collect()
    }
    for b in 0..25 {
        stones = blink(&stones);
        println!("stones length after {} blinks = {}", b, stones.len());
    }
}

fn part2() {
    #[derive(Debug, PartialEq, Eq, Clone)]
    struct Stone {
        orig_id: i64,
        value: i64,
    }

    let lines = read_lines("./day11/input").unwrap().collect::<Vec<_>>();
    let stones: Vec<Stone> = lines[0]
        .split(" ")
        .map(|s| s.parse().unwrap())
        .map(|value| Stone {
            value,
            orig_id: value,
        })
        .collect();

    fn rule_1(stone: &Stone) -> Option<Vec<Stone>> {
        if stone.value == 0 {
            return Some(vec![Stone {
                orig_id: stone.value,
                value: 1,
            }]);
        }
        None
    }

    fn rule_2(stone: &Stone) -> Option<Vec<Stone>> {
        let curr_val = stone.value;
        let num_digits = curr_val.ilog10() + 1;
        if num_digits % 2 == 0 {
            let factor = 10_u32.pow(num_digits / 2) as i64;
            return Some(vec![
                Stone {
                    value: curr_val / factor,
                    orig_id: curr_val,
                },
                Stone {
                    value: curr_val % factor,
                    orig_id: curr_val,
                },
            ]);
        }
        None
    }

    fn rule_3(stone: &Stone) -> Vec<Stone> {
        vec![Stone {
            value: stone.value * 2024,
            orig_id: stone.value,
        }]
    }

    fn blink_stone(stone: &Stone) -> Vec<Stone> {
        if let Some(stone) = rule_1(stone) {
            stone
        } else if let Some(stone) = rule_2(stone) {
            return stone;
        } else {
            return rule_3(stone);
        }
    }
    type Cache = HashMap<(i64, usize), usize>;

    fn blink_num_iter(stone: &Stone, num_blinks: usize, mut cache: Cache) -> Cache {
        if cache.contains_key(&(stone.value, num_blinks)) {
            return cache;
        }
        let new_stones = blink_stone(stone);

        if num_blinks == 1 {
            cache.insert((stone.value, num_blinks), new_stones.len());
            return cache;
        }
        let mut total = 0_usize;
        for s in new_stones {
            // store any sub-parts in the cache
            cache = blink_num_iter(&s, num_blinks - 1, cache);
            let count = cache.get(&(s.value, num_blinks - 1)).unwrap();
            total += count;
        }
        cache.insert((stone.value, num_blinks), total);
        // println!("{}: done with iter {}", stone.value, num_blinks);

        cache

        // since it wasn't in the cache previously, add it
    }

    // hash from (orig stone, num blinks) -> number of stones
    let mut cache = Cache::new();

    // for stone_id in 0..100 {
    //     let mut stones_for_cache = vec![Stone {
    //         value: stone_id,
    //         orig_id: stone_id,
    //     }];
    //     for b in 0..50_i64 {
    //         stones_for_cache = blink(&stones_for_cache);
    //         cache
    //             .entry((stone_id, b + 1))
    //             .or_insert(stones_for_cache.len());
    //     }
    // }

    let mut part2_total = 0_usize;
    for stone in &stones {
        cache = blink_num_iter(stone, 75, cache);
        let tmp = cache.get(&(stone.value, 75)).unwrap();
        part2_total += *tmp;
    }

    // collect all the items in the cache with 38 blinks

    println!("{:?}", cache.len());
    println!("{:?}", part2_total);
}
fn main() {
    part1();
    part2()
}
