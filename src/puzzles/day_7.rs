use camino::Utf8PathBuf;
use id_tree::{InsertBehavior, Node, Tree};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

#[derive(Debug)]
struct Cd(Utf8PathBuf);

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((
        map(map(tag("ls"), |_| Ls), Into::into),
        map(map(preceded(tag("cd "), parse_path), Cd), Into::into),
    ))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

fn tree() -> Tree<FsEntry> {
    let lines = include_str!("day_7_input.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut tree = Tree::<FsEntry>::new();
    let root = tree
        .insert(Node::new(FsEntry { size: 0 }), InsertBehavior::AsRoot)
        .unwrap();
    let mut curr = root;

    for line in lines {
        //println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // just ignore those
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore, we're already there
                    }
                    ".." => {
                        curr = tree.get(&curr).unwrap().parent().unwrap().clone();
                    }
                    _ => {
                        let node = Node::new(FsEntry { size: 0 });
                        curr = tree.insert(node, InsertBehavior::UnderNode(&curr)).unwrap();
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // ignore, we'll do that when we `cd` into them
                }
                Entry::File(size, _path) => {
                    let node = Node::new(FsEntry { size });
                    tree.insert(node, InsertBehavior::UnderNode(&curr)).unwrap();
                }
            },
        }
    }

    //let mut s = String::new();
    //tree.write_formatted(&mut s).unwrap();
    //println!("{s}");

    tree
}

#[derive(Debug)]
struct FsEntry {
    size: u64,
}

fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> u64 {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child).unwrap());
    }
    total
}

pub fn sum_of_dir_smaller_than_100000() -> u64 {
    let tree = tree();
    tree.traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n))
        .filter(|&s| s <= 100_000)
        .sum()
}

pub fn size_of_smallest_dir_to_delete() -> u64 {
    let tree = tree();
    let total_space = 70000000_u64;
    let used_space = total_size(&tree, tree.get(tree.root_node_id().unwrap()).unwrap());
    let free_space = total_space - used_space;
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    tree.traverse_pre_order(tree.root_node_id().unwrap())
        .unwrap()
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n))
        .filter(|&s| s >= minimum_space_to_free)
        .min()
        .unwrap()
}
