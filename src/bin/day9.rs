use anyhow::Result;

const PUZZLE_INPUT: &'static str = include_str!("input/day9.txt");
type Position = (u64, u64);

fn parse(input: &str) -> Vec<Position> {
    input
        .lines()
        .flat_map(|x| {
            let mut iter = x.split(",");

            let x = iter.next()?.parse().ok()?;
            let y = iter.next()?.parse().ok()?;

            Some((x, y))
        })
        .collect()
}

fn calc_area(a: &Position, b: &Position) -> u64 {
    (a.0.max(b.0) - a.0.min(b.0) + 1) * (a.1.max(b.1) - a.1.min(b.1) + 1)
}

fn part_1(input: &str) -> u64 {
    let lst = parse(input);

    let mut area = 0;
    for outer in lst.iter() {
        for inner in lst.iter() {
            let inner_area = calc_area(outer, inner);
            println!("outer: {outer:?} <-> {inner:?} = {inner_area:?}");
            if area < inner_area {
                area = inner_area;
            }
        }
    }

    area
}

fn part_2(input: &str) -> u64 {
    0
}

fn main() -> Result<()> {
    let result1 = part_1(PUZZLE_INPUT);
    println!("PART 1: {result1}");

    // let result2 = part_2(PUZZLE_INPUT);
    // println!("PART 2: {result2}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 50);
    }

    // #[test]
    // fn test_part_2() {
    //     let result = part_2(INPUT);
    //     assert_eq!(result, 25272);
    // }
}
