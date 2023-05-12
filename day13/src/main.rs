use std::{
    cmp::Ordering,
    env,
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Debug)]
struct BadInputToken(char);

impl fmt::Display for BadInputToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unrecognised input character '{}'", self.0)
    }
}

impl Error for BadInputToken {}

#[derive(Clone, Debug, Eq)]
enum Entry {
    List(Vec<Entry>),
    Number(i32),
}

fn compare_lists(lhs: &Vec<Entry>, rhs: &Vec<Entry>) -> Ordering {
    let mut lhs_it = lhs.into_iter();
    let mut rhs_it = rhs.into_iter();

    loop {
        let (lhs, rhs) = (lhs_it.next(), rhs_it.next());

        // The following will return the ordering immediately once it is
        // no longer ambiguous (i.e. not equal).
        match (lhs, rhs) {
            (None, None) => break,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => {
                return Ordering::Greater;
            }
            (Some(lhs), Some(rhs)) => {
                let res = lhs.cmp(rhs);
                if res.is_ne() {
                    return res;
                }
            }
        }
    }

    Ordering::Equal
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Entry::Number(a), Entry::Number(b)) => a.cmp(b),
            (Entry::List(a), Entry::List(b)) => compare_lists(a, b),
            (Entry::Number(a), Entry::List(b)) => compare_lists(&vec![Entry::Number(*a)], b),
            (Entry::List(a), Entry::Number(b)) => compare_lists(a, &vec![Entry::Number(*b)]),
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn parse_entry(line: String) -> Result<Entry, Box<dyn Error>> {
    let mut entry_stack = vec![Entry::List(vec![])];

    for c in line.chars() {
        match c {
            '[' => {
                entry_stack.push(Entry::List(vec![]));
            }
            ']' => {
                // Pop the top of the stack. It might be
                // a number or it might be a list in the
                // case that there were no entries in the
                // array.
                let mut e = entry_stack.pop().unwrap();
                if let Entry::Number(_) = e {
                    let list = entry_stack.last_mut().unwrap();

                    if let Entry::List(list) = list {
                        list.push(e);
                    } else {
                        panic!("Oops");
                    }
                    e = entry_stack.pop().unwrap();
                }
                if let Entry::List(list) = e {
                    let parent_list = entry_stack.last_mut().unwrap();
                    if let Entry::List(parent_list) = parent_list {
                        parent_list.push(Entry::List(list));
                    }
                }
            }
            '0'..='9' => {
                // Start of or extension of a number
                let value = c as i32 - '0' as i32;
                if let Entry::Number(n) = entry_stack.last_mut().unwrap() {
                    *n = *n * 10 + value;
                } else {
                    entry_stack.push(Entry::Number(value));
                }
            }
            ',' => {
                // End of a number, or separates lists
                if let Entry::Number(_) = entry_stack.last().unwrap() {
                    let e = entry_stack.pop().unwrap();
                    if let Entry::List(l) = entry_stack.last_mut().unwrap() {
                        l.push(e);
                    }
                }
            }
            _ => {
                return Err(Box::new(BadInputToken(c)));
            }
        }
    }
    Ok(entry_stack.pop().unwrap())
}


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let file = File::open(path)?;

    // Every 2 lines is a pair, with a blank line in between
    let mut first: Option<Entry> = None;
    let mut pair_idx: usize = 1;
    let mut pt1_result: usize = 0;

    for line in BufReader::new(file).lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let e = parse_entry(line)?;
        if let Some(lhs) = first {
            // Do the comparison
            let c = lhs.cmp(&e);
            if c.is_le() {
                pt1_result += pair_idx;
            }
            pair_idx += 1;
            first = None;
        } else {
            first = Some(e);
        }
    }

    println!("Part 1 result: {}", pt1_result);

    Ok(())
}
