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
    let mut state: State = State {
        head_position: (0, 0),
        tail_position: (0, 0),
    };
    let mut positions_visited: HashSet<(i32, i32)> = HashSet::new();
    positions_visited.insert((0, 0)); // also add initial position

    for line in input.lines() {
        let direction = line.chars().next().ok_or("unexpected empty line")?;
        let amount = line.chars().skip(2).collect::<String>().parse::<u32>()?;

        for _ in 0..amount {
            let (new_state, tail_moved_to) = tick(&state, direction);
            state = new_state;
            if let Some(x) = tail_moved_to {
                positions_visited.insert(x);
            }
        }
    }
    Ok(positions_visited.len().to_string())
}

fn part2(_input: &str) -> Result<String> {
    Err("not implemented".into())
}

struct State {
    head_position: (i32, i32),
    tail_position: (i32, i32),
}

// advances simulation by one tick
// return new state and "events", in this case position if the tail moved
fn tick(state: &State, direction: char) -> (State, Option<(i32, i32)>) {
    let head_position = match direction {
        'U' => (state.head_position.0, state.head_position.1 + 1),
        'R' => (state.head_position.0 + 1, state.head_position.1),
        'D' => (state.head_position.0, state.head_position.1 - 1),
        'L' => (state.head_position.0 - 1, state.head_position.1),
        _ => panic!(),
    };

    let dx = head_position.0 - state.tail_position.0;
    let dy = head_position.1 - state.tail_position.1;

    let tail_moved_to = if dx.abs() > 1 || dy.abs() > 1 {
        Some(state.head_position) // move to where head was
    } else {
        None
    };
    let tail_position = tail_moved_to.unwrap_or(state.tail_position);

    (
        State {
            head_position,
            tail_position,
        },
        tail_moved_to,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        concat!("R 4\n", "U 4\n", "L 3\n", "D 1\n", "R 4\n", "D 1\n", "L 5\n", "R 2");

    #[test]
    fn part1_test() -> Result<()> {
        let actual = part1(INPUT)?;
        assert_eq!(actual, "13");

        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        Ok(())
    }
}
