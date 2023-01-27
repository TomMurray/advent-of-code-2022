use std::{
    env,
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Iterator,
};

use regex::Regex;

struct Stacks(Vec<Vec<char>>);

impl Stacks {
    // Lifetime elision rule should be filling this out
    // I wrote out lifetimes explicitly to satisfy myself that despite
    // contained unsafe code this remains true.
    fn get_mut_stack_pair<'a>(
        &'a mut self,
        (a, b): (usize, usize),
    ) -> (&'a mut Vec<char>, &'a mut Vec<char>) {
        // In order to get a mutable reference to source and dest
        // we require some unsafe code. We know that source and dest
        // never overlap and we panic if they are at runtime for some
        // unexpected reason.
        assert!(a != b);
        unsafe {
            // To explain this bit of magic for myself,
            // each of these lines first takes a mutable reference to
            // the sub-element we want, then casts it to a pointer (unsafe)
            // then casts it back to a mutable reference. This
            // avoids the borrow checker acting for the input &mut self
            // and instead we get 2 new &mut Vec<char> that themselves
            // have borrow checking.
            (
                &mut *(&mut self.0[a] as *mut _),
                &mut *(&mut self.0[b] as *mut _),
            )
        }
    }

    /// Move crates from one stack to another.
    /// This has the 'part 2' behaviour that when > 1 are
    /// moved at a time, their relative ordering is maintained
    /// in the move. The 'part 1' behaviour can be implemented on
    /// top of this by only moving one crate at a time.
    fn mov(&mut self, count: usize, from: usize, to: usize) {
        assert!(from < self.0.len() && to < self.0.len());
        let (from_stack, to_stack) = self.get_mut_stack_pair((from, to));
        let crates = from_stack.drain(from_stack.len() - count..);
        to_stack.extend(crates);
    }
}

impl fmt::Display for Stacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num_columns = self.0.len();
        // Find max stack height
        let max_stack_height = (&self.0)
            .into_iter()
            .map(|x| x.len())
            .max()
            .expect("Need at least 1 column");
        writeln!(
            f,
            "Stack with max height {max_stack_height} and {num_columns} columns"
        )?;

        // Write layer by layer the result, in the same style as the input
        for y in (0..max_stack_height).rev() {
            let mut first = true;
            for column in &self.0 {
                if first {
                    write!(f, " ")?;
                }
                if column.len() > y {
                    write!(f, "[{}]", column[y])?;
                } else {
                    write!(f, "   ")?;
                }
                first = false;
            }
            // Next line
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parse_stacks(lines: &mut Lines<BufReader<File>>) -> Stacks {
    let mut contents = Vec::new();
    let mut first_line = true;
    'outer: for line in lines {
        let mut remainder: &str = &line.unwrap();
        let mut column = 0;
        while remainder.len() >= 3 {
            // Expect a single space between columns, trim this off if not the first column
            if column != 0 {
                assert!(remainder.chars().next().unwrap() == ' ');
                remainder = &remainder[1..];
            }
            println!("remainder (after chopping off start)='{}'", remainder);

            let (lhs, rhs) = remainder.split_at(3);
            //println!("lhs={}, rhs={}", lhs, rhs);

            // If this is the first line, insert columns into the contents vector
            if first_line {
                contents.push(Vec::<char>::new());
            }

            let mut it = lhs.chars();

            // We know there are exactly 3 characters in lhs, so just unwrap
            let first_char = it.next().unwrap();
            println!("First char of column {} is {}", column, first_char);
            // Push the contents
            if first_char == '[' {
                contents[column].push(it.next().unwrap());
                assert!(it.next().unwrap() == ']');
            } else if first_char == ' ' {
                let second_char = it.next().unwrap();
                println!("Second char of column {} is {}", column, second_char);

                // Check for column numbering
                if second_char >= '0' && second_char <= '9' {
                    // End iteration over lines.
                    // Next line should be instructions (ignoring blank lines)
                    let third_char = it.next().unwrap();
                    assert!(third_char == ' ');
                    break 'outer;
                } else {
                    let third_char = it.next().unwrap();
                    assert!(second_char == ' ');
                    assert!(third_char == ' ');
                }
            }

            column += 1;
            remainder = rhs;
        }

        first_line = false;
    }
    // When we are done with this, all the stacks will be upside down
    // because we parsed them that way. Reverse them now.
    for column in &mut contents {
        column.reverse();
    }
    Stacks(contents)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let file = File::open(path)?;

    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();

    // First parse the initial state of the stacks
    let mut stacks = parse_stacks(&mut lines);

    println!("Initial stacks are:\n{}", stacks);

    // Now apply the instructions
    let re = Regex::new(r"move (\d+) from (\d{1}) to (\d{1})").unwrap();
    for line in lines {
        let line = line.unwrap();
        let captures = re.captures(&line);
        if let Some(captures) = captures {
            let parsed: Vec<usize> = (1..4)
                .map(|x| captures[x].parse::<usize>().unwrap())
                .collect();
            let (count, from, to) = (parsed[0], parsed[1], parsed[2]);
            println!("Move {} boxes from column {} to column {}", count, from, to);
            stacks.mov(count, from - 1, to - 1);
            // Part 1 behaviour:
            // for _ in 0..count {
            //     stacks.mov(1, from - 1, to - 1);
            // }
        }
    }

    println!("Final stacks are:\n{}", stacks);

    let stack_tops: String = stacks
        .0
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|x| x.into_iter().rev().next().unwrap())
        .collect();
    println!("Meaning all the top letters are {}", stack_tops);

    Ok(())
}
