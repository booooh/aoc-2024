use std::collections::HashSet;

use common::read_lines;

fn part1() {
    let word_search_rows = read_lines("./day04/input").unwrap().collect::<Vec<_>>();
    //let word_search_rows = read_lines("./day04/example").unwrap().collect::<Vec<_>>();

    let num_rows = word_search_rows.len() as i32;
    let width = word_search_rows[0].len() as i32;
    let word_search = word_search_rows
        .into_iter()
        .collect::<String>()
        .chars()
        .collect::<Vec<_>>();
    let word_to_find = "XMAS".chars().collect::<Vec<_>>();
    let word_len = word_to_find.len() as i32;
    let first_letter = word_to_find[0];
    let directions = vec![
        (0, -1),
        (0, 1),
        (1, 0),
        (1, -1),
        (1, 1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
    ];

    fn is_applicable(
        step: &(i32, i32),
        index: (i32, i32),
        word_len: i32,
        height: i32,
        width: i32,
    ) -> bool {
        let final_x = index.0 + (step.0 * (word_len - 1));
        let final_y = index.1 + (step.1 * (word_len - 1));

        return final_x < width && final_y < height && final_x >= 0 && final_y >= 0;
    }

    fn word_chars(
        word_search: &Vec<char>,
        step: &(i32, i32),
        index: &(i32, i32),
        word_len: i32,
        width: i32,
    ) -> Vec<char> {
        let mut result = vec![];
        for i in 0..word_len {
            let char_col = index.0 + (step.0 * i);
            let char_row = index.1 + (step.1 * i);
            let next_index = (char_col + (char_row * width)) as usize;
            result.push(word_search[next_index]);
        }

        result
    }

    let mut num_matches = 0;
    for row in 0..num_rows {
        for col in 0..width {
            let idx = (row * width + col) as usize;
            let letter = word_search[idx];
            if letter != first_letter {
                continue;
            }

            // this matches the first letter, check if we can match the whole word
            for dir in directions.iter() {
                if is_applicable(dir, (col, row), word_len, num_rows, width) {
                    let candidate = word_chars(&word_search, dir, &(col, row), word_len, width);
                    if candidate == word_to_find {
                        num_matches += 1;
                    }
                }
            }
        }
    }
    println!("{:?}", num_matches);
}

fn part2() {
    let word_search_rows = read_lines("./day04/input").unwrap().collect::<Vec<_>>();
    //let word_search_rows = read_lines("./day04/example").unwrap().collect::<Vec<_>>();

    let num_rows = word_search_rows.len() as i32;
    let width = word_search_rows[0].len() as i32;
    let word_search = word_search_rows
        .into_iter()
        .collect::<String>()
        .chars()
        .collect::<Vec<_>>();
    let word_to_find = "MAS".chars().collect::<Vec<_>>();
    let word_len = word_to_find.len() as i32;
    let first_letter = word_to_find[0];
    let directions = vec![(1, -1), (1, 1), (-1, 1), (-1, -1)];

    fn is_applicable(
        step: &(i32, i32),
        index: (i32, i32),
        word_len: i32,
        height: i32,
        width: i32,
    ) -> bool {
        let final_x = index.0 + (step.0 * (word_len - 1));
        let final_y = index.1 + (step.1 * (word_len - 1));

        return final_x < width && final_y < height && final_x >= 0 && final_y >= 0;
    }

    fn word_chars(
        word_search: &Vec<char>,
        step: &(i32, i32),
        index: &(i32, i32),
        word_len: i32,
        width: i32,
    ) -> Vec<char> {
        let mut result = vec![];
        for i in 0..word_len {
            let char_col = index.0 + (step.0 * i);
            let char_row = index.1 + (step.1 * i);
            let next_index = (char_col + (char_row * width)) as usize;
            result.push(word_search[next_index]);
        }

        result
    }

    let mut a_locations = HashSet::new();
    let mut num_matches = 0;
    for row in 0..num_rows {
        for col in 0..width {
            let idx = (row * width + col) as usize;
            let letter = word_search[idx];
            if letter != first_letter {
                continue;
            }

            // this matches the first letter, check if we can match the whole word
            for dir in directions.iter() {
                if is_applicable(dir, (col, row), word_len, num_rows, width) {
                    let candidate = word_chars(&word_search, dir, &(col, row), word_len, width);
                    if candidate == word_to_find {
                        let a_loc = (col + dir.0, row + dir.1);
                        if a_locations.contains(&a_loc) {
                            num_matches += 1;
                        } else {
                            a_locations.insert(a_loc);
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", num_matches);
}

fn main() {
    part1();
    part2();
}
