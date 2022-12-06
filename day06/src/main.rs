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
    let data: Vec<char> = input.chars().collect();
    let mut nth_character = 0;
    for (i, window) in data.windows(4).enumerate() {
        let hashset: HashSet<char> = HashSet::from_iter(window.iter().cloned());
        if hashset.len() == 4 {
            // first time a marker appears is after the nth character arrives
            // so with 4 size window, for example index 3 means after 4th window = after 7th char
            nth_character = i + 4;
            break;
        }
    }
    Ok(nth_character.to_string())
}

fn part2(_input: &str) -> Result<String> {
    Err("not implemented".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() -> Result<()> {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let actual = part1(input)?;
        assert_eq!(actual, "7");

        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        Ok(())
    }
}
