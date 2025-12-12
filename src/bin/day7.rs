use std::collections::{HashMap, HashSet};

use anyhow::Result;

const PUZZLE_INPUT: &'static str = include_str!("input/day7.txt");
type Position = (u64, u64);

fn parse(input: &str) -> (Position, HashSet<Position>, Position) {
    let iter = input.lines().enumerate().flat_map(|(ridx, row)| {
        row.bytes()
            .enumerate()
            .map(move |(cidx, b)| (ridx as u64, cidx as u64, b))
    });

    let my = iter.clone().map(|(y, _, _)| y).max().unwrap();
    let mx = iter.clone().map(|(_, x, _)| x).max().unwrap();

    let (py, px, _) = iter.clone().find(|(_, _, b)| *b == b'S').unwrap();
    (
        (px, py),
        iter.filter_map(|(sy, sx, b)| (b == b'^').then(|| (sx, sy)))
            .collect(),
        (mx, my),
    )
}

fn part_1(input: &str) -> u64 {
    let (start_pos, splitters, (max_x, max_y)) = parse(input);
    let mut beams = HashSet::new();
    beams.insert(start_pos);

    let mut count = 0;
    loop {
        let mut beams_to_add = vec![];
        for beam in beams.iter() {
            let mut beam = beam.clone();
            beam.1 += 1;
            if splitters.contains(&beam) {
                count += 1;
                let mut new_beam = beam.clone();
                new_beam.0 += 1;
                beams_to_add.push(new_beam);
                beam.0 -= 1;
            }
            beams_to_add.push(beam);
        }

        beams.clear();
        for b in beams_to_add.into_iter() {
            beams.insert(b);
        }

        if !beams
            .iter()
            .any(|(x, y)| (0..=max_x).contains(x) && (0..=max_y).contains(y))
        {
            break;
        }
    }

    count
}
fn insert_or_add(map: &mut HashMap<Position, u64>, pos: Position, prev: u64) {
    let val = map.get(&pos).map(|x| *x);
    let val = val.map(|x| prev + x).unwrap_or(prev);
    map.insert(pos, val);
}

fn part_2(input: &str) -> u64 {
    let (start_pos, splitters, (max_x, max_y)) = parse(input);
    let mut sims: HashMap<Position, u64> = HashMap::new();
    sims.insert(start_pos, 1);

    loop {
        let mut sims_replace = HashMap::new();
        for (beam, prev) in sims.iter() {
            let mut beam = beam.clone();
            beam.1 += 1;
            if splitters.contains(&beam) {
                let mut new_beam = beam.clone();
                new_beam.0 -= 1;
                insert_or_add(&mut sims_replace, new_beam, *prev);
                beam.0 += 1;
            }

            insert_or_add(&mut sims_replace, beam, *prev);
        }

        sims = sims_replace;

        if !sims
            .iter()
            .any(|((x, y), _)| (0..=max_x).contains(x) && (0..=max_y).contains(y))
        {
            break;
        }
    }

    sims.values().sum()
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

    const INPUT: &'static str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 40);
    }
}
