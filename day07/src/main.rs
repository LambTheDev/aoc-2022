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
    let sum: u32 = get_directory_sizes(input)?
        .iter()
        .filter(|x| **x < 100000)
        .sum();
    Ok(sum.to_string())
}

fn part2(input: &str) -> Result<String> {
    let mut directory_sizes = get_directory_sizes(input)?;
    let total_used = directory_sizes.last().unwrap();
    let total_unused = 70000000 - total_used;
    let needed = 30000000 - total_unused;

    directory_sizes.sort();
    let smallest = directory_sizes.iter().find(|x| **x > needed).unwrap();
    Ok(smallest.to_string())
}

fn get_directory_sizes(input: &str) -> Result<Vec<u32>> {
    // The input seems to visit each directory depth first, and do "ls" on
    // first visit.
    // My idea here is to push the sum of all file sizes to a stack on "ls".
    // After exiting a directory with "cd ..", top of the stack will contain
    // the current directory's total sum. This sum needs also be added
    // to the parents sum.
    // Finally after the last command the rest of the stack needs to be
    // unwinded the same way as "cd ..".

    let mut directory_size_stack: Vec<u32> = Vec::new();
    let mut result: Vec<u32> = Vec::new();

    let pop_and_sum = |stack: &mut Vec<u32>| {
        let sum = stack.pop().unwrap_or(0);
        if let Some(parent) = stack.last_mut() {
            *parent += sum;
        }
        sum
    };

    for block in input.split("$ ").skip(1) {
        let mut lines = block.lines();
        let command = lines.next().ok_or("unexpected empty line")?;
        let output: Vec<_> = lines.collect();

        match command {
            "ls" => {
                let sum: u32 = output
                    .iter()
                    .flat_map(|x| x.split(' '))
                    .flat_map(str::parse::<u32>)
                    .sum();
                directory_size_stack.push(sum);
            }
            "cd .." => {
                let sum = pop_and_sum(&mut directory_size_stack);
                result.push(sum);
            }
            _ => (),
        }
    }

    // unwind rest of the stack
    for _ in 0..directory_size_stack.len() {
        let sum = pop_and_sum(&mut directory_size_stack);
        result.push(sum);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = concat!(
        "$ cd /\n",
        "$ ls\n",
        "dir a\n",
        "14848514 b.txt\n",
        "8504156 c.dat\n",
        "dir d\n",
        "$ cd a\n",
        "$ ls\n",
        "dir e\n",
        "29116 f\n",
        "2557 g\n",
        "62596 h.lst\n",
        "$ cd e\n",
        "$ ls\n",
        "584 i\n",
        "$ cd ..\n",
        "$ cd ..\n",
        "$ cd d\n",
        "$ ls\n",
        "4060174 j\n",
        "8033020 d.log\n",
        "5626152 d.ext\n",
        "7214296 k"
    );

    #[test]
    fn part1_test() -> Result<()> {
        let actual = part1(INPUT)?;
        assert_eq!(actual, "95437");

        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let actual = part2(INPUT)?;
        assert_eq!(actual, "24933642");

        Ok(())
    }
}
