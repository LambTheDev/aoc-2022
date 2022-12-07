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
    // The input seems to visit each directory depth first, and do "ls" on
    // first visit.
    // My idea here is to push the sum of all file sizes to a stack on "ls".
    // After exiting a directory with "cd ..", top of the stack will contain
    // the current directory's total sum. This sum needs also be added
    // to the parents sum.
    // Finally after the last command the rest of the stack needs to be
    // unwinded the same way as "cd ..".

    let mut directory_size_stack: Vec<u32> = Vec::new();
    let mut result_sum = 0;

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
                if sum < 100000 {
                    result_sum += sum;
                }
            }
            _ => (),
        }
    }

    // unwind rest of the stack
    for _ in 0..directory_size_stack.len() {
        let sum = pop_and_sum(&mut directory_size_stack);
        if sum < 100000 {
            result_sum += sum;
        }
    }

    Ok(result_sum.to_string())
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
        let actual = part1(input)?;
        assert_eq!(actual, "95437");

        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        Ok(())
    }
}
