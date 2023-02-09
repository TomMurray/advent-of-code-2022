use std::{
    cell::RefCell,
    env,
    error::Error,
    fmt,
    fs::File,
    io::{self, BufRead, BufReader},
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
        write!(f, "The following line is not a valid cd output line: {}", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
enum DirectoryTreeNodeType {
    Directory,
    File(usize), // Also has a size
}

#[derive(Debug, Clone)]
struct DirectoryTree {
    parent: Weak<DirectoryTree>,
    children: Vec<Rc<DirectoryTree>>,
    name: String,
    node_type: DirectoryTreeNodeType,
}

impl DirectoryTree {
    fn new(name: String, parent: Weak<DirectoryTree>, node_type : DirectoryTreeNodeType) -> Self {
        Self {
            parent,
            children: vec![],
            name,
            node_type
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
}

impl fmt::Display for DirectoryTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(parent) = self.parent.upgrade() {
            parent.fmt(f)?;
        }
        write!(f, "{}/", self.name)
    }
}

// We will need some struct to track directory structure etc.
fn process_commands<LinesIter: Iterator<Item = io::Result<String>>>(
    lines: &mut LinesIter,
) -> Result<Rc<DirectoryTree>, Box<dyn Error>> {
    let mut root_node = Rc::new(DirectoryTree::root());

    let mut curr_node: Rc<DirectoryTree> = root_node.clone();
    let mut ls = false;
    while let Some(Ok(line)) = lines.next() {
        if &line[0..2] == "$ " {
            let cmd_token = &line[2..];
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
                            if let Some(parent) = curr_node.parent.upgrade() {
                                parent.clone()
                            } else {
                                return Err(Box::new(NoParentError));
                            }
                        }
                        _ => {
                            if let Some(child) =
                                (&curr_node.children).into_iter().find(|x| x.name == arg)
                            {
                                child.clone()
                            } else {
                                return Err(Box::new(NoChildError(String::from(arg))));
                            }
                        }
                    }
                }
                _ => {
                    return Err(Box::new(NotACommandError(
                        "Line must start with '$ '".to_string(),
                    )))
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
                // TODO: Looks like I do need RefCell to actually modify the current node. Makes sense.
                curr_node.children.push(
                    Rc::new(DirectoryTree::new(
                        String::from(dirname),
                        Rc::downgrade(&curr_node),
                        DirectoryTreeNodeType::Directory
                    ))
                );
            } else {
                if let Some((size, name)) = line.split_once(' ') {

                } else {
                    return Err(Box::new(NotACdEntryError(String::from(line))));
                }
            }
        }
    }
    Ok(root_node)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let input = File::open(input)?;

    // Process commands
    let mut line_iter = BufReader::new(input).lines().into_iter();
    let directory_tree = process_commands(&mut line_iter)?;

    Ok(())
}
