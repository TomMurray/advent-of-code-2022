
use std::{env, fs::File, io::{BufReader, BufRead}, error::Error};

struct Rucksack<'a> {
    a : &'a str,
    b : &'a str
}


fn to_priority(c : u8) -> u8 {
    if c >= 'a' as u8 && c <= 'z' as u8 {
        return c - 'a' as u8 + 1
    } else if c >= 'A' as u8 && c <= 'Z' as u8 {
        return c - 'A' as u8 + 27
    } else {
        panic!("Unexpected rucksack content {}", c);
    }
}

impl<'a> Rucksack<'a> {
    fn new(s : &'a str) -> Rucksack<'a> {
        assert!(s.len() % 2 == 0);
        Rucksack {
            a: &s[0..s.len() / 2],
            b: &s[s.len() / 2..s.len()]
        }
    }

    fn find_duplicate(&self) -> Option<u8> {
        let mut contained : u64 = 0;
        // We know that only ascii chars are used here
        for c in self.a.bytes() {
            let b = to_priority(c);
            contained |= 1u64 << b;
        }

        for c in self.b.bytes() {
            let b = to_priority(c);
            let found = contained & (1u64 << b) != 0;
            if found {
                return Some(c)
            }
            
        }
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = env::args().collect();
    let path : &str = &args[1];
    let input_file = File::open(path)?;

    let lines = BufReader::new(input_file).lines();
    
    let mut total_prios : u64 = 0;
    for line in lines {
        if let Ok(l) = line {
            // Process each rucksack
            let sack = Rucksack::new(&l);
            let dup = sack.find_duplicate().expect("should always be a duplicate in a valid sack");
            let prio : u64 = to_priority(dup).into();
            println!("Found duplicate in sack {} with priority {}", dup, prio);
            total_prios += prio;
        }
    }
    println!("Total of priorities of duplicated sack items was {}", total_prios);
    Ok(())
}