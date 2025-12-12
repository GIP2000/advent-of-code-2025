use std::collections::HashMap;

use anyhow::Result;

const PUZZLE_INPUT: &'static str = include_str!("input/day8.txt");
type Position = (u64, u64, u64);

fn parse(input: &str) -> Vec<Position> {
    input
        .lines()
        .flat_map(|x| {
            let mut pos_iter = x.split(",").flat_map(|x| x.parse().ok());
            let x = pos_iter.next()?;
            let y = pos_iter.next()?;
            let z = pos_iter.next()?;
            Some((x, y, z))
        })
        .collect()
}

fn square_distance(a: Position, b: Position) -> u64 {
    fn safe_sub(a: u64, b: u64) -> u64 {
        a.max(b) - a.min(b)
    }

    safe_sub(a.0, b.0).pow(2) + safe_sub(a.1, b.1).pow(2) + safe_sub(a.2, b.2).pow(2)
}

fn part_1(input: &str, amount: usize) -> u64 {
    let lst = parse(input);

    let mut flat_dists_sorted: Vec<_> = lst
        .iter()
        .enumerate()
        .flat_map(|(io, outer)| {
            lst.iter()
                .enumerate()
                .filter(move |(ii, _)| *ii != io)
                .map(move |(ii, inner)| (io, ii, square_distance(inner.clone(), outer.clone())))
        })
        .collect();

    flat_dists_sorted.sort_by_key(|(_, _, dist)| *dist);
    flat_dists_sorted.reverse();

    println!("sorted: {:?}", flat_dists_sorted);

    let mut circuit_count = 0;
    let mut circuits: HashMap<usize, u64> = HashMap::new();

    for (box_a, box_b, _) in flat_dists_sorted.into_iter().take(amount) {
        let circuit_a = circuits.get(&box_a).map(|x| *x);
        let circuit_b = circuits.get(&box_b).map(|x| *x);

        match (circuit_a, circuit_b) {
            (None, None) => {
                circuits.insert(box_a, circuit_count);
                circuits.insert(box_b, circuit_count);
                circuit_count += 1;
            }
            (Some(x), None) | (None, Some(x)) => {
                circuits.insert(box_a, x);
                circuits.insert(box_b, x);
            }
            (Some(a), Some(b)) => {
                for (_, id) in circuits.iter_mut().filter(|(_, id)| **id == b) {
                    *id = a;
                }
            }
        };
        println!("circuits {:?}: ", circuits);
    }

    let mut circs: HashMap<u64, u64> = HashMap::new();
    for id in circuits.into_values() {
        if let Some(x) = circs.get_mut(&id) {
            *x += 1;
        } else {
            circs.insert(id, 1);
        }
    }

    let mut circs: Vec<_> = circs.into_values().collect();
    circs.sort();
    circs.into_iter().take(3).product()
}
fn part_2(input: &str) -> u64 {
    0
}

fn main() -> Result<()> {
    let result1 = part_1(PUZZLE_INPUT, 1000);
    println!("PART 1: {result1}");

    // let result2 = part_2(PUZZLE_INPUT);
    // println!("PART 2: {result2}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT, 10);
        assert_eq!(result, 21);
    }

    // #[test]
    // fn test_part_2() {
    //     let result = part_2(INPUT);
    //     assert_eq!(result, 40);
    // }
}
