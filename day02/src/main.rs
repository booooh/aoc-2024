use common::read_lines;
use itertools::Itertools;

fn part1() {
    let reports = read_lines("./day02/input").unwrap().map(|x| {
        x.split_whitespace()
            .map(|s| s.to_owned().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    fn is_safe(report: &Vec<i32>) -> bool {
        let mut deltas = report.windows(2).map(|x| x[1] - x[0]);
        let all_gradual_increase = deltas.clone().all(|x| (1..=3).contains(&x));
        let all_gradual_decrease = deltas.all(|x| (-3..=-1).contains(&x));

        all_gradual_decrease || all_gradual_increase
    }
    let safe_reports = reports.filter(is_safe).collect::<Vec<_>>();
    println!("{:?}", safe_reports.len());
}

fn part2() {
    let reports = read_lines("./day02/input").unwrap().map(|x| {
        x.split_whitespace()
            .map(|s| s.to_owned().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    let dampened_reports = reports.map(|r| {
        let num_elems = r.len() - 1;
        r.into_iter().combinations(num_elems).collect::<Vec<_>>()
    });
    fn is_safe(report: &Vec<i32>) -> bool {
        let mut deltas = report.windows(2).map(|x| x[1] - x[0]);
        let all_gradual_increase = deltas.clone().all(|x| (1..=3).contains(&x));
        let all_gradual_decrease = deltas.all(|x| (-3..=-1).contains(&x));

        all_gradual_decrease || all_gradual_increase
    }
    fn is_dampened_safe(dampened_report: &Vec<Vec<i32>>) -> bool {
        dampened_report.iter().any(is_safe)
    }
    let safe_reports = dampened_reports
        .filter(is_dampened_safe)
        .collect::<Vec<_>>();
    println!("{:?}", safe_reports.len());
}

fn main() {
    part1();
    part2();
}
