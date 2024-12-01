use itertools::Itertools;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

advent_of_code::solution!(7);

type FileSize = u32;

// TODO we dont really need the names tho?
struct FileNode {
    name: String,
    size: FileSize,
}

struct DirectoryNode {
    name: String,
    parent: Option<Weak<RefCell<DirectoryNode>>>,
    children: Vec<Rc<RefCell<DirectoryNode>>>,
    cached_size: Option<FileSize>,
}

enum FilesystemNode {
    File(FileNode),
    Directory(DirectoryNode),
}

impl FilesystemNode {
    fn size(&mut self) -> FileSize {
        match self {
            FilesystemNode::File(file) => file.size,
            FilesystemNode::Directory(dir) => dir.size(),
        }
    }
}

impl DirectoryNode {
    fn new(name: &str) -> Self {
        DirectoryNode {
            name: name.to_string(),
            parent: None,
            children: Vec::new(),
            cached_size: None,
        }
    }

    fn add_child(&mut self, child: Rc<RefCell<DirectoryNode>>) {
        self.children.push(child);
    }

    fn size(&mut self) -> FileSize {
        if let Some(size) = self.cached_size {
            return size;
        }

        let size = self
            .children
            .iter_mut()
            .map(|child| child.borrow_mut().size()) // Change child.borrow().size() to child.borrow_mut().size()
            .sum();
        self.cached_size = Some(size);
        size
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let root = Rc::new(RefCell::new(DirectoryNode::new("root")));
    let mut current_dir = root.clone();
    let mut is_listing = false;

    for line in input.lines() {
        if Some('$') == line.chars().next() {
            let parts = line.split(' ').collect::<Vec<_>>();

            match parts[1] {
                "cd" => match parts[2] {
                    ".." => {
                        let parent = {
                            let current_dir_ref = current_dir.borrow();
                            current_dir_ref.parent.clone()
                        };
                        current_dir = parent.unwrap().upgrade().unwrap();
                    }
                    "/" => {
                        current_dir = root.clone();
                    }
                    dir => {
                        let found_child = {
                            let current_dir_ref = current_dir.borrow();
                            current_dir_ref
                                .children
                                .iter()
                                .find(|child| child.borrow().name == dir)
                                .cloned()
                        };

                        if let Some(child) = found_child {
                            current_dir = child;
                        } else {
                            // If the directory does not exist, we create it instead

                            let new_dir = Rc::new(RefCell::new(DirectoryNode::new(dir)));

                            {
                                let mut current_dir_mut = current_dir.borrow_mut();
                                new_dir.borrow_mut().parent = Some(Rc::downgrade(&current_dir));
                                current_dir_mut.add_child(new_dir.clone());
                            }
                        }
                    }
                },
                "ls" => {
                    is_listing = true;
                }
                file_desc => {
                    let (size, name) = file_desc.split(' ').tuples().next().unwrap();

                    let file = Rc::new(RefCell::new(FilesystemNode::File(FileNode {
                        name: name.to_string(),
                        size: size.parse().unwrap(),
                    })));

                    {
                        let mut current_dir_mut = current_dir.borrow_mut();
                        current_dir_mut.add_child(file.clone()); // Change the type of the child parameter to Rc<RefCell<FilesystemNode>>
                    }
                }
            }
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
