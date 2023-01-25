use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    ops::Range,
};

fn read_lines(f: File) -> Lines<BufReader<File>> {
    BufReader::new(f).lines()
}

fn parse_range(s: &str) -> Range<u32> {
    let (lower, upper) = s.split_once('-').expect("Ill-formed input");
    Range::<u32> {
        start: lower.parse::<u32>().unwrap(),
        end: upper.parse::<u32>().unwrap() + 1,
    }
}

fn parse_pair(s: &str) -> (Range<u32>, Range<u32>) {
    let (lhs, rhs) = s.split_once(',').expect("Ill-formatted input");
    (parse_range(lhs), parse_range(rhs))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let file = File::open(path)?;

    let mut fully_contained = 0;
    let mut overlapping = 0;
    for line in read_lines(file) {
        let line = line.expect("There shouldn't be empty lines in the input as far as I'm aware");
        let (lhs, rhs) = parse_pair(&line);

        if (lhs.start <= rhs.start && lhs.end >= rhs.end)
            || (rhs.start <= lhs.start && rhs.end >= lhs.end)
        {
            fully_contained += 1;
        }

        if lhs.start < rhs.end && lhs.end > rhs.start {
            overlapping += 1;
        }
    }

    println!(
        "Number of pairs where one elf's assignments fully contain the others is {}",
        fully_contained
    );
    println!(
        "Number of pairs where one elf's assignments overlap the others is {}",
        overlapping
    );
    Ok(())
}
