use std::{collections::HashSet, ops::RangeInclusive};

use anyhow::Result;

const PUZZLE_INPUT: &'static str = include_str!("input/day2.txt");

fn parse(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    input.trim().split(",").filter_map(|x| {
        let x = x.trim();
        let mut iter = x.split("-");
        let first = iter.next()?.parse().ok()?;
        let second = iter.next()?.parse().ok()?;
        Some(first..=second)
    })
}

fn is_repeating_once(num: u64) -> bool {
    let num_str = num.to_string();
    if num_str.len() % 2 != 0 {
        return false;
    }
    let (first, second) = num_str.split_at(num_str.len() / 2);
    first == second
}

fn part1(input: &str) -> u64 {
    let mut dp: HashSet<u64> = HashSet::new();
    let mut acc = 0u64;

    let list = parse(input);

    for range in list {
        for num in range {
            if dp.contains(&num) {
                break;
            }
            // todo check if the number is repeating
            if is_repeating_once(num) {
                acc += num;
            }
            dp.insert(num);
        }
    }
    return acc;
}

fn is_repeating(num: u64) -> bool {
    let num_str = num.to_string();

    'outer: for chunk in 1..num_str.len() {
        let substr = &num_str[0..chunk];
        let rest = &num_str[chunk..];

        let len = substr.len();
        if rest.len() % len != 0 {
            continue;
        }
        let mut end_idx = len;

        while end_idx <= rest.len() {
            let testsubstr = &rest[end_idx - len..end_idx];

            if testsubstr != substr {
                continue 'outer;
            }
            end_idx += len;
        }
        return true;
    }

    return false;
}

fn part2(input: &str) -> u64 {
    let mut dp: HashSet<u64> = HashSet::new();
    let mut acc = 0u64;

    let list = parse(input);

    for range in list {
        for num in range {
            if dp.contains(&num) {
                break;
            }
            // todo check if the number is repeating
            if is_repeating(num) {
                acc += num;
            }
            dp.insert(num);
        }
    }
    return acc;
}

fn main() -> Result<()> {
    let result1 = part1(PUZZLE_INPUT);
    println!("PART 1: {result1}");

    let result2 = part2(PUZZLE_INPUT);
    println!("PART 2: {result2}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 4174379265);
    }
}
