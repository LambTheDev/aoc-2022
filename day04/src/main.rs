use std::env;
use std::fs::File;
use std::io::Read;

use regex::Regex;

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
    let ranges = parse_ranges(input);
    let mut count = 0;

    for ((l1, l2), (r1, r2)) in ranges {
        let is_right_contained = (r1 >= l1) && (r2 <= l2);
        let is_left_contained = (l1 >= r1) && (l2 <= r2);
        if is_right_contained || is_left_contained {
            count += 1;
        }
    }

    Ok(count.to_string())
}

fn part2(input: &str) -> Result<String> {
    let ranges = parse_ranges(input);
    let mut count = 0;

    for ((l1, l2), (r1, r2)) in ranges {
        if (l1 <= r2) && (l2 >= r1) {
            count += 1;
        }
    }

    Ok(count.to_string())
}

type Range = (u32, u32);

fn parse_ranges(input: &str) -> Vec<(Range, Range)> {
    let r = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let mut ranges: Vec<(Range, Range)> = Vec::new();

    for line in input.lines() {
        let captures = r.captures(line).unwrap();
        let l1: u32 = (captures[1]).parse().unwrap();
        let l2: u32 = (captures[2]).parse().unwrap();
        let r1: u32 = (captures[3]).parse().unwrap();
        let r2: u32 = (captures[4]).parse().unwrap();

        ranges.push(((l1, l2), (r1, r2)))
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() -> Result<()> {
        let input = concat!(
            "2-4,6-8\n",
            "2-3,4-5\n",
            "5-7,7-9\n",
            "2-8,3-7\n",
            "6-6,4-6\n",
            "2-6,4-8",
        );
        let actual = part1(input)?;
        assert_eq!(actual, "2");

        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = concat!(
            "2-4,6-8\n",
            "2-3,4-5\n",
            "5-7,7-9\n",
            "2-8,3-7\n",
            "6-6,4-6\n",
            "2-6,4-8",
        );
        let actual = part2(input)?;
        assert_eq!(actual, "4");

        Ok(())
    }
}
