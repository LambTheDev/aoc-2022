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
        let priority = calculate_priority(*common_item);
        priority_sum += priority;
    }
    Ok(priority_sum.to_string())
}

fn part2(input: &str) -> Result<String> {
    let sacks: Vec<_> = input
        .lines()
        .map(|s| s.chars().collect::<HashSet<_>>())
        .collect();

    let mut priority_sum = 0;
    for group in sacks.chunks(3) {
        let sack1 = &group[0];
        let sack2 = &group[1];
        let sack3 = &group[2];

        let intersection12: HashSet<_> = sack1.intersection(sack2).cloned().collect();
        let mut intersection123 = intersection12.intersection(sack3);
        let common_item = intersection123.next().unwrap();
        let priority = calculate_priority(*common_item);
        priority_sum += priority;
    }
    Ok(priority_sum.to_string())
}

fn calculate_priority(c: char) -> u32 {
    let ascii = c as u32;
    if ascii > 96 {
        ascii - 96
    } else {
        ascii - 64 + 26
    }
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

    #[test]
    fn part2_test() -> Result<()> {
        let input = concat!(
            "vJrwpWtwJgWrhcsFMMfFFhFp\n",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
            "PmmdzqPrVvPwwTWBwg\n",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
            "ttgJtRGJQctTZtZT\n",
            "CrZsJsPPZsGzwwsLwLmpwMDw\n",
        );
        let actual = part2(input)?;
        assert_eq!(actual, "70");

        Ok(())
    }
}
