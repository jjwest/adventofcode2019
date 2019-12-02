use std::env;
use std::error::Error;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let file = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            eprintln!("usage: {} FILE", env::args().nth(0).unwrap());
            std::process::exit(1);
        }
    };

    let input = fs::read_to_string(&file)?;

    let fuel = part1(&input)?;
    println!("Part 1: {}", fuel);

    let fuel = part2(&input)?;
    println!("Part 2: {}", fuel);

    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    let mut sum_fuel = 0;

    for line in input.split_whitespace() {
        let mass: i32 = line.parse()?;
        sum_fuel += (mass / 3) - 2;
    }

    Ok(sum_fuel)
}

fn part2(input: &str) -> Result<i32> {
    let mut total_sum = 0;

    for line in input.split_whitespace() {
        let mut mass: i32 = line.parse()?;
        let mut sum = 0;

        loop {
            let added = (mass / 3) - 2;
            if added <= 0 {
                break;
            }
            sum += added;
            mass = added;
        }

        total_sum += sum;
    }


    Ok(total_sum)
}
