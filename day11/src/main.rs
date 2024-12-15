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
        return None;
    }

    fn rule_3(stones: &Vec<i64>, idx: usize) -> Vec<i64> {
        return vec![stones[idx] * 2024];
    }

    fn blink_stone(stones: &Vec<i64>, idx: usize) -> Vec<i64> {
        if let Some(stone) = rule_1(stones, idx) {
            return stone;
        } else if let Some(stone) = rule_2(stones, idx) {
            return stone;
        } else {
            return rule_3(stones, idx);
        }
    }

    fn bink(stones: &Vec<i64>) -> Vec<i64> {
        return (0..stones.len())
            .map(|idx| blink_stone(stones, idx))
            .flatten()
            .collect();
    }

    for b in 0..25 {
        stones = bink(&stones);
        println!("stones length after {} blinks = {}", b, stones.len());
    }
}

fn part2() {
    let lines = read_lines("./day11/input").unwrap().collect::<Vec<_>>();
}
fn main() {
    part1();
    part2()
}
