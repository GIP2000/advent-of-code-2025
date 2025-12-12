
use anyhow::Result;

const PUZZLE_INPUT: &'static str = include_str!("input/day4.txt");

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|x| x.bytes().map(|x| x == b'@').collect())
        .collect()
}

macro_rules! min_max_or_continue {
    ($val: expr, $min: expr, $max: expr) => {
        if $val < $min || $val > $max {
            continue;
        } else {
            $val
        }
    };
}

fn part_1(input: &str) -> u64 {
    let grid = parse(input);
    let mut sum = 0u64;
    for (row_idx, row) in grid.iter().enumerate() {
        'main: for (col_idx, is_paper) in row.iter().enumerate() {
            if !is_paper {
                continue;
            }

            let mut count = 0;
            for rdx in -1..=1 {
                for cdx in -1..=1 {
                    if rdx == 0 && cdx == 0 {
                        continue;
                    }

                    let new_row =
                        min_max_or_continue!(row_idx as isize + rdx, 0, (grid.len() - 1) as isize)
                            as usize;

                    let new_col = min_max_or_continue!(
                        col_idx as isize + cdx,
                        0,
                        (grid[new_row].len() - 1) as isize
                    ) as usize;

                    let grid_val = grid[new_row][new_col];

                    count += grid_val as u8;

                    if count >= 4 {
                        continue 'main;
                    }
                }
            }
            sum += 1;
        }
    }
    sum
}

fn part_2(input: &str) -> u64 {
    let mut grid = parse(input);
    let mut sum = 0u64;
    loop {
        let prev_sum = sum;
        for row_idx in 0..grid.len() {
            'main: for col_idx in 0..grid[row_idx].len() {
                let is_paper = grid[row_idx][col_idx];

                if !is_paper {
                    continue;
                }

                let mut count = 0;
                for rdx in -1..=1 {
                    for cdx in -1..=1 {
                        if rdx == 0 && cdx == 0 {
                            continue;
                        }

                        let new_row = min_max_or_continue!(
                            row_idx as isize + rdx,
                            0,
                            (grid.len() - 1) as isize
                        ) as usize;

                        let new_col = min_max_or_continue!(
                            col_idx as isize + cdx,
                            0,
                            (grid[new_row].len() - 1) as isize
                        ) as usize;

                        let grid_val = grid[new_row][new_col];

                        count += grid_val as u8;

                        if count >= 4 {
                            continue 'main;
                        }
                    }
                }
                sum += 1;
                grid[row_idx][col_idx] = false;
            }
        }
        if prev_sum == sum {
            break;
        }
    }
    sum
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

    const INPUT: &'static str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT);
        assert_eq!(result, 43);
    }
}
