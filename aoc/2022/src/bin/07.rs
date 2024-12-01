advent_of_code::solution!(7);

use std::thread::current;

use itertools::Itertools;

type FileSize = u32;

// TODO we dont really need the names tho?
struct FileNode {
    name: String,
    size: FileSize,
}

struct DirectoryNode<'a> {
    name: String,
    parent: Option<&'a mut DirectoryNode<'a>>,
    children: Vec<FilesystemNode<'a>>,
    cached_size: Option<FileSize>,
}

enum FilesystemNode<'a> {
    File(FileNode),
    Directory(DirectoryNode<'a>),
}

impl<'a> FilesystemNode<'a> {
    fn size(&mut self) -> FileSize {
        match self {
            FilesystemNode::File(file) => file.size,
            FilesystemNode::Directory(dir) => dir.size(),
        }
    }
}

impl<'a> DirectoryNode<'a> {
    fn new(name: &str) -> Self {
        DirectoryNode {
            name: name.to_string(),
            parent: None,
            children: Vec::new(),
            cached_size: None,
        }
    }

    fn add_child(&mut self, child: FilesystemNode<'a>) {
        self.children.push(child);
    }

    fn size(&mut self) -> FileSize {
        if let Some(size) = self.cached_size {
            return size;
        }

        let size = self.children.iter_mut().map(|child| child.size()).sum();
        self.cached_size = Some(size);
        size
    }

    fn set_parent(&mut self, parent: &'a mut DirectoryNode<'a>) {
        self.parent = Some(parent);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut root = DirectoryNode::new("root");
    let mut current_dir = &mut root;

    for line in input.lines() {
        if Some('$') == line.chars().next() {
            let parts = line.split(' ').collect::<Vec<_>>();

            // we don't need to do anything if the command is ls
            if parts[1] == "cd" {
                match parts[2] {
                    "/" => {
                        current_dir = &mut root;
                    }
                    ".." => {
                        current_dir = current_dir.parent.as_mut().unwrap();
                    }
                    dir => {
                        if let Some(child) = current_dir.children.iter_mut().find(|child| {
                            if let FilesystemNode::Directory(ref mut dir_node) = child {
                                dir_node.name == dir
                            } else {
                                false
                            }
                        }) {
                            if let FilesystemNode::Directory(ref mut dir_node) = child {
                                current_dir = dir_node;
                            }
                        } else {
                            return None;
                        }
                    }
                }
            }
        } else {
            // if not command we are listing
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
