use std::fs::File;
use std::error::Error;
use std::env;
use std::path::Path;
use std::io::{self, BufRead};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P : AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

const K : usize = 3;

// We call this continuously for each input, and it updates the data structure.
// We choose to do 'find the smallest in an unsorted vector and replace that'
// approach because K is very small here. Should definetely be factored out
// into its own struct!
fn top_small_k_inputstream(top : &mut [i32; K], n : &mut usize, next : i32) -> Result<(), Box<dyn Error>> {
  if *n < K {
    // just add it to the list
    top[*n] = next;
    *n += 1;
  } else {
    // Replace smallest of the values
    let curr_smallest = top.iter_mut().min().expect("Should be non-zero number of elements in array");
    if *curr_smallest < next {
      *curr_smallest = next;
    }
  }
  Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
  let args : Vec<String> = env::args().collect();
  let path = &args[1];
  let mut max_calorie_counts = [0, 0, 0];
  let mut n : usize = 0;
  {
    let mut calorie_count : i32 = 0;
    if let Ok(lines) = read_lines(path) {
      for line in lines {
        if let Ok(s) = line {
          if s.is_empty() {
            top_small_k_inputstream(& mut max_calorie_counts, &mut n, calorie_count)?;
            calorie_count = 0;
          } else {
            calorie_count += s.parse::<i32>().unwrap();
          }
        }
      }
    }
  }
  let mut total : i32 = 0;
  for i in 0..n {
    println!("#{} had {} calories", i, max_calorie_counts[i]);
    total += max_calorie_counts[i];
  }
  println!("Sum of those calories is {}", total);
  Ok(())
}
