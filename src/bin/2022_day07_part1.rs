//! Advent of Code 2022 Day 07
//! https://adventofcode.com/2022/day/7
//!
//! Challenge part 1
//!
//! Reads an input file containing Linux-style commands and their output. The output is used to
//! create an internal representation of the directories and files. The files have an associated
//! size. Determines the size of each directory by summing all files in the directory and all
//! sub-directories, sums the totals of all directories at least 100,000 in size, and displays this
//! as the challenge answer.

use std::fmt;
use std::fmt::Display;
use std::fs;

type FileSize = u64;
type NodeId = usize;

const INPUT_FILENAME: &str = "2022_day07_input.txt";
const ROOT_NODE_ID: NodeId = 0;
const CHALLENGE_DIR_SIZE: FileSize = 100_000; // Min. size to be included in challenge total.

#[derive(Clone, Debug, PartialEq)]
enum Node {
    Directory {
        name: String,
        parent: NodeId,
        children: Vec<NodeId>,
    },
    File {
        name: String,
        parent: NodeId,
        file_size: FileSize,
    },
}

/// A `Tree` contains all the nodes in this directory hierarchy. It is created with a root
/// directory named "/". It is special in that its parent is itself. All nodes are referenced by
/// their index in the `t` vector, referred to as the `NodeId`. The root node has a NodeId of 0.
//
// Implementation note: although links between nodes could be implemented with borrows (e.g.,
// &Node), this is complex in Rust and offers poor performance. The latter is because Nodes packed
// into a vector will be close in memory, whereas nodes independently stored in heap memory may be
// placed further apart.
#[derive(Debug, PartialEq)]
struct Tree {
    t: Vec<Node>,
}

impl Tree {
    /// Creates a new `Tree` that is prepopulated with an empty root directory.
    fn new() -> Self {
        Self {
            t: vec![Node::Directory {
                name: "/".to_string(),
                parent: ROOT_NODE_ID,
                children: Vec::new(),
            }],
        }
    }

    /// Creates a new directory node and adds it to the end of the list of nodes maintained in
    /// `Tree`. `name` should not have leading or trailing whitespace.
    fn add_directory_node(&mut self, name: &str, parent: NodeId) -> NodeId {
        let new_node_id = self.t.len();
        self.t.push(Node::Directory {
            name: name.to_string(),
            parent,
            children: Vec::new(),
        });

        match &mut self.t[parent] {
            Node::Directory {
                name: _,
                parent: _,
                children,
            } => {
                children.push(new_node_id);
            }
            _ => {
                panic!("Fatal error - the parent of a node was not a Directory object, which should never happen");
            }
        }

        new_node_id
    }

    /// Creates a new file node and adds it to the end of the list of nodes maintained in `Tree`.
    fn add_file_node(&mut self, name: &str, parent: NodeId, file_size: FileSize) -> NodeId {
        let new_node_id = self.t.len();
        self.t.push(Node::File {
            name: name.to_string(),
            parent,
            file_size,
        });

        match &mut self.t[parent] {
            Node::Directory {
                name: _,
                parent: _,
                children,
            } => {
                children.push(new_node_id);
            }
            _ => {
                panic!("Fatal error - the parent of a node was not a Directory object, which should never happen");
            }
        }

        new_node_id
    }
}

/// Displays this `Tree` in the same format used by the challenge.
impl Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recurse(
            tree: &Tree,
            f: &mut fmt::Formatter<'_>,
            current_node_idx: NodeId,
            depth: usize,
        ) -> fmt::Result {
            match &tree.t[current_node_idx] {
                Node::Directory {
                    name,
                    parent: _,
                    children,
                } => {
                    if let Err(err) = write!(f, "{0:>1$} {name} (dir)\n", "-", 2 * depth + 1) {
                        return Err(err);
                    }

                    for child in children.iter() {
                        if let Err(err) = recurse(tree, f, *child, depth + 1) {
                            return Err(err);
                        }
                    }
                    Ok(())
                }
                Node::File {
                    name,
                    parent: _,
                    file_size,
                } => {
                    return write!(
                        f,
                        "{0:>1$} {name} (file, size={file_size})\n",
                        "-",
                        2 * depth + 1
                    );
                }
            }
        }

        recurse(self, f, 0, 0)
    }
}

/// Handle a 'cd' command. `dir_name` can be:
///     "/" to return the `NodeId` of the root directory
///     ".." to return the `NodeId` of the `current_dir_id`'s parent
///     a sub-directory name to return its `NodeId`
///
/// If a sub-directory is specified that does not exist it is created.
///
/// # Panics
///
/// Panics if `dir_name` is empty or if `current_dir_id` is not a `Directory` node.
fn do_cd(tree: &mut Tree, current_dir_id: NodeId, dir_name: &str) -> NodeId {
    assert!(
        dir_name.len() > 0,
        "cd must be called with a directory name"
    );

    match dir_name {
        "/" => {
            return ROOT_NODE_ID;
        }
        ".." => match tree.t[current_dir_id] {
            Node::Directory { parent, .. } => {
                return parent;
            }
            _ => {
                panic!("Internal error: do_cd was called with a non-directory node");
            }
        },
        _ => match &tree.t[current_dir_id] {
            Node::Directory { children, .. } => {
                for &c in children {
                    if let Node::Directory { name, .. } = &tree.t[c] {
                        if name == dir_name {
                            return c;
                        }
                    }
                }
                return tree.add_directory_node(dir_name, current_dir_id);
            }
            _ => {
                panic!("Internal error: do_cd was called with a non-directory node");
            }
        },
    }
}

/// Calculates the size of each directory in `tree`. A directory's size is the total of all the
/// files it contains directly and indirectly (i.e., via sub-directories). Returns a vector that
/// uses the same indexes as the `NodeId`'s in `tree` and which contains the size of each
/// directory in `tree`. For example, the size of the directory with NodeId 2 in `tree` can be
/// found in index 2 of the result.
fn determine_directory_sizes(tree: &Tree) -> Vec<Option<FileSize>> {
    let node_count = tree.t.len();
    let mut dir_sizes = vec![None; node_count];

    fn recurse(tree: &Tree, dir_sizes: &mut Vec<Option<FileSize>>, current_dir_id: NodeId) {
        match &tree.t[current_dir_id] {
            Node::Directory { children, .. } => {
                let mut dir_size = 0;

                for c in children {
                    match &tree.t[*c] {
                        Node::Directory { .. } => {
                            if dir_sizes[*c].is_none() {
                                recurse(tree, dir_sizes, *c);
                            }
                            dir_size += dir_sizes[*c].unwrap();
                        }
                        Node::File { file_size, .. } => {
                            dir_size += file_size;
                        }
                    }
                }
                dir_sizes[current_dir_id] = Some(dir_size);
            }
            _ => {
                panic!("Internal error: determine_directory_sizes internal function was called with a non-directory node");
            }
        }
    }

    recurse(tree, &mut dir_sizes, ROOT_NODE_ID);

    dir_sizes
}

/// Returns the sum of the sizes of all directories with a size of `CHALLENGE_DIR_SIZE` or less.
/// This the answer for part 1 of the challenge.
fn challenge_answer(tree: &Tree) -> FileSize {
    let dir_sizes = determine_directory_sizes(tree);

    dir_sizes
        .iter()
        .map(|ds| ds.unwrap_or(0))
        .filter(|ds| ds <= &CHALLENGE_DIR_SIZE)
        .sum()
}

/// Takes a string containing the entire input file and converts it into a tree which is then
/// returned. Each line of input must be one of:
///     $ cd <directory_name>
///     $ ls
///     dir <directory_name>
///     <file_size> <file_name>
///
/// # Panics
///
/// Panics if the input is malformed.
fn parse_input(input: &str) -> Tree {
    let mut tree = Tree::new();
    let mut cwd = ROOT_NODE_ID; // current working directory

    for line in input.lines() {
        if line != "" {
            if line.starts_with("$ cd ") {
                let dir_name = line.strip_prefix("$ cd ").unwrap().trim();
                cwd = do_cd(&mut tree, cwd, dir_name);
            } else if line.starts_with("dir ") {
                let dir_name = line.strip_prefix("dir ").unwrap().trim();
                _ = do_cd(&mut tree, cwd, dir_name);
            } else if line.starts_with("$ ls") {
                // No action required.
            } else {
                let (file_size_str, file_name) = line.split_once(' ').unwrap();
                let file_size = FileSize::from_str_radix(file_size_str, 10).unwrap();
                _ = tree.add_file_node(file_name, cwd, file_size);
            }
        }
    }

    tree
}

fn main() {
    let input = fs::read_to_string(INPUT_FILENAME).expect("Error reading input file");
    let tree = parse_input(&input);

    println!("The challenge answer is {}", challenge_answer(&tree),);
}

// Test data based on examples on the challenge page.
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    const EXPECTED_OUTPUT: &str = "\
- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)
";

    #[test]
    fn test_do_cd() {
        let mut tree = Tree::new();
        assert_eq!(do_cd(&mut tree, ROOT_NODE_ID, "subdir1"), 1);

        assert_eq!(
            tree.t.get(1),
            Some(&Node::Directory {
                name: "subdir1".to_string(),
                parent: ROOT_NODE_ID,
                children: Vec::new()
            })
        );

        assert_eq!(do_cd(&mut tree, 1, "subdir2"), 2);
        assert_eq!(
            tree.t.get(2),
            Some(&Node::Directory {
                name: "subdir2".to_string(),
                parent: 1,
                children: Vec::new()
            })
        );

        assert_eq!(do_cd(&mut tree, ROOT_NODE_ID, "subdir3"), 3);
        assert_eq!(
            tree.t.get(3),
            Some(&Node::Directory {
                name: "subdir3".to_string(),
                parent: ROOT_NODE_ID,
                children: Vec::new()
            })
        );

        if let Node::Directory { children, .. } = &tree.t[ROOT_NODE_ID] {
            assert_eq!(children, &vec![1, 3]);
        } else {
            panic!("Error: the root directory must be a directory type, but isn't.");
        }

        if let Node::Directory { children, .. } = &tree.t[1] {
            assert_eq!(children, &vec![2]);
        } else {
            panic!("Error: 'subdir2' directory must be a directory type, but isn't.");
        }

        assert_eq!(do_cd(&mut tree, ROOT_NODE_ID, "/"), ROOT_NODE_ID);
        assert_eq!(do_cd(&mut tree, 1, "/"), ROOT_NODE_ID);
        assert_eq!(do_cd(&mut tree, 2, "/"), ROOT_NODE_ID);
        assert_eq!(do_cd(&mut tree, 3, "/"), ROOT_NODE_ID);

        assert_eq!(do_cd(&mut tree, ROOT_NODE_ID, ".."), ROOT_NODE_ID);
        assert_eq!(do_cd(&mut tree, 1, ".."), ROOT_NODE_ID);
        assert_eq!(do_cd(&mut tree, 2, ".."), 1);
        assert_eq!(do_cd(&mut tree, 3, ".."), ROOT_NODE_ID);

        assert_eq!(do_cd(&mut tree, ROOT_NODE_ID, "subdir1"), 1);
        assert_eq!(do_cd(&mut tree, 1, "subdir2"), 2);
        assert_eq!(do_cd(&mut tree, ROOT_NODE_ID, "subdir3"), 3);
    }

    #[test]
    #[should_panic]
    fn test_do_cd_with_bad_dir_name() {
        let mut tree = Tree::new();
        do_cd(&mut tree, ROOT_NODE_ID, "");
    }

    #[test]
    fn test_parse_input() {
        let tree = parse_input(TEST_INPUT);

        assert_eq!(tree.to_string(), EXPECTED_OUTPUT);
    }

    #[test]
    fn test_determine_directory_sizes() {
        let tree = parse_input(TEST_INPUT);

        assert_eq!(
            determine_directory_sizes(&tree),
            vec![
                Some(48381165), // Dir '/'
                Some(94853),    // Dir 'a'
                None,
                None,
                Some(24933642), // Dir 'd'
                Some(584),      // Dir 'e'
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ]
        );
    }

    #[test]
    fn test_challenge_answer() {
        let tree = parse_input(TEST_INPUT);

        assert_eq!(challenge_answer(&tree), 95437);
    }
}
