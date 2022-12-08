use std::env;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    // first arg is the name of the program
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => String::from("input.txt"),
    };
    let mut input: String = String::new();
    File::open(path)?.read_to_string(&mut input)?;

    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<String> {
    let grid = parse_grid(input)?;
    let mut visibility_map: Vec<bool> = vec![false; grid.values.len()];

    // from up
    for x in 0..grid.width {
        let mut highest = None;
        for y in 0..grid.height {
            let i = y * grid.width + x;
            let height = grid.values[i];
            match highest {
                Some(h) if height <= h => (),
                _ => {
                    highest = Some(height);
                    visibility_map[i] = true;
                }
            }
        }
    }

    // from down
    for x in 0..grid.width {
        let mut highest = None;
        for y in (0..grid.height).rev() {
            let i = y * grid.width + x;
            let height = grid.values[i];
            match highest {
                Some(h) if height <= h => (),
                _ => {
                    highest = Some(height);
                    visibility_map[i] = true;
                }
            }
        }
    }

    // from left
    for y in 0..grid.height {
        let mut highest = None;
        for x in 0..grid.width {
            let i = y * grid.width + x;
            let height = grid.values[i];
            match highest {
                Some(h) if height <= h => (),
                _ => {
                    highest = Some(height);
                    visibility_map[i] = true;
                }
            }
        }
    }

    // from right
    for y in 0..grid.height {
        let mut highest = None;
        for x in (0..grid.width).rev() {
            let i = y * grid.width + x;
            let height = grid.values[i];
            match highest {
                Some(h) if height <= h => (),
                _ => {
                    highest = Some(height);
                    visibility_map[i] = true;
                }
            }
        }
    }

    // debug print
    print_grid(&grid, &visibility_map);

    Ok(visibility_map.iter().filter(|x| **x).count().to_string())
}

fn part2(_input: &str) -> Result<String> {
    Err("not implemented".into())
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    values: Vec<u32>,
}

fn parse_grid(input: &str) -> Result<Grid> {
    let width = input.lines().next().ok_or("missing lines")?.len();
    let height = input.lines().count();
    let mut values: Vec<u32> = Vec::new();

    for line in input.lines() {
        let column: Vec<u32> = line.chars().flat_map(|c| c.to_digit(10)).collect();
        values.extend(column);
    }
    Ok(Grid {
        width,
        height,
        values,
    })
}

fn print_grid(grid: &Grid, visibility_map: &[bool]) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let i = y * grid.width + x;
            let c = if visibility_map[i] { 'X' } else { '_' };
            print!("{c}");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = concat!("30373\n", "25512\n", "65332\n", "33549\n", "35390");

    #[test]
    fn part1_test() -> Result<()> {
        let actual = part1(INPUT)?;
        assert_eq!(actual, "21");

        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        Ok(())
    }
}
