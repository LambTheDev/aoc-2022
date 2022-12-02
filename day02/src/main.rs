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
    // rust has an implementation to convert from Vec<Result<_>> to Result<Vec<_>>
    // we just need to provide the wanted type here
    let rounds: Result<Vec<_>> = input.lines().map(parse_round_part1).collect();
    let score: i32 = rounds?.iter().map(get_score).sum();

    Ok(score.to_string())
}

fn part2(input: &str) -> Result<String> {
    let rounds: Result<Vec<_>> = input.lines().map(parse_round_part2).collect();
    let score: i32 = rounds?.iter().map(get_score).sum();

    Ok(score.to_string())
}

fn parse_round_part1(src: &str) -> Result<(Shapes, Shapes)> {
    let opponent = match src.chars().nth(0).ok_or("unexpected input")? {
        'A' => Ok(Shapes::Rock),
        'B' => Ok(Shapes::Paper),
        'C' => Ok(Shapes::Scissors),
        _ => Err("invalid opponent shape"),
    }?;
    let me = match src.chars().nth(2).ok_or("unexpected input")? {
        'X' => Ok(Shapes::Rock),
        'Y' => Ok(Shapes::Paper),
        'Z' => Ok(Shapes::Scissors),
        _ => Err("invalid my shape"),
    }?;

    Ok((opponent, me))
}

fn parse_round_part2(src: &str) -> Result<(Shapes, Shapes)> {
    let opponent = match src.chars().nth(0).ok_or("unexpected input")? {
        'A' => Ok(Shapes::Rock),
        'B' => Ok(Shapes::Paper),
        'C' => Ok(Shapes::Scissors),
        _ => Err("invalid opponent shape"),
    }?;
    let me = match src.chars().nth(2).ok_or("unexpected input")? {
        'X' => match opponent {
            Shapes::Rock => Ok(Shapes::Scissors),
            Shapes::Paper => Ok(Shapes::Rock),
            Shapes::Scissors => Ok(Shapes::Paper),
        },
        'Y' => Ok(opponent.clone()),
        'Z' => match opponent {
            Shapes::Rock => Ok(Shapes::Paper),
            Shapes::Paper => Ok(Shapes::Scissors),
            Shapes::Scissors => Ok(Shapes::Rock),
        },
        _ => Err("invalid round result"),
    }?;

    Ok((opponent, me))
}

fn get_score(round: &(Shapes, Shapes)) -> i32 {
    let outcome_score = match &round {
        // win
        (Shapes::Rock, Shapes::Paper) => 6,
        (Shapes::Paper, Shapes::Scissors) => 6,
        (Shapes::Scissors, Shapes::Rock) => 6,
        // lose
        (Shapes::Rock, Shapes::Scissors) => 0,
        (Shapes::Paper, Shapes::Rock) => 0,
        (Shapes::Scissors, Shapes::Paper) => 0,
        // draw
        (lhs, rhs) if lhs == rhs => 3,
        _ => panic!(),
    };
    let shape_score = match &round {
        (_, Shapes::Rock) => 1,
        (_, Shapes::Paper) => 2,
        (_, Shapes::Scissors) => 3,
    };
    outcome_score + shape_score
}

#[derive(Clone, PartialEq)]
enum Shapes {
    Rock,
    Paper,
    Scissors,
}
