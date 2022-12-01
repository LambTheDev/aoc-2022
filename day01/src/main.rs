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

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut max_total_calories = 0;
    let mut current_sum = 0;
    
    for line in input.lines() {
        if line.is_empty() {
            if current_sum > max_total_calories {
                max_total_calories = current_sum;
            }
            current_sum = 0;
        } else {
            let calories: i32 = str::parse(line)?;
            current_sum += calories;
        }
    }

    println!("{max_total_calories}");
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut total_calories: Vec<i32> = Vec::new();
    let mut current_sum = 0;
    
    for line in input.lines() {
        if line.is_empty() {
            total_calories.push(current_sum);
            current_sum = 0;
        } else {
            let calories: i32 = str::parse(line)?;
            current_sum += calories;
        }
    }

    total_calories.sort(); // ascending
    let sum_top_three: i32 = total_calories.iter().rev().take(3).sum();
    println!("{sum_top_three}");

    Ok(())
}
