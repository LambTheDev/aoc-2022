use lazy_static::lazy_static;
use regex::Regex;
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
    let mut monkes: Vec<Monke> = Vec::new();
    let mut monke_items: Vec<Vec<i32>> = Vec::new();
    let mut activity: Vec<u32> = Vec::new();

    for monke_str in input.split("\n\n") {
        let (monke, items) = parse_monke(monke_str)?;
        monkes.push(monke);
        monke_items.push(items);
        activity.push(0);
    }

    for _ in 0..20 {
        for (i, (monke, activity)) in monkes.iter().zip(activity.iter_mut()).enumerate() {
            let items = monke_items[i].clone();
            monke_items[i] = Vec::new();

            for mut worry_level in items {
                worry_level = evaluate(&monke.operation, worry_level);
                worry_level /= 3;
                let target_monke_id = if worry_level % monke.test_divisible == 0 {
                    monke.test_true_monke
                } else {
                    monke.test_false_monke
                };
                // taking a shortcut here... id happens to be the index :)
                monke_items[target_monke_id as usize].push(worry_level);
                *activity += 1;
            }
        }
    }

    activity.sort();

    let top_monkes: Vec<_> = activity.iter().rev().take(2).collect();
    let monke_business = top_monkes[0] * top_monkes[1];
    Ok(monke_business.to_string())
}

fn part2(_input: &str) -> Result<String> {
    Err("not implemented".into())
}

fn evaluate(operation: &Operation, operand: i32) -> i32 {
    match operation {
        Operation::Addition(x) => operand + x,
        Operation::Multiplication(x) => operand * x,
        Operation::Exponent2 => operand * operand,
    }
}

#[derive(Debug)]
enum Operation {
    Addition(i32),
    Multiplication(i32),
    Exponent2,
}

// (sic)
#[derive(Debug)]
struct Monke {
    id: u32,
    operation: Operation,
    test_divisible: i32,
    test_true_monke: u32,
    test_false_monke: u32,
}

fn parse_monke(src: &str) -> Result<(Monke, Vec<i32>)> {
    lazy_static! {
        static ref RE_ID: Regex = Regex::new(r"Monkey (\d+):").unwrap();
        static ref RE_ITEMS: Regex = Regex::new(r"items: (.+)$").unwrap();
        static ref RE_OPERATION: Regex = Regex::new(r"= old ([+*]) (\w*)").unwrap();
    }
    let mut lines = src.lines();

    let id: u32 = RE_ID
        .captures(lines.next().ok_or("missing line")?)
        .ok_or("id not found")?[1]
        .parse()?;

    let items: Vec<i32> = RE_ITEMS
        .captures(lines.next().ok_or("missing line")?)
        .ok_or("items not found")?[1]
        .split(", ")
        .flat_map(str::parse)
        .collect();

    let operation_captures = RE_OPERATION
        .captures(lines.next().ok_or("missing line")?)
        .ok_or("operation not found")?;
    let operation_result: Result<Operation> = match (&operation_captures[1], &operation_captures[2])
    {
        ("+", src) => Ok(Operation::Addition(src.parse()?)),
        ("*", "old") => Ok(Operation::Exponent2),
        ("*", src) => Ok(Operation::Multiplication(src.parse()?)),
        _ => Err("parsing operation failed".into()),
    };
    let operation = operation_result?;

    let test_divisible: i32 = lines
        .next()
        .ok_or("missing line")?
        .split(' ')
        .last()
        .ok_or("test divisible not found")?
        .parse()?;
    let test_true_monke: u32 = lines
        .next()
        .ok_or("missing line")?
        .split(' ')
        .last()
        .ok_or("test true monke not found")?
        .parse()?;
    let test_false_monke: u32 = lines
        .next()
        .ok_or("missing line")?
        .split(' ')
        .last()
        .ok_or("test false monke not found")?
        .parse()?;

    let monke = Monke {
        id,
        operation,
        test_divisible,
        test_true_monke,
        test_false_monke,
    };
    Ok((monke, items))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = concat!(
        "Monkey 0:\n",
        "  Starting items: 79, 98\n",
        "  Operation: new = old * 19\n",
        "  Test: divisible by 23\n",
        "    If true: throw to monkey 2\n",
        "    If false: throw to monkey 3\n",
        "\n",
        "Monkey 1:\n",
        "  Starting items: 54, 65, 75, 74\n",
        "  Operation: new = old + 6\n",
        "  Test: divisible by 19\n",
        "    If true: throw to monkey 2\n",
        "    If false: throw to monkey 0\n",
        "\n",
        "Monkey 2:\n",
        "  Starting items: 79, 60, 97\n",
        "  Operation: new = old * old\n",
        "  Test: divisible by 13\n",
        "    If true: throw to monkey 1\n",
        "    If false: throw to monkey 3\n",
        "\n",
        "Monkey 3:\n",
        "  Starting items: 74\n",
        "  Operation: new = old + 3\n",
        "  Test: divisible by 17\n",
        "    If true: throw to monkey 0\n",
        "    If false: throw to monkey 1"
    );

    #[test]
    fn part1_test() -> Result<()> {
        let actual = part1(INPUT)?;
        assert_eq!(actual, "10605");

        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        todo!()
    }
}
