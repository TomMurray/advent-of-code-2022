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
    let marker_len: usize = args[2].parse()?;
    // Read in the input in one chunk. We have a single line input to parse.
    let input: Vec<u8> = fs::read_to_string(input)?.bytes().collect();

    // The method is very simple. We assume the window length is not too long
    // and we just build a bitset each time and count the bits in order to
    // determine if we have found the marker
    if input.len() < marker_len {
        return Err(Box::new(InputTooShortError));
    }
    assert!(input.len() >= marker_len);
    let mut found_idx: Option<usize> = None;
    'outer: for idx in 0..input.len() - marker_len {
        let slice = &input[idx..idx + marker_len];
        let mut bits: u32 = 0;
        for c in slice {
            let prev = bits;
            bits |= 1u32 << to_idx(*c);
            if bits == prev {
                // If bits didn't change, we know there were duplicate letters so break
                continue 'outer;
            }
        }
        found_idx = Some(idx);
        break;
    }
    if let Some(idx) = found_idx {
        println!("Found marker in range {}..{}", idx, idx + marker_len);
    } else {
        return Err(Box::new(MarkerNotFoundError));
    }

    Ok(())
}
