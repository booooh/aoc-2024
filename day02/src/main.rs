use common::read_lines;

fn part1() {
    let reports = read_lines("./day02/input").unwrap().map(|x| {
        x.split_whitespace()
            .map(|s| s.to_owned().parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    fn is_safe(report: &Vec<i32>) -> bool {
        let mut deltas = report.windows(2).map(|x| x[1] - x[0]);
        let all_gradual_increase = deltas.clone().all(|x| x >= 1 && x <= 3);
        let all_gradual_decrease = deltas.all(|x| x <= -1 && x >= -3);

        return all_gradual_decrease || all_gradual_increase;
    }
    let safe_reports = reports.filter(|r| is_safe(r)).collect::<Vec<_>>();
    println!("{:?}", safe_reports.len());
}

fn part2() {}

fn main() {
    part1();
    part2();
}
