use std::collections::HashSet;
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
    let mut priority_sum = 0;
    for line in input.lines() {
        let count_half = line.len() / 2;
        let compartment1: HashSet<_> = line.chars().take(count_half).collect();
        let compartment2: HashSet<_> = line.chars().skip(count_half).collect();
        let mut intersection = compartment1.intersection(&compartment2);
        let common_item = intersection.next().unwrap();
        let ascii = *common_item as u32;
        let priority = if ascii > 96 {
            ascii - 96
        } else {
            ascii - 64 + 26
        };
        priority_sum += priority;
    }
    Ok(priority_sum.to_string())
}

fn part2(_input: &str) -> Result<String> {
    Err("not implemented".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() -> Result<()> {
        let input = concat!(
            "vJrwpWtwJgWrhcsFMMfFFhFp\n",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
            "PmmdzqPrVvPwwTWBwg\n",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
            "ttgJtRGJQctTZtZT\n",
            "CrZsJsPPZsGzwwsLwLmpwMDw\n",
        );
        let actual = part1(input)?;
        assert_eq!(actual, "157");

        Ok(())
    }
}
