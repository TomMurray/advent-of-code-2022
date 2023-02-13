use std::{
    collections::HashSet,
    env,
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct ParseDirectionErr(String);

impl fmt::Display for ParseDirectionErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse direction token '{}'", self.0)
    }
}

impl Error for ParseDirectionErr {}

impl FromStr for Direction {
    type Err = ParseDirectionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(ParseDirectionErr(String::from(s))),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new() -> Self {
        Vec2 { x: 0, y: 0 }
    }

    fn signum(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    fn zero() -> Self {
        Vec2::new()
    }

    fn up() -> Self {
        Self { x: 0, y: 1 }
    }
    fn down() -> Self {
        Self { x: 0, y: -1 }
    }

    fn left() -> Self {
        Self { x: -1, y: 0 }
    }

    fn right() -> Self {
        Self { x: 1, y: 0 }
    }
}

fn dir_to_delta(d: Direction) -> Vec2 {
    match d {
        Direction::Up => Vec2::up(),
        Direction::Down => Vec2::down(),
        Direction::Right => Vec2::right(),
        Direction::Left => Vec2::left(),
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let input = File::open(input)?;

    let num_nodes = &args[2];
    let num_nodes: usize = num_nodes.parse()?;

    // Initial state
    // 'head' is the first in the list, tail nodes are higher indices
    let mut positions = vec![Vec2::new(); num_nodes];

    let mut visited_positions = HashSet::new();
    // Initial visited position - always the last tail node
    visited_positions.insert(*positions.last().unwrap());

    // Read and process instructions
    for line in BufReader::new(input).lines() {
        let line = line.unwrap();
        // Each input should consist of a direction followed by a number
        // of steps in that direction
        let (dir, num_steps) = line.split_once(' ').unwrap();
        let dir = dir.parse()?;
        let num_steps: i32 = num_steps.parse()?;
        let step_delta = dir_to_delta(dir);

        for _ in 0..num_steps {
            let mut delta = step_delta;

            for idx in 0..positions.len() - 1 {
                positions[idx] += delta;

                // Distance between this and next node in the chain
                let distance = positions[idx] - positions[idx + 1];
                if distance.x.abs() > 1 || distance.y.abs() > 1 {
                    delta = distance.signum();
                } else {
                    delta = Vec2::new();
                    // We can exit early here if we didn't move
                    break;
                }
            }

            // If the last tail node should move, move it and update visited positions
            if delta != Vec2::zero() {
                *positions.last_mut().unwrap() += delta;
                visited_positions.insert(*positions.last().unwrap());
            }
        }
    }

    println!("No. of visited positions was {}", visited_positions.len());

    Ok(())
}
