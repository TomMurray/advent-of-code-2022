use std::{env, fs::File, error::Error, io::{BufReader, BufRead}, iter::Iterator};

struct Stacks(Vec<Vec<char>>);

impl Stacks {
}


fn parse_stacks<It>(lines : &mut It) -> Stacks 
where
  It : Iterator 
{
    let mut contents: Vec<Vec<char>> = Vec::new();
    while let Some(line) = lines.next() {
        // Parse each line while it matches a certain pattern
        
    }
    Stacks (contents)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = env::args().collect();
    let path = &args[1];
    let file = File::open(path)?;

    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines().into_iter();

    // First parse the initial state of the stacks
    let mut stacks = parse_stacks(&mut lines);

    // Then parse the instructions
    
    Ok(())
}
