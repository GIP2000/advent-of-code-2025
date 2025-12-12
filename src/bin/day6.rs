use std::str::FromStr;

use anyhow::{bail, Result};

const PUZZLE_INPUT: &'static str = include_str!("input/day6.txt");

#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        use Op::*;
        Ok(match s {
            "*" => Mul,
            "+" => Add,
            "-" => Sub,
            "/" => Div,
            _ => bail!(""),
        })
    }
}

#[derive(Debug)]
struct MathProblem {
    numbers: Vec<u64>,
    operation: Op,
}

impl MathProblem {
    pub fn run(&self) -> u64 {
        let iter = self.numbers.iter();
        match self.operation {
            Op::Add => iter.sum(),
            Op::Sub => todo!(),
            Op::Mul => iter.product(),
            Op::Div => todo!(),
        }
    }
}

fn parse_1(input: &str) -> Vec<MathProblem> {
    let mut lst: Vec<_> = input
        .lines()
        .map(|x| x.split(" ").filter(|x| !x.trim().is_empty()).enumerate())
        .collect();
    let mut problems = vec![];
    let mut ops = vec![];

    loop {
        let mut found_one = false;
        for val in lst.iter_mut() {
            if let Some((i, x)) = val.next() {
                found_one = true;
                if let Ok(num) = x.parse::<u64>() {
                    if i >= problems.len() {
                        for _ in problems.len()..=i {
                            problems.push(Vec::new());
                        }
                    }
                    problems[i].push(num);
                } else if let Ok(op) = x.parse::<Op>() {
                    ops.push(op);
                }
            }
        }
        if !found_one {
            break;
        }
    }

    problems
        .into_iter()
        .zip(ops.into_iter())
        .map(|(numbers, operation)| MathProblem { numbers, operation })
        .collect()
}

fn part_1(input: &str) -> u64 {
    let val = parse_1(input);
    val.into_iter()
        .map(|x| {
            let result = x.run();
            result
        })
        .sum()
}

fn parse_2(input: &str) -> Vec<MathProblem> {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|x| x.bytes().collect::<Vec<_>>())
        .collect();

    let mut problems = vec![];
    let mut ops = vec![];
    let mut ip = 0;

    let row_len = grid.iter().map(|x| x.len()).max().unwrap();

    // i think this is a grid
    for ir in 0..row_len {
        let mut col: Vec<u8> = vec![];
        let mut move_on = true;
        for ic in 0..grid.len() {
            let digit = if let Some(digit) = grid[ic].get(ir) {
                *digit
            } else {
                continue;
            };

            if digit != b' ' {
                move_on = false;
            }

            if digit.is_ascii_digit() {
                col.push(digit);
                continue;
            }
            let op = match digit {
                b'*' => Op::Mul,
                b'+' => Op::Add,
                _ => {
                    continue;
                }
            };
            ops.push(op);
        }

        if move_on {
            ip += 1;
        }

        if ip >= problems.len() {
            for _ in problems.len()..=ip {
                problems.push(vec![]);
            }
        }
        if let Ok(value) = col
            .into_iter()
            .map(|x| x as char)
            .collect::<String>()
            .parse::<u64>()
        {
            problems[ip].push(value);
        }
    }

    problems
        .into_iter()
        .zip(ops.into_iter())
        .map(|(numbers, operation)| MathProblem { numbers, operation })
        .collect()
}

fn part_2(input: &str) -> u64 {
    let val = parse_2(input);
    val.into_iter()
        .map(|x| {
            let result = x.run();
            result
        })
        .sum()
}

fn main() -> Result<()> {
    let result1 = part_1(PUZZLE_INPUT);
    println!("PART 1: {result1}");

    let result2 = part_2(PUZZLE_INPUT);
    println!("PART 2: {result2}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = r#"
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 3263827);
    }
}
