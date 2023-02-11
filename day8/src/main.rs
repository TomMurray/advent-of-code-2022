use std::{fmt::{Debug, self}, fs::{File, self}, error::Error, env, str::FromStr};

struct Matrix<T> {
    storage : Vec<T>,
    inner_dim : usize
}

impl Matrix<u32> {
    fn from_iter<'a, InnerIterator : Iterator<Item = char>, I : Iterator<Item = InnerIterator>>(iter : I) -> Self {
        let mut storage = vec![];
        let mut row_len: Option<usize> = None;
        for row in iter {
            if let None = row_len {
                let mut entry_count: usize = 0;
                for value in row {
                    storage.push(value.to_digit(10).unwrap());
                    entry_count += 1;
                }
                row_len = Some(entry_count);
            } else {
                for value in row {
                    storage.push(value.to_digit(10).unwrap());
                }
            }
        }
        Self{ storage, inner_dim: row_len.unwrap() }
    }
}

impl<T> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  TODO: fmt::Display for Matrix<T>")
    }
} 

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let args : Vec<String> = env::args().collect();
    let input = &args[1];
    let input = fs::read_to_string(input)?;
    
    let mat = Matrix::from_iter(input.lines().map(|x| x.chars()));

    println!("Initial matrix:\n{}", mat);

    Ok(())
}
