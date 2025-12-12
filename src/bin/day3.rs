use anyhow::Result;

const PUZZLE_INPUT: &'static str = include_str!("input/day3.txt");

fn parse(input: &str) -> impl Iterator<Item = Box<[u64]>> {
    input
        .lines()
        .map(|x| x.bytes().map(|x| (x - b'0') as u64).collect())
}

fn get_max_n(bank: &[u64], n: usize) -> u64 {
    (0..n)
        .rev()
        .fold((0, 0), |(skip_len, joltage), taker| {
            let len = bank.len().saturating_sub(skip_len);
            let (idx, val) = bank
                .iter()
                .enumerate()
                .skip(skip_len)
                .take(len.saturating_sub(taker))
                .fold(
                    (0, 0),
                    |(old_i, max), (i, val)| if *val > max { (i, *val) } else { (old_i, max) },
                );
            (idx + 1, joltage * 10 + val)
        })
        .1
}

fn part_2(input: &str) -> u64 {
    parse(input).map(|bank| get_max_n(&bank, 12)).sum()
}

fn part_1(input: &str) -> u64 {
    parse(input).map(|bank| get_max_n(&bank, 2)).sum()
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

    const INPUT: &'static str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 3121910778619);
    }
}
