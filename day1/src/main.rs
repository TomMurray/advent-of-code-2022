use std::fs::File;

fn main() {
  const PATH : &str = "input.txt";
  let _input_file = File::open(PATH).unwrap();
}
