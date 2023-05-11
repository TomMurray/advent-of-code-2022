use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    env,
    error::Error,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
    mem::swap,
    ops::{Add, AddAssign, Sub},
};

use day12::MinHeapKeyValue;

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

fn pt1(terrain: &Vec<u8>, dims: Vec2, start: Vec2, end: Vec2) -> u32 {
    // Run djikstra's on the graph formed by coordinates in the terrain.
    let mut next = MinHeapKeyValue::new();

    let total_positions = dims.flatten();
    let mut visited = vec![false; total_positions];
    let mut distances = vec![u32::MAX; total_positions];

    let start_idx = dims.lin(&start);
    visited[start_idx] = true;
    distances[start_idx] = 0;
    next.insert(0u32, start_idx);

    let end_idx = dims.lin(&end);

    while let Some((distance, idx)) = next.pop() {
        if idx == end_idx {
            // We have found the shortest path, break
            break;
        }
        visited[idx] = true;
        let coord = dims.from_lin(idx);

        for delta in [Vec2::up(), Vec2::right(), Vec2::down(), Vec2::left()] {
            // If the height differential is greater than +1, we can't go in that
            // direction
            let neighbour = coord + delta;
            if neighbour.x < 0 || neighbour.x >= dims.x || neighbour.y < 0 || neighbour.y >= dims.y
            {
                continue;
            }
            let neighbour_idx = dims.lin(&neighbour);
            if terrain[idx] + 1 < terrain[neighbour_idx] {
                continue;
            }
            if visited[neighbour_idx] {
                continue;
            }
            // Otherwise update neighbour's distance, and add it to the set to visit
            let new_distance = distance + 1;
            let neighbour_distance = &mut distances[neighbour_idx];
            if new_distance < *neighbour_distance {
                *neighbour_distance = new_distance;
                next.insert_or_decrease_key(neighbour_idx, new_distance);
            }
        }
    }
    distances[end_idx]
}

fn pt2(terrain: &Vec<u8>, dims: Vec2, starts: &Vec<Vec2>, end: Vec2) -> u32 {
    // Make the start points into a set
    let mut end_idxs: HashSet<usize> = starts.into_iter().map(|x| dims.lin(&x)).collect();

    // We'll actually start from the end point, and walk towards the start points.
    let mut next = MinHeapKeyValue::new();

    let mut visited = vec![false; dims.flatten()];
    let mut distances = vec![u32::MAX; dims.flatten()];

    let start_idx = dims.lin(&end);
    visited[start_idx] = true;
    distances[start_idx] = 0;
    next.insert(0u32, start_idx);

    let mut min_distance = u32::MAX;
    while let Some((distance, idx)) = next.pop() {
        if end_idxs.remove(&idx) {
            // Update the min distance
            if distances[idx] < min_distance {
                min_distance = distances[idx];
            }
            // Finish if the set of end indices is empty
            if end_idxs.is_empty() {
                break;
            }
        }
        visited[idx] = true;
        let coord = dims.from_lin(idx);

        for delta in [Vec2::up(), Vec2::right(), Vec2::down(), Vec2::left()] {
            let neighbour = coord + delta;
            if neighbour.x < 0 || neighbour.y < 0 || neighbour.x >= dims.x || neighbour.y >= dims.y
            {
                continue;
            }
            let neighbour_idx = dims.lin(&neighbour);
            if terrain[neighbour_idx] + 1 < terrain[idx] {
                continue;
            }

            if visited[neighbour_idx] {
                continue;
            }

            // Otherwise update neighbour's distance, and add it to the set to visit
            let new_distance = distance + 1;
            let neighbour_distance = &mut distances[neighbour_idx];
            if new_distance < *neighbour_distance {
                *neighbour_distance = new_distance;
                next.insert_or_decrease_key(neighbour_idx, new_distance);
            }
        }
    }

    min_distance
}

fn main() -> Result<(), Box<dyn Error>> {
    // Basically, parse the input into an array of integer heights, perform A* pathfinding where
    // each position is a node in a graph, and a node is connected to another node if it is
    // neighbouring and the height change is no more than 1.

    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let input = File::open(input)?;

    let mut terrain = vec![];
    let mut dims = Vec2::new();
    let (mut start, mut end) = (Vec2::new(), Vec2::new());

    for (y, line) in BufReader::new(input).lines().enumerate() {
        let line = line?;
        dims.x = line.len().try_into().unwrap();

        for (x, mut c) in line.bytes().enumerate() {
            if c == 'S' as u8 {
                start = Vec2 {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };
                c = 'a' as u8;
            } else if c == 'E' as u8 {
                end = Vec2 {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };
                c = 'z' as u8;
            }
            assert!(c >= 'a' as u8 && c <= 'z' as u8);
            terrain.push(c - 'a' as u8);
        }
    }

    dims.y = TryInto::<i32>::try_into(terrain.len()).unwrap() / dims.x;

    let pt1_distance = pt1(&terrain, dims, start, end);
    println!(
        "Shortest distance from start to finish calculated as {}",
        pt1_distance
    );

    // Part 2
    let mut starts = vec![];
    for idx in 0..dims.flatten() {
        if terrain[idx] == 0 {
            starts.push(dims.from_lin(idx));
        }
    }

    let pt2_distance = pt2(&terrain, dims, &starts, end);
    println!(
        "Shortest distance from any point with height 'a' to finish calculated as {}",
        pt2_distance
    );

    Ok(())
}
