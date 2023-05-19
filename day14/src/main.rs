use std::{
    cmp::{self, min},
    env,
    error::Error,
    fmt::{self, Write},
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign, Sub},
    thread, time,
};

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
        Self { x: 0, y: -1 }
    }
    fn down() -> Self {
        Self { x: 0, y: 1 }
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

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", &self.x, &self.y)
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

struct Sim {
    v: Vec<char>,
    dims: Vec2,
    // Sim does not necessarily start at coordinates (0,0).
    // The following gives an offset for the simulated area
    // in coordinate space.
    offset: Vec2,
}

impl Sim {
    fn new(dims: Vec2, offset: Vec2) -> Self {
        Self {
            v: vec!['.'; dims.flatten()],
            dims,
            offset,
        }
    }

    fn add_line(&mut self, line: &Vec<Vec2>) {
        // Start by marking the first coordinate alone,
        // then iterate the remaining points.
        let mut first = line.first();

        // Shouldn't ever be the case but...
        if let Some(first) = first {
            self.v[self.dims.lin(&(*first - self.offset))] = '#';
        } else {
            return;
        }

        for idx in 1..line.len() {
            let begin = line[idx - 1] - self.offset;
            let end = line[idx] - self.offset;

            // Lines are always vertical or horizontal
            let delta = (end - begin).signum();
            // Checks that lines are vertical or horizontal and not both.
            assert!(delta.x + delta.y != 0 && delta.x * delta.y == 0);

            let mut curr = begin;
            while curr != end {
                curr = curr + delta;
                self.v[self.dims.lin(&curr)] = '#';
            }
        }
    }

    fn get(&self, coord: Vec2) -> Option<char> {
        let coord = coord - self.offset;
        if coord.x < 0 || coord.x >= self.dims.x || coord.y < 0 || coord.y >= self.dims.y {
            return None;
        }

        Some(self.v[self.dims.lin(&coord)])
    }

    fn set(&mut self, coord: Vec2, val: char) {
        let offset_coord = coord - self.offset;
        self.v[self.dims.lin(&offset_coord)] = val;
    }
}

impl fmt::Display for Sim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Start coordinates
        writeln!(f, "{}", &self.offset)?;
        for y in 0..self.dims.y {
            for x in 0..self.dims.x {
                f.write_char(self.v[self.dims.lin(&Vec2 { x, y })])?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn parse_coord(s: &mut &str) -> Result<Vec2, Box<dyn Error>> {
    let comma_idx = s.find(',').unwrap();
    let end_coord_idx = s.find(' ').unwrap_or(s.len());
    let x_coord: i32 = s[0..comma_idx].parse()?;
    let y_coord: i32 = s[comma_idx + 1..end_coord_idx].parse()?;
    *s = &s[end_coord_idx..];
    Ok(Vec2 {
        x: x_coord,
        y: y_coord,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
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

    // Figure out the bounds of our terrain
    let mut min = Vec2 {
        x: i32::MAX,
        y: i32::MAX,
    };
    let mut max = Vec2 {
        x: i32::MIN,
        y: i32::MIN,
    };
    for line in &lines {
        for point in line {
            min.x = cmp::min(min.x, point.x);
            min.y = cmp::min(min.y, point.y);
            max.x = cmp::max(max.x, point.x);
            max.y = cmp::max(max.y, point.y);
        }
    }

    // Note that sand will fall in from (500, 0) so we must
    // include this in our bounds.
    const SPAWN: Vec2 = Vec2 { x: 500, y: 0 };
    min.x = cmp::min(min.x, SPAWN.x);
    min.y = cmp::min(min.y, SPAWN.y);
    max.x = cmp::max(max.x, SPAWN.x);
    max.y = cmp::max(max.y, SPAWN.y);

    max.x += 1;
    max.y += 1;

    // Setup the sim
    let mut sim = Sim::new(max - min, min);

    println!("Before adding lines:\n{}", &sim);
    for line in &lines {
        sim.add_line(&line);
    }
    println!("After adding lines:\n{}", &sim);

    // There are 2 states to the simulation, when we're dropping a
    // block of sand in, it is the only thing that moves until it
    // either settles or falls off the edge.
    let mut settled_count = 0;
    'outer: loop {
        let mut next_coord = SPAWN;

        // Iterate until at rest or off-screen
        loop {
            // Try immediately below
            let immediately_below = next_coord + Vec2::down();
            if let Some(content) = sim.get(immediately_below) {
                if content == '.' {
                    next_coord = immediately_below;
                    continue;
                }
            } else {
                break 'outer;
            }
            // Try below and to the left
            let to_the_left = next_coord + Vec2::down() + Vec2::left();
            if let Some(content) = sim.get(to_the_left) {
                if content == '.' {
                    next_coord = to_the_left;
                    continue;
                }
            } else {
                break 'outer;
            }

            // Try below and to the right
            let to_the_right = next_coord + Vec2::down() + Vec2::right();
            if let Some(content) = sim.get(to_the_right) {
                if content == '.' {
                    next_coord = to_the_right;
                    continue;
                }
            } else {
                break 'outer;
            }

            // If none of the above were air, the sand comes to rest.
            sim.set(next_coord, 'o');
            settled_count += 1;
            break;
        }
    }

    println!(
        "Final grid:\n{}\nUnits of sand that came to rest before dropping into the void: {}",
        &sim, &settled_count
    );

    Ok(())
}
