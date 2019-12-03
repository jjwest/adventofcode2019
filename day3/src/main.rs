use std::collections::{HashSet, HashMap};
use std::env;
use std::error::Error;
use std::fs;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Copy, Clone)]
struct Move {
    dir: Direction,
    steps: i32,
}

impl FromStr for Move {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> std::result::Result<Move, Self::Err> {
        let dir = match s.chars().nth(0).unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            other => return Err(From::from(format!("{} is not a valid direction", other))),
        };

        let steps = s[1..].parse()?;

        Ok(Move { dir, steps })
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct PosIterator<'a> {
    at: Point,
    next: Move,
    moves: &'a [Move],
}

impl<'a> PosIterator<'a> {
    fn new(moves: &'a [Move]) -> Self {
        Self {
            at: Point::new(0, 0),
            next: moves[0],
            moves
        }
    }
}

impl<'a> Iterator for PosIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.steps == 0 {
            if self.moves.len() > 1 {
                self.moves = &self.moves[1..];
                self.next = self.moves[0];
            } else {
                return None;
            }
        }

        match self.next.dir {
            Direction::Up => self.at.y += 1,
            Direction::Down => self.at.y -= 1,
            Direction::Left => self.at.x -= 1,
            Direction::Right => self.at.x += 1,
        }

        self.next.steps -= 1;
        Some(self.at)
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
    let mut lines = input.lines();

    let mut wire0: Vec<Move> = Vec::new();
    let line0 = lines.next().unwrap();
    for m in line0.trim().split(",") {
        wire0.push(m.parse()?);
    }


    let mut wire1: Vec<Move> = Vec::new();
    let line1 = lines.next().unwrap();
    for m in line1.trim().split(",") {
        wire1.push(m.parse()?);
    }

    part1(&wire0, &wire1);
    part2(&wire0, &wire1);

    Ok(())
}

fn part1(wire0: &[Move], wire1: &[Move]) {
    let wire0_positions: HashSet<Point> = PosIterator::new(&wire0).collect();
    let mut shortest_distance = std::i32::MAX;

    for pos in PosIterator::new(&wire1) {
        if wire0_positions.contains(&pos) {
            let d = pos.x.abs() + pos.y.abs();
            if d < shortest_distance {
                shortest_distance = d;
            }
        }
    }

    println!("part1: {}", shortest_distance);
}

fn part2(wire0: &[Move], wire1: &[Move]) {
    let wire0_footprints = get_footprints(wire0);
    let wire1_footprints = get_footprints(wire1);

    let mut min_distance = std::usize::MAX;

    for (key, val0) in &wire0_footprints {
        if let Some(val1) = wire1_footprints.get(key) {
            let d = val0 + val1;
            if d < min_distance {
                min_distance = d;
            }
        }
    }

    println!("part2: {}", min_distance);
}

fn get_footprints(wire: &[Move]) -> HashMap<Point, usize> {
    let mut footprints = HashMap::new();

    for (i, pos) in PosIterator::new(wire).enumerate() {
        if !footprints.contains_key(&pos) {
            footprints.insert(pos, i + 1);
        }
    }

    footprints
}
