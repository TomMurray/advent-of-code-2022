use std::{env, error::Error};
use std::{fmt, fs};

fn to_idx(c: u8) -> usize {
    assert!(c >= 'a' as u8 && c <= 'z' as u8);
    (c - ('a' as u8)) as usize
}

#[derive(Debug, Clone)]
struct InputTooShortError;
impl Error for InputTooShortError {}

impl fmt::Display for InputTooShortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No marker found because input was < 4 characters long")
    }
}

#[derive(Debug, Clone)]
struct MarkerNotFoundError;
impl Error for MarkerNotFoundError {}

impl fmt::Display for MarkerNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Marker not found in input")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    // Read in the input in one chunk. We have a single line input to parse.
    let input: Vec<u8> = fs::read_to_string(input)?.bytes().collect();

    // As we have a window of just 4 characters it actually seems very reasonable
    // to just build a bitset at each starting character

    if input.len() < 4 {
        return Err(Box::new(InputTooShortError));
    }
    assert!(input.len() >= 4);
    let mut found_idx: Option<usize> = None;
    'outer: for idx in 0..input.len() - 4 {
        let slice = &input[idx..idx + 4];
        let mut bits: u32 = 0;
        for c in slice {
            bits |= 1u32 << to_idx(*c);
        }
        if bits.count_ones() == 4 as u32 {
            // We found the marker
            found_idx = Some(idx);
            break 'outer;
        }
    }
    if let Some(idx) = found_idx {
        println!("Found marker in range {}..{}", idx, idx + 4);
    } else {
        return Err(Box::new(MarkerNotFoundError));
    }

    Ok(())
}
