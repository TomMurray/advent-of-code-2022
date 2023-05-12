use std::{error::Error, env, fs::File, io::{BufReader, BufRead}, ops::{Add, AddAssign, Sub}};

// Lazily copied from day 12
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
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

    fn lin(&self, other: &Self) -> usize {
        assert!(other.x < self.x && other.y < self.y);
        (other.y * self.x + other.x).try_into().unwrap()
    }

    fn from_lin(&self, f: usize) -> Self {
        let f: i32 = f.try_into().unwrap();
        assert!(f < self.x * self.y);
        Self {
            x: f % self.x,
            y: f / self.x,
        }
    }

    fn flatten(&self) -> usize {
        (self.x * self.y).try_into().unwrap()
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

fn parse_coord(s : &mut &str) -> Result<Vec2, Box<dyn Error>> {
    let comma_idx = s.find(',').unwrap();
    let end_coord_idx = s.find(' ').unwrap_or(s.len());
    let x_coord : i32 = s[0..comma_idx].parse()?;
    let y_coord : i32 = s[comma_idx + 1..end_coord_idx].parse()?;
    *s = &s[end_coord_idx..];
    Ok(Vec2{ x : x_coord, y : y_coord })
}


fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = env::args().collect();
    let path = &args[1];
    let file = File::open(path)?;

    let mut lines = vec![];

    for line in BufReader::new(file).lines() {
        let line = line?;

        let mut points = vec![];

        let mut slice = &line[0..];
        points.push(parse_coord(&mut slice)?);
        while !slice.is_empty() {
            // Expect an arrow if the string isn't empty
            assert!(&slice[..4] == " -> ");
            slice = &slice[4..];

            points.push(parse_coord(&mut slice)?);
        }

        lines.push(points);
    }

    println!("Parsed lines:");
    for line in &lines {
        println!("  {:?}", line);
    }

    Ok(())
}
