use bit_set::BitSet;
use itertools::Itertools;
use std::{
    env,
    error::Error,
    fmt::{self},
    fs::{self},
    iter::once,
    ops::{Index, Mul},
};

struct Matrix<T> {
    storage: Vec<T>,
    inner_dim: usize,
}

impl<T> Matrix<T> {
    fn width(&self) -> usize {
        self.inner_dim
    }

    fn height(&self) -> usize {
        self.storage.len() / self.inner_dim
    }

    fn num_elements(&self) -> usize {
        self.storage.len()
    }
}

impl Matrix<u32> {
    fn from_iter<'a, InnerIterator: Iterator<Item = char>, I: Iterator<Item = InnerIterator>>(
        iter: I,
    ) -> Self {
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
        Self {
            storage,
            inner_dim: row_len.unwrap(),
        }
    }
}

impl<T> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  TODO: fmt::Display for Matrix<T>")
    }
}

impl<T, Idx> Index<Idx> for Matrix<T>
where
    Idx: Mul + Into<usize>,
{
    type Output = [T];

    fn index(&self, index: Idx) -> &Self::Output {
        let idx_cast: usize = index.into();
        &self.storage[idx_cast * self.inner_dim..(idx_cast + 1) * self.inner_dim]
    }
}

// This function has really been the bane of solving this problem in a succinct way.
// I'm not sure what the rust way of solving this problem is but I should go look at
// some other solutions. Perhaps more generically making this a pair of ranges with
// step?
type InnerBox<'a> = Box<dyn Iterator<Item = (usize, usize)> + 'a>;
fn get_iter<'a, T>(
    m: &'a Matrix<T>,
    column_major: bool,
    reversed: bool,
) -> Box<dyn Iterator<Item = InnerBox<'a>> + 'a> {
    let xmax = m.width();
    let ymax = m.height();

    if column_major {
        let outer_range = 0..xmax;
        if reversed {
            Box::new(
                outer_range.map(move |x| {
                    Box::new((0..ymax).rev().cartesian_product(once(x))) as InnerBox<'a>
                }),
            )
        } else {
            Box::new(
                outer_range
                    .map(move |x| Box::new((0..ymax).cartesian_product(once(x))) as InnerBox<'a>),
            )
        }
    } else {
        let outer_range = 0..ymax;
        if reversed {
            Box::new(
                outer_range.map(move |y| {
                    Box::new(once(y).cartesian_product((0..xmax).rev())) as InnerBox<'a>
                }),
            )
        } else {
            Box::new(
                outer_range
                    .map(move |y| Box::new(once(y).cartesian_product(0..xmax)) as InnerBox<'a>),
            )
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let input = fs::read_to_string(input)?;

    let mat = Matrix::from_iter(input.lines().map(|x| x.chars()));

    println!("Initial matrix:\n{}", mat);

    let mut visible: BitSet = BitSet::with_capacity(mat.num_elements());

    let mut iterators = vec![];
    {
        for column_major in [true, false] {
            for reversed in [true, false] {
                iterators.push(get_iter(&mat, column_major, reversed));
            }
        }
    }

    let mut visible_count: usize = 0;
    for outer_iter in iterators {
        for inner_iter in outer_iter {
            let mut curr_max: Option<u32> = None;
            for (y, x) in inner_iter {
                let tree_size = mat[y][x];
                if let Some(unpacked_max) = &curr_max {
                    if tree_size > *unpacked_max {
                        if visible.insert(y * mat.width() + x) {
                            visible_count += 1;
                        }
                        curr_max = Some(tree_size);
                    }
                } else {
                    if visible.insert(y * mat.width() + x) {
                        visible_count += 1;
                    }
                    curr_max = Some(tree_size);
                }
            }
        }
    }

    println!("Total visible trees were {}", visible_count);

    // Part 2
    //
    // We can do this incrementally over the whole matrix, once forwards, once backwards.

    let mut best: (usize, usize) = (0, 0);
    let mut best_score: Option<u64> = None;
    for score_y in 0..mat.height() {
        for score_x in 0..mat.width() {
            let height = mat[score_y][score_x];

            // Check each direction
            let max_x = ((score_x + 1)..mat.width())
                .find(|x| mat[score_y][*x] >= height)
                .or(Some(mat.width() - 1))
                .unwrap();
            let min_x = (0..score_x)
                .rev()
                .find(|x| mat[score_y][*x] >= height)
                .or(Some(0))
                .unwrap();
            let max_y = ((score_y + 1)..mat.height())
                .find(|y| mat[*y][score_x] >= height)
                .or(Some(mat.height() - 1))
                .unwrap();
            let min_y = (0..score_y)
                .rev()
                .find(|y| mat[*y][score_x] >= height)
                .or(Some(0))
                .unwrap();

            // Add up the score
            let score: u64 =
                ((max_x - score_x) * (score_x - min_x) * (max_y - score_y) * (score_y - min_y))
                    .try_into()
                    .unwrap();

            let update = if let Some(best_score) = best_score {
                score > best_score
            } else {
                true
            };
            if update {
                best_score = Some(score);
                best = (score_y, score_x);
            }
        }
    }

    println!(
        "Got best score at {:?} with score of {}",
        best,
        best_score.unwrap()
    );

    Ok(())
}
