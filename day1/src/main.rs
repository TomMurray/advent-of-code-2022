use std::fs::File;

fn main() {
  const PATH : &str = "input.txt";
  let _input_file = match File::open(PATH) {
    Ok(file) => { println!("Successfully opened {}", PATH); file},
    Err(error) => panic!("Could not open {}: {:?}", PATH, error)
  };
}
