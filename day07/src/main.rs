use itertools::Itertools;
use std::str::FromStr;

use common::read_lines;

type Op = fn(i64, i64) -> i64;

fn part1() {
    #[derive(Debug)]
    struct EquationLine {
        operands: Vec<i64>,
        res: i64,
    }

    impl EquationLine {
        fn is_correct(&self) -> bool {
            self.permutations()
                .iter()
                .any(|perm| self.eval_permutation(perm) == self.res)
        }

        fn eval_permutation(&self, perm: &Vec<Op>) -> i64 {
            let mut cur_res = self.operands[0];
            let zip_iter = self.operands[1..].iter().zip(perm.iter());

            for (&rhs, &op) in zip_iter {
                cur_res = op(cur_res, rhs);
            }

            cur_res
        }

        fn permutations(&self) -> Vec<Vec<Op>> {
            let ops = [i64::wrapping_add, i64::wrapping_mul];
            

            (0..self.operands.len() - 1)
                .map(|_| (0..2).map(|i| ops[i]))
                .multi_cartesian_product()
                .collect::<Vec<_>>()
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct ParseLineError;

    impl FromStr for EquationLine {
        type Err = ParseLineError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (res, operand_str) = s.split_once(": ").unwrap();

            let operands: Vec<i64> = operand_str.split(" ").map(|n| n.parse().unwrap()).collect();
            Ok(Self {
                operands,
                res: res.trim().parse().unwrap(),
            })
        }
    }

    let equations: Vec<EquationLine> = read_lines("./day07/input")
        .unwrap()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut sum = 0i64;
    for eq in equations {
        if eq.is_correct() {
            sum += eq.res;
        }
    }
    println!("{}", sum);
}

fn part2() {
    fn concat(lhs: i64, rhs: i64) -> i64 {
        let factor = 10_i64.pow(rhs.ilog10() + 1);
        (lhs * factor) + rhs
    }
    #[derive(Debug)]
    struct EquationLine {
        operands: Vec<i64>,
        res: i64,
    }

    impl EquationLine {
        fn is_correct(&self) -> bool {
            self.permutations()
                .iter()
                .any(|perm| self.eval_permutation(perm) == self.res)
        }

        fn eval_permutation(&self, perm: &Vec<Op>) -> i64 {
            let mut cur_res = self.operands[0];
            let zip_iter = self.operands[1..].iter().zip(perm.iter());

            for (&rhs, &op) in zip_iter {
                cur_res = op(cur_res, rhs);
            }

            cur_res
        }

        fn permutations(&self) -> Vec<Vec<Op>> {
            let ops = [i64::wrapping_add, i64::wrapping_mul, concat];
            

            (0..self.operands.len() - 1)
                .map(|_| (0..3).map(|i| ops[i]))
                .multi_cartesian_product()
                .collect::<Vec<_>>()
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct ParseLineError;

    impl FromStr for EquationLine {
        type Err = ParseLineError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (res, operand_str) = s.split_once(": ").unwrap();

            let operands: Vec<i64> = operand_str.split(" ").map(|n| n.parse().unwrap()).collect();
            Ok(Self {
                operands,
                res: res.trim().parse().unwrap(),
            })
        }
    }

    let equations: Vec<EquationLine> = read_lines("./day07/input")
        .unwrap()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut sum = 0i64;
    for eq in equations {
        if eq.is_correct() {
            sum += eq.res;
        }
    }
    println!("{}", sum);

    let lines = read_lines("./day07/input").unwrap().collect::<Vec<_>>();
}
fn main() {
    part1();
    part2()
}
