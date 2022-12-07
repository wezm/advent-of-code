use std::cell::RefCell;
use std::io::{self};
use std::ops::DerefMut;
use std::rc::Rc;

const LIMIT: usize = 100_000;
const DISK_CAPACITY: usize = 70_000_000;
const NEED_FREE: usize = 30_000_000;

type Node<'a> = Rc<RefCell<FsNode<'a>>>;

struct FsNode<'a> {
    parent: Option<Node<'a>>,
    data: FsNodeData<'a>,
}
enum FsNodeData<'a> {
    Dir(Dir<'a>),
    File(File<'a>),
}

struct Dir<'a> {
    name: &'a str,
    size: usize,
    children: Vec<Node<'a>>,
}

struct File<'a> {
    name: &'a str,
    size: usize,
}

fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("input/day_7.txt")?;
    let root = new_node(None, FsNodeData::Dir(Dir::new("/")));
    let mut pwd = Rc::clone(&root);

    for line in input.lines() {
        match line.chars().next() {
            Some('$') => pwd = execute_cmd(&line[1..], Rc::clone(&pwd), &root)?,
            Some('d') => add_dir(line, Rc::clone(&pwd))?,
            Some(c) if c.is_ascii_digit() => add_file(line, Rc::clone(&pwd))?,
            c => panic!("unexpected character: {:?}", c),
        }
    }

    // Traverse the filesystem tree looking for large dirs
    let sum_of_large_dirs = traverse(&root.borrow());
    println!("Part 1: {}", sum_of_large_dirs);

    let disk_free = DISK_CAPACITY - node_size(&root.borrow());
    let need_to_free = NEED_FREE - disk_free;
    println!(
        "Part 2: Disk free {}, need to free {}",
        disk_free, need_to_free
    );

    // Need to find the directory with size closest to need_to_free
    let part2 = find_closest_dir_size(&root.borrow(), need_to_free, DISK_CAPACITY);
    println!("Part 2: {}", part2);

    Ok(())
}

fn traverse(node: &FsNode<'_>) -> usize {
    let mut sum = 0;
    match &node.data {
        FsNodeData::Dir(Dir { size, children, .. }) => {
            if *size <= LIMIT {
                sum += size
            }
            for child in children {
                sum += traverse(&child.borrow())
            }
            sum
        }
        FsNodeData::File(_) => 0,
    }
}

fn find_closest_dir_size(node: &FsNode<'_>, need_to_free: usize, mut min: usize) -> usize {
    match &node.data {
        FsNodeData::Dir(Dir { size, children, .. }) => {
            if *size >= need_to_free && *size < min {
                min = *size
            }
            for child in children {
                let size = find_closest_dir_size(&child.borrow(), need_to_free, DISK_CAPACITY);
                if size >= need_to_free && size < min {
                    min = size
                }
            }
            min
        }
        FsNodeData::File(_) => min,
    }
}

/// Execute a command, returning the new pwd
fn execute_cmd<'a, 'b>(line: &'a str, pwd: Node<'b>, root: &'a Node<'b>) -> io::Result<Node<'b>> {
    let mut words = line.trim().split(' ');
    let cmd = words.next();
    let arg = words.next();
    match (cmd, arg) {
        (Some("ls"), None) => Ok(pwd),
        (Some("cd"), Some("..")) => pwd
            .borrow()
            .parent
            .as_ref()
            .map(Rc::clone)
            .ok_or_else(|| err("node has no parent")),
        (Some("cd"), Some("/")) => Ok(Rc::clone(root)),
        (Some("cd"), Some(dir)) => {
            let pwd = pwd.borrow();
            let FsNode { data: FsNodeData::Dir(pwd), .. } = &*pwd else {
                return Err(err("pwd is not a dir"))
            };
            pwd.children
                .iter()
                .find(|child| {
                    matches!(
                        *child.borrow(),
                        FsNode {
                            data: FsNodeData::Dir(Dir { name, .. }),
                            ..
                        } if name == dir
                    )
                })
                .ok_or_else(|| err("node has no parent"))
                .cloned()
        }
        _ => Err(err("unexpected command")),
    }
}

fn add_dir<'a>(line: &'a str, pwd: Node<'a>) -> io::Result<()> {
    let mut words = line.trim().split(' ');
    let Some("dir") = words.next() else {
        return Err(err("didn't get dir"))
    };
    let Some(name) = words.next() else {
        return Err(err("missing dir name"))
    };

    let mut dir = pwd.borrow_mut();
    let FsNode { data: FsNodeData::Dir(dir), .. } = dir.deref_mut() else {
        return Err(err("pwd is not a dir"))
    };
    dir.children.push(new_node(
        Some(Rc::clone(&pwd)),
        FsNodeData::Dir(Dir::new(name)),
    ));
    Ok(())
}

fn add_file<'a>(line: &'a str, pwd: Node<'a>) -> io::Result<()> {
    let mut words = line.trim().split(' ');
    let size = words
        .next()
        .and_then(|size| size.parse::<usize>().ok())
        .ok_or_else(|| err("file missing size"))?;
    let Some(name) = words.next() else {
        return Err(err("missing file name"))
    };

    let mut dir = pwd.borrow_mut();
    let FsNode { data: FsNodeData::Dir(dir), parent, .. } = dir.deref_mut() else {
        return Err(err("pwd is not a dir"))
    };
    dir.children.push(new_node(
        Some(Rc::clone(&pwd)),
        FsNodeData::File(File::new(name, size)),
    ));
    dir.size += size;
    if let Some(parent) = parent {
        update_size(size, &mut parent.borrow_mut());
    }
    Ok(())
}

fn update_size(size: usize, node: &mut FsNode<'_>) {
    let FsNode { data: FsNodeData::Dir(dir), parent, .. } = node else {
        panic!("node is not a dir")
    };
    dir.size += size;
    if let Some(parent) = parent {
        update_size(size, &mut parent.borrow_mut())
    }
}

fn new_node<'a>(parent: Option<Node<'a>>, data: FsNodeData<'a>) -> Node<'a> {
    Rc::new(RefCell::new(FsNode::new(parent, data)))
}

fn node_size(node: &FsNode<'_>) -> usize {
    let FsNode { data: FsNodeData::Dir(dir), .. } = node else {
        panic!("node is not a dir")
    };
    dir.size
}

fn err(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}

impl<'a> FsNode<'a> {
    fn new(parent: Option<Node<'a>>, data: FsNodeData<'a>) -> Self {
        FsNode { parent, data }
    }
}

impl<'a> Dir<'a> {
    fn new(name: &'a str) -> Self {
        Dir {
            size: 0,
            name,
            children: Vec::new(),
        }
    }
}

impl<'a> File<'a> {
    fn new(name: &'a str, size: usize) -> Self {
        File { name, size }
    }
}
