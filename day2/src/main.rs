use std::env;
use std::error::Error;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

enum Instruction {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt,
}

impl From<&[i32]> for Instruction {
    fn from(v: &[i32]) -> Self {
        match v[0] {
            1 => Instruction::Add(v[1] as usize, v[2] as usize, v[3] as usize),
            2 => Instruction::Mul(v[1] as usize, v[2] as usize, v[3] as usize),
            99 => Instruction::Halt,
            _ => panic!("Illegal opcode"),
        }
    }
}

fn main() -> Result<()> {
    let file = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            eprintln!("usage: {} FILE", env::args().nth(0).unwrap());
            std::process::exit(1);
        }
    };

    let input = fs::read_to_string(&file)?;
    let mut source: Vec<i32> = Vec::new();

    for value in input.trim().split(",") {
        source.push(value.parse()?);
    }

    part1(source.clone());
    part2(source.clone());

    Ok(())
}

fn part1(mut memory: Vec<i32>) {
    memory[1] = 12;
    memory[2] = 2;
    execute(&mut memory);
    println!("part1: {}", memory[0]);
}

fn part2(original: Vec<i32>) {
    let target_output = 19690720;
    let mut memory = original.clone();

    for i in 0..100 {
        for j in 0..100 {
            memory[1] = i;
            memory[2] = j;
            execute(&mut memory);

            if memory[0] == target_output {
                println!("part2: {}", 100 * memory[1] + memory[2]);
                return;
            }

            memory.copy_from_slice(&original[..]);
        }
    }
}

fn execute(source: &mut [i32]) {
    for i in (0..source.len()).step_by(4) {
        let opcode = Instruction::from(&source[i..]);
        match opcode {
            Instruction::Add(in0, in1, out) => source[out] = source[in0] + source[in1],
            Instruction::Mul(in0, in1, out) => source[out] = source[in0] * source[in1],
            Instruction::Halt => break,
        }
    }
}
