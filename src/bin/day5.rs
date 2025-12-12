use anyhow::Result;
use std::{collections::HashSet, ops::RangeInclusive};

const PUZZLE_INPUT: &'static str = include_str!("input/day5.txt");

fn parse(input: &str) -> Result<(Vec<RangeInclusive<u64>>, impl Iterator<Item = u64>)> {
    let (dbstr, liststr) = input
        .split_once("\n\n")
        .ok_or(anyhow::anyhow!("Can't find it"))?;

    let db = dbstr
        .lines()
        .map(|x| {
            let (start, end) = x.split_once("-").unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    let list = liststr.lines().flat_map(|x| x.parse().ok());

    Ok((db, list))
}

fn part_1(input: &str) -> u64 {
    let (good_ids, available_ids) = parse(input).unwrap();
    available_ids
        .filter_map(|x| {
            good_ids
                .iter()
                .find(|range| x >= *range.start() && x <= *range.end())
                .map(|_| 1)
        })
        .sum()
}

fn part_2(input: &str) -> u64 {
    let (mut good_ids, _) = parse(input).unwrap();
    loop {
        let ranges: Vec<RangeInclusive<u64>> = good_ids.clone();
        let mut new_good_ids: Vec<RangeInclusive<u64>> = Vec::new();
        let mut skip_ids = HashSet::new();

        let mut flag = false;
        'outer: for (ie, existing) in good_ids.iter().enumerate() {
            if skip_ids.contains(&ie) {
                continue;
            }
            for (ic, check) in ranges.iter().enumerate() {
                if skip_ids.contains(&ic) || ic == ie {
                    continue;
                }

                if existing.contains(check.start())
                    || existing.contains(check.end())
                    || check.contains(existing.start())
                    || check.contains(existing.end())
                {
                    let start = *existing.start().min(check.start());
                    let end = *existing.end().max(check.end());
                    let new = start..=end;
                    new_good_ids.push(new);
                    skip_ids.insert(ie);
                    skip_ids.insert(ic);
                    flag = true;
                    continue 'outer;
                }
            }
            new_good_ids.push(existing.clone())
        }
        good_ids = new_good_ids;
        if !flag {
            break;
        }
    }
    good_ids.into_iter().map(|x| x.count() as u64).sum()
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

    const INPUT: &'static str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 14);
    }
}
