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
    let mut parts = input.split("\n\n");
    let mut stacks = parse_stacks(parts.next().ok_or("stacks not found")?)?;
    let commands = parse_commands(parts.next().ok_or("commands not found")?)?;

    for command in commands {
        for _ in 0..command.amount {
            // we can't borrow 'stacks' as mutable twice in the same block... so extract item first
            let item: char = {
                let from = stacks.get_mut(command.from).ok_or("invalid from stack")?;
                from.pop().ok_or("unexpected empty stack")?
            };
            let to = stacks.get_mut(command.to).ok_or("invalid to stack")?;
            to.push(item);
        }
    }

    get_message_from_stacks(stacks)
}

fn part2(input: &str) -> Result<String> {
    let mut parts = input.split("\n\n");
    let mut stacks = parse_stacks(parts.next().ok_or("stacks not found")?)?;
    let commands = parse_commands(parts.next().ok_or("commands not found")?)?;

    for command in commands {
        let mut items: Vec<char> = Vec::new();
        for _ in 0..command.amount {
            let from = stacks.get_mut(command.from).ok_or("invalid from stack")?;
            let item = from.pop().ok_or("unexpected empty stack")?;
            items.push(item);
        }
        // insert in reverse order... even if it's not a realistic crane now though :D
        for &item in items.iter().rev() {
            let to = stacks.get_mut(command.to).ok_or("invalid to stack")?;
            to.push(item);
        }
    }

    get_message_from_stacks(stacks)
}

struct Command {
    amount: u32,
    from: usize,
    to: usize,
}

fn parse_stacks(src: &str) -> Result<Vec<Vec<char>>> {
    let mut iter = src.lines().rev();
    let last_line = iter.next().unwrap();
    let stack_count = last_line.split_whitespace().count();
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stack_count);

    for _ in 0..stack_count {
        stacks.push(Vec::new())
    }

    for line in iter {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let item = line
                .chars()
                .skip(i * 4 + 1)
                .take(1)
                .next()
                .ok_or("expected longer line")?;
            if item != ' ' {
                stack.push(item);
            }
        }
    }

    Ok(stacks)
}

fn parse_commands(src: &str) -> Result<Vec<Command>> {
    let mut commands: Vec<Command> = Vec::new();
    for line in src.lines() {
        let mut words = line.split_whitespace();
        // each nth call "consumes" words iterator
        let amount: u32 = words.nth(1).ok_or("parsing amount failed")?.parse()?;
        let from: usize = words.nth(1).ok_or("parsing from failed")?.parse()?;
        let to: usize = words.nth(1).ok_or("parsing to failed")?.parse()?;
        commands.push(Command {
            amount,
            from: from - 1, // 1-index to 0-index
            to: to - 1,
        });
    }
    Ok(commands)
}

fn get_message_from_stacks(stacks: Vec<Vec<char>>) -> Result<String> {
    let mut msg = String::new();
    for mut stack in stacks {
        msg.push(stack.pop().ok_or("unexpected empty stack")?);
    }
    Ok(msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() -> Result<()> {
        let input = concat!(
            "    [D]    \n",
            "[N] [C]    \n",
            "[Z] [M] [P]\n",
            " 1   2   3 \n",
            "\n",
            "move 1 from 2 to 1\n",
            "move 3 from 1 to 3\n",
            "move 2 from 2 to 1\n",
            "move 1 from 1 to 2",
        );
        let actual = part1(input)?;
        assert_eq!(actual, "CMZ");

        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        let input = concat!(
            "    [D]    \n",
            "[N] [C]    \n",
            "[Z] [M] [P]\n",
            " 1   2   3 \n",
            "\n",
            "move 1 from 2 to 1\n",
            "move 3 from 1 to 3\n",
            "move 2 from 2 to 1\n",
            "move 1 from 1 to 2",
        );
        let actual = part2(input)?;
        assert_eq!(actual, "MCD");

        Ok(())
    }
}
