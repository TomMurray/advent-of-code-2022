use std::{
    cmp::Reverse,
    env,
    error::Error,
    fmt,
    fs::File,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

use regex::Regex;

// Represents an operand in an operation
#[derive(Copy, Clone, Debug)]
enum Operand {
    Literal(i64),
    Old, // symbolic
}

impl FromStr for Operand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Old),
            _ => Ok(Operand::Literal(s.parse()?)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum OpType {
    Add,
    Mul,
}

impl FromStr for OpType {
    type Err = MonkeyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(OpType::Add),
            "*" => Ok(OpType::Mul),
            _ => Err(MonkeyParseError),
        }
    }
}

#[derive(Clone, Debug)]
struct Operation {
    lhs: Operand,
    rhs: Operand,
    optype: OpType,
}

type MonkeyID = usize;

struct Monkey {
    items: Vec<i64>,
    op: Operation,
    test: i64,
    outs: [MonkeyID; 2],
}

#[derive(Clone, Debug)]
struct MonkeyParseError;

impl fmt::Display for MonkeyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Problem parsing monkey")
    }
}

impl Error for MonkeyParseError {}

// Function to parse a single starting state for a monkey
fn parse_monkey<I>(line_it: &mut I) -> Result<Monkey, Box<dyn Error>>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    let line = line_it.next().unwrap()?;
    let (id, items) = line.split_once(':').unwrap();
    if id.trim() != "Starting items" {
        return Err(Box::new(MonkeyParseError));
    }

    let items = items.trim().split(", ");
    // This magic seems to be collecting the first error when
    // collecting the results from iteration, if there is one.
    let items = items
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let line = line_it.next().unwrap()?;
    let re = Regex::new(r"Operation: new = (old|new|[0-9]+) ([\+*]) (old|new|[0-9]+)").unwrap();
    let captures = re.captures(&line).unwrap();
    let lhs: Operand = captures.get(1).unwrap().as_str().parse()?;
    let rhs: Operand = captures.get(3).unwrap().as_str().parse()?;
    let optype: OpType = captures.get(2).unwrap().as_str().parse()?;
    let op = Operation { lhs, rhs, optype };

    let line = line_it.next().unwrap()?;
    let captures = Regex::new(r"Test: divisible by ([0-9]+)")
        .unwrap()
        .captures(&line)
        .unwrap();
    let test: i64 = captures.get(1).unwrap().as_str().parse()?;
    let line = line_it.next().unwrap()?;
    let captures = Regex::new(r"If true: throw to monkey ([0-9]+)")
        .unwrap()
        .captures(&line)
        .unwrap();
    let monkey1: MonkeyID = captures.get(1).unwrap().as_str().parse()?;
    let line = line_it.next().unwrap()?;
    let captures = Regex::new(r"If false: throw to monkey ([0-9]+)")
        .unwrap()
        .captures(&line)
        .unwrap();
    let monkey0: MonkeyID = captures.get(1).unwrap().as_str().parse()?;

    Ok(Monkey {
        items,
        op: op,
        test: test,
        outs: [monkey0, monkey1],
    })
}

fn get_monkey_triplet(v: &mut [Monkey], idx: MonkeyID) -> (&mut Monkey, [&mut Monkey; 2]) {
    unsafe {
        let m = &mut v[idx];
        let idx0 = m.outs[0];
        let idx1 = m.outs[1];
        assert!(idx != idx0 && idx != idx1 && idx0 != idx1);
        let m = m as *mut Monkey;

        let out0 = &mut v[idx0] as *mut Monkey;
        let out1 = &mut v[idx1] as *mut Monkey;
        (&mut *m, [&mut *out0, &mut *out1])
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(nums: &[i64]) -> i64 {
    let a = nums[0];
    if nums.len() == 1 {
        return a;
    }
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

const PART1: bool = false;

fn round(v: &mut [Monkey], c: &mut [usize], wrap_to: i64) {
    for idx in 0..v.len() {
        let (from, outs) = get_monkey_triplet(v, idx);

        // Apply the update to each item in order
        for item in &mut from.items {
            *item %= wrap_to;

            let lhs = match from.op.lhs {
                Operand::Literal(val) => val,
                Operand::Old => *item,
            };
            let rhs = match from.op.rhs {
                Operand::Literal(val) => val,
                Operand::Old => *item,
            };
            *item = match from.op.optype {
                OpType::Add => lhs + rhs,
                OpType::Mul => lhs * rhs,
            };

            if PART1 {
                *item /= 3;
            }

            // Update count for this monkey
            c[idx] += 1;
        }
        for item in &from.items {
            let test = (item % from.test == 0) as usize;
            outs[test].items.push(*item);
        }
        from.items.clear();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let input = File::open(input)?;

    let mut lines = BufReader::new(input).lines();
    let mut monkeys = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let monkey_idx: MonkeyID = Regex::new(r"Monkey ([0-9]+):")
            .unwrap()
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()?;
        assert!(monkey_idx == monkeys.len());
        monkeys.push(parse_monkey(&mut lines)?);
    }

    let wrap_to = lcm(&(&monkeys).into_iter().map(|m| m.test).collect::<Vec<i64>>());

    let mut inspection_counts: Vec<usize> = vec![0usize; monkeys.len()];
    const NUM_ROUNDS: usize = if PART1 { 20 } else { 10000 };
    for _ in 0..NUM_ROUNDS {
        round(&mut monkeys, &mut inspection_counts, wrap_to);
    }

    println!(
        "Inspection counts after round {} were: {:?}",
        NUM_ROUNDS, inspection_counts
    );

    // Get top 2 monkeys in terms of activity. We'll do this dumbly with a sort
    let mut sorted = inspection_counts;
    sorted.sort_by_key(|x| Reverse(*x));

    let top2 = &sorted[0..2];
    println!(
        "Top 2 inspection counts were {:?} for a product of {}",
        top2,
        top2.into_iter().product::<usize>()
    );

    Ok(())
}
