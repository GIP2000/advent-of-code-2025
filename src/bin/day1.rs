use anyhow::Result;

const PUZZLE_INPUT: &'static str = include_str!("input/day1.txt");

fn parse(s: &str) -> impl Iterator<Item = isize> {
    s.lines().filter_map(|x| {
        let mut iter = x.chars();
        let dir = iter.next()?;
        let signum = match dir {
            'R' => Some(1isize),
            'L' => Some(-1),
            _ => None,
        }?;
        let val: isize = iter.collect::<String>().parse().ok()?;

        Some(val * signum)
    })
}

pub fn part_1(input: &str) -> Result<usize> {
    let list = parse(input);

    let mut start = 50isize;
    let mut count = 0usize;

    for number in list {
        let new_num = start + number;

        start = if new_num < 0 {
            100 - (new_num.abs() % 100)
        } else {
            new_num
        } % 100;

        if start == 0 {
            count += 1;
        }
    }
    Ok(count)
}

/// This is the stupid brute force way to do this
pub fn part_2_dumb(input: &str) -> Result<usize> {
    let list = parse(input);

    let mut start = 50isize;
    let mut count = 0usize;

    for number in list {
        let dir = number.signum();
        for _ in 0..number.abs() {
            start += 1 * dir;

            if start > 99 {
                start = 0;
            } else if start < 0 {
                start = 99;
            }

            if start == 0 {
                count += 1;
            }
        }
    }
    Ok(count)
}

/// This is the way to do it with only math
pub fn part_2_proc(input: &str) -> Result<usize> {
    let list = parse(input);

    let mut start = 50isize;
    let mut count = 0usize;

    for number in list {
        let increment = number.abs() as usize / 100;
        if increment > 0 {
            count += increment;
        }

        let raw_add = start + number;

        let new_num = if raw_add < 0 {
            100 - (raw_add.abs() % 100)
        } else {
            raw_add
        } % 100;

        let dir = number.signum();
        if start != 0
            && ((dir > 0 && start > new_num) || (dir < 0 && new_num > start) || new_num == 0)
        {
            count += 1;
        }
        start = new_num;
    }
    Ok(count)
}

pub fn part_2(input: &str) -> Result<usize> {
    Ok(parse(input)
        .fold((0, 50), |(mut count, mut start), number| {
            let increment = number.abs() as usize / 100;
            if increment > 0 {
                count += increment;
            }

            let raw_add = start + number;

            let new_num = if raw_add < 0 {
                100 - (raw_add.abs() % 100)
            } else {
                raw_add
            } % 100;

            let dir = number.signum();
            if start != 0
                && ((dir > 0 && start > new_num) || (dir < 0 && new_num > start) || new_num == 0)
            {
                count += 1;
            }
            start = new_num;

            (count, start)
        })
        .0)
}

fn main() -> Result<()> {
    let result1 = part_1(PUZZLE_INPUT)?;
    println!("Part 1 : {result1}");

    let result2 = part_2(PUZZLE_INPUT)?;
    println!("Part 2 : {result2}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = r#"""
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"""#;

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT).unwrap();
        assert_eq!(result, 6);
    }
}
