
use std::{env, fs::File, io::{BufReader, BufRead}, error::Error};

struct Rucksack<'a> {
    s : &'a str
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

fn get_item_mask(s : &str) -> u64 {
    let mut mask : u64 = 0;
    // We know that only ascii chars are used here
    for c in s.bytes() {
        let b = to_priority(c);
        mask |= 1u64 << b;
    }
    mask
}

impl<'a> Rucksack<'a> {
    fn new(s : &'a str) -> Rucksack<'a> {
        assert!(s.len() % 2 == 0);
        Rucksack {
            s
        }
    }

    fn get_item_mask(&self) -> u64 {
        get_item_mask(self.s)
    }

    fn find_duplicate(&self) -> Option<u8> {
        let lhs : &str = &self.s[0..self.s.len() / 2];
        let rhs : &str = &self.s[(self.s.len() / 2)..self.s.len()];

        let contained = get_item_mask(lhs);

        for c in rhs.bytes() {
            let b = to_priority(c);
            let found = contained & (1u64 << b) != 0;
            if found {
                return Some(c)
            }
            
        }
        None
    }
}

const ELVES_IN_GROUP : usize = 3;

fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = env::args().collect();
    let path : &str = &args[1];
    let input_file = File::open(path)?;

    let lines = BufReader::new(input_file).lines();
    
    let mut total_dup_prios : u64 = 0;
    let mut total_badge_prios : u64 = 0;
    let mut group_count = 0;
    let mut curr_badge_mask = !0u64;
    for line in lines {
        if let Ok(l) = line {
            // Process each rucksack
            let sack = Rucksack::new(&l);
            let dup = sack.find_duplicate().expect("should always be a duplicate in a valid sack");
            let dup_prio : u64 = to_priority(dup).into();
            total_dup_prios += dup_prio;
            curr_badge_mask &= sack.get_item_mask();
            group_count = (group_count + 1) % ELVES_IN_GROUP;
            if group_count == 0 {
                let badge_prio : u64 = curr_badge_mask.trailing_zeros().into();
                total_badge_prios += badge_prio;
                curr_badge_mask = !0u64;
            }
        }
    }
    assert!(group_count == 0);
    println!("Total of priorities of duplicated sack items was {}", total_dup_prios);
    println!("Total of priorities of badges in each elf group was {}", total_badge_prios);
    Ok(())
}
