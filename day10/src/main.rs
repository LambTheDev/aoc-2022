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
    let mut cycles = 0;
    let mut reg_x = 1;
    let mut signal_strengths: Vec<i32> = Vec::new();

    let mut tick = |x: i32| {
        cycles += 1;
        if cycles % 40 == 19 {
            let signal_strength = x * (cycles + 1);
            signal_strengths.push(signal_strength);
        }
    };

    for line in input.lines() {
        let mut split = line.split(' ');

        match split.next() {
            Some("noop") => {
                tick(reg_x);
            }
            Some("addx") => {
                tick(reg_x);
                let v: i32 = split.next().ok_or("missing v")?.parse()?;
                reg_x += v;
                tick(reg_x);
            }
            _ => panic!(),
        };
    }
    Ok(signal_strengths.iter().sum::<i32>().to_string())
}

fn part2(_input: &str) -> Result<String> {
    Err("not implemented".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = concat!(
        "addx 15\n",
        "addx -11\n",
        "addx 6\n",
        "addx -3\n",
        "addx 5\n",
        "addx -1\n",
        "addx -8\n",
        "addx 13\n",
        "addx 4\n",
        "noop\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx 5\n",
        "addx -1\n",
        "addx -35\n",
        "addx 1\n",
        "addx 24\n",
        "addx -19\n",
        "addx 1\n",
        "addx 16\n",
        "addx -11\n",
        "noop\n",
        "noop\n",
        "addx 21\n",
        "addx -15\n",
        "noop\n",
        "noop\n",
        "addx -3\n",
        "addx 9\n",
        "addx 1\n",
        "addx -3\n",
        "addx 8\n",
        "addx 1\n",
        "addx 5\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -36\n",
        "noop\n",
        "addx 1\n",
        "addx 7\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 2\n",
        "addx 6\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "addx 7\n",
        "addx 1\n",
        "noop\n",
        "addx -13\n",
        "addx 13\n",
        "addx 7\n",
        "noop\n",
        "addx 1\n",
        "addx -33\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 2\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx 8\n",
        "noop\n",
        "addx -1\n",
        "addx 2\n",
        "addx 1\n",
        "noop\n",
        "addx 17\n",
        "addx -9\n",
        "addx 1\n",
        "addx 1\n",
        "addx -3\n",
        "addx 11\n",
        "noop\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "addx -13\n",
        "addx -19\n",
        "addx 1\n",
        "addx 3\n",
        "addx 26\n",
        "addx -30\n",
        "addx 12\n",
        "addx -1\n",
        "addx 3\n",
        "addx 1\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -9\n",
        "addx 18\n",
        "addx 1\n",
        "addx 2\n",
        "noop\n",
        "noop\n",
        "addx 9\n",
        "noop\n",
        "noop\n",
        "noop\n",
        "addx -1\n",
        "addx 2\n",
        "addx -37\n",
        "addx 1\n",
        "addx 3\n",
        "noop\n",
        "addx 15\n",
        "addx -21\n",
        "addx 22\n",
        "addx -6\n",
        "addx 1\n",
        "noop\n",
        "addx 2\n",
        "addx 1\n",
        "noop\n",
        "addx -10\n",
        "noop\n",
        "noop\n",
        "addx 20\n",
        "addx 1\n",
        "addx 2\n",
        "addx 2\n",
        "addx -6\n",
        "addx -11\n",
        "noop\n",
        "noop\n",
        "noop"
    );

    #[test]
    fn part1_test() -> Result<()> {
        let actual = part1(INPUT)?;
        assert_eq!(actual, "13140");

        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        Ok(())
    }
}
