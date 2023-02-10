use std::{
    cell::RefCell,
    env,
    error::Error,
    fmt,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    rc::{Rc, Weak},
};

#[derive(Debug, Clone)]
struct NotACommandError(String);

impl Error for NotACommandError {}

impl fmt::Display for NotACommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line does not represent a command: {}", self.0)
    }
}

#[derive(Debug, Clone)]
struct NoCommandError;

impl Error for NoCommandError {}

impl fmt::Display for NoCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "No current command but line does not represent a command"
        )
    }
}

#[derive(Debug, Clone)]
struct NoParentError;

impl Error for NoParentError {}

impl fmt::Display for NoParentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current directory has no parent")
    }
}

#[derive(Debug, Clone)]
struct NoChildError(String);

impl Error for NoChildError {}

impl fmt::Display for NoChildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current directory has no child with name '{}'", self.0)
    }
}

#[derive(Debug, Clone)]
struct NotACdEntryError(String);

impl Error for NotACdEntryError {}

impl fmt::Display for NotACdEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "The following line is not a valid cd output line: {}",
            self.0
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum DirectoryTreeNodeType {
    Directory,
    File(usize), // Also has a size
}

#[derive(Debug, Clone)]
struct DirectoryTree {
    parent: Weak<RefCell<DirectoryTree>>,
    children: Vec<Rc<RefCell<DirectoryTree>>>,
    name: String,
    node_type: DirectoryTreeNodeType,
}

impl DirectoryTree {
    fn new(
        name: String,
        parent: Weak<RefCell<DirectoryTree>>,
        node_type: DirectoryTreeNodeType,
    ) -> Self {
        Self {
            parent,
            children: vec![],
            name,
            node_type,
        }
    }

    fn root() -> Self {
        Self {
            parent: Weak::new(),
            children: vec![],
            name: String::from(""),
            node_type: DirectoryTreeNodeType::Directory,
        }
    }

    fn path_name(&self) -> String {
        let mut buf: Vec<u8> = Vec::new();
        let mut next = self.parent.clone();
        let mut path_stack = vec![];
        while let Some(parent) = next.upgrade() {
            path_stack.push(parent.clone());
            next = parent.borrow().parent.clone();
        }
        for part in path_stack.into_iter().rev() {
            write!(buf, "{}/", part.borrow().name).unwrap();
        }
        write!(buf, "{}", self.name).unwrap();
        std::str::from_utf8(&buf[..]).unwrap().to_string()
    }
}

impl fmt::Display for DirectoryTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.path_name())?;
        if let DirectoryTreeNodeType::File(size) = self.node_type {
            write!(f, " size {}", size)?;
        }
        writeln!(f, "")?;

        // Print children recursively
        for child in &self.children {
            child.borrow().fmt(f)?;
        }
        Ok(())
    }
}

// We will need some struct to track directory structure etc.
fn process_commands<LinesIter: Iterator<Item = io::Result<String>>>(
    lines: &mut LinesIter,
) -> Result<Rc<RefCell<DirectoryTree>>, Box<dyn Error>> {
    let root_node = Rc::new(RefCell::new(DirectoryTree::root()));

    let mut curr_node = root_node.clone();
    let mut ls = false;
    while let Some(Ok(line)) = lines.next() {
        if &line[0..2] == "$ " {
            let cmd_token = &line[2..4];
            match cmd_token {
                "ls" => {
                    ls = true;
                }
                "cd" => {
                    let arg = &line[5..];
                    curr_node = match arg {
                        "/" => root_node.clone(),
                        ".." => {
                            // Get parent node
                            if let Some(parent) = curr_node.borrow().parent.upgrade() {
                                parent.clone()
                            } else {
                                return Err(Box::new(NoParentError));
                            }
                        }
                        _ => {
                            if let Some(child) = (&curr_node.borrow().children)
                                .into_iter()
                                .find(|x| x.borrow().name == arg)
                            {
                                child.clone()
                            } else {
                                return Err(Box::new(NoChildError(String::from(arg))));
                            }
                        }
                    }
                }
                _ => {
                    return Err(Box::new(NotACommandError(format!(
                        "Unrecognised command '{}'",
                        cmd_token
                    ))))
                }
            }
        } else {
            if !ls {
                return Err(Box::new(NotACommandError(format!("Line must start with '$ ' unless this line represents the output of an ls command"))));
            }

            // Process output - it will either start with 'dir' and indicate a directory or with a number to indicate file size
            if line.len() >= 4 && &line[0..4] == "dir " {
                let dirname = &line[4..];
                // Insert a new directory below the current node
                curr_node
                    .borrow_mut()
                    .children
                    .push(Rc::new(RefCell::new(DirectoryTree::new(
                        String::from(dirname),
                        Rc::downgrade(&curr_node),
                        DirectoryTreeNodeType::Directory,
                    ))));
            } else {
                if let Some((size, name)) = line.split_once(' ') {
                    let size: usize = size.parse()?;
                    curr_node
                        .borrow_mut()
                        .children
                        .push(Rc::new(RefCell::new(DirectoryTree::new(
                            String::from(name),
                            Rc::downgrade(&curr_node),
                            DirectoryTreeNodeType::File(size),
                        ))))
                } else {
                    return Err(Box::new(NotACdEntryError(String::from(line))));
                }
            }
        }
    }
    Ok(root_node)
}

type ProblemResult = Vec<(Rc<RefCell<DirectoryTree>>, usize)>;
fn get_directories_within_size_limit_aux(
    tree: &Rc<RefCell<DirectoryTree>>,
    limit: usize,
    res: &mut ProblemResult,
) -> usize {
    if let DirectoryTreeNodeType::File(size) = tree.borrow().node_type {
        size
    } else {
        let mut total_size: usize = 0;
        for child in &tree.borrow().children {
            total_size += get_directories_within_size_limit_aux(&child, limit, res);
        }

        if total_size <= limit {
            res.push((tree.clone(), total_size));
        }

        total_size
    }
}

fn get_directories_within_size_limit(
    tree: &Rc<RefCell<DirectoryTree>>,
    limit: usize,
) -> ProblemResult {
    let mut res = vec![];
    get_directories_within_size_limit_aux(tree, limit, &mut res);
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let input = File::open(input)?;

    // Process commands
    let mut line_iter = BufReader::new(input).lines().into_iter();
    let directory_tree = process_commands(&mut line_iter)?;

    // Print out the directory tree just ot see if it looks correct
    println!("{}", directory_tree.borrow());

    // Now calculate the answer we're looking for.
    // Using a DFS of directory tree we identify the size of each
    // directory during iteration
    const SIZE_LIMIT: usize = 100000;
    let directories = get_directories_within_size_limit(&directory_tree, SIZE_LIMIT);

    println!("Directories under {} in size:", SIZE_LIMIT);
    for (dir, total_size) in &directories {
        println!(
            "Directory {} had total size {}",
            dir.borrow().path_name(),
            total_size
        );
    }
    let total_size_of_all_under_limit: usize = (&directories).into_iter().map(|e| e.1).sum();
    println!(
        "Sum of all directories under {} in size was {}",
        SIZE_LIMIT, total_size_of_all_under_limit
    );

    Ok(())
}
