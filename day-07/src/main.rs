use std::{cell::RefCell, fs::read_to_string, rc::Rc};

fn main() {
    let root = parse("input.txt");
    println!("part1 solution {}", find_sum_of_dirs_bellow_limit(&root));
    println!("part2 solution {}", find_size_of_dir_to_delete(&root));
}

fn find_size_of_dir_to_delete(root: &Rc<RefCell<FileEntry>>) -> usize {
    let current_used_size = root.borrow().size();
    let target_size = 30000000 - (70000000 - current_used_size);
    find_size_of_dir_to_delete_rec(root, usize::MAX, target_size)
}

fn find_size_of_dir_to_delete_rec(
    root: &Rc<RefCell<FileEntry>>,
    min: usize,
    limit: usize,
) -> usize {
    usize::min(
        match &*root.borrow() {
            FileEntry::Dir((_, items, _)) => items
                .iter()
                .filter(|item| item.borrow().is_dir())
                .map(|item| {
                    let size = item.borrow().size();
                    let curr_min = if size >= limit && size < min {
                        size
                    } else {
                        min
                    };
                    usize::min(
                        find_size_of_dir_to_delete_rec(item, curr_min, limit),
                        curr_min,
                    )
                })
                .min()
                .unwrap_or(min),
            FileEntry::File(_) => usize::MAX,
        },
        min,
    )
}

fn find_sum_of_dirs_bellow_limit(root: &Rc<RefCell<FileEntry>>) -> usize {
    let mut sum = 0;
    find_sum_of_dirs_bellow_limit_rec(root, &mut sum, 100000);
    sum
}

fn find_sum_of_dirs_bellow_limit_rec(root: &Rc<RefCell<FileEntry>>, sum: &mut usize, limit: usize) {
    match &*root.borrow() {
        FileEntry::Dir((_, items, _)) => items.iter().for_each(|item| {
            if !item.borrow().is_dir() {
                return;
            }
            let size = item.borrow().size();
            if size < limit {
                *sum += size;
            }
            find_sum_of_dirs_bellow_limit_rec(item, sum, limit);
        }),
        FileEntry::File(_) => {}
    }
}

type DirInfo = (
    String,
    Vec<Rc<RefCell<FileEntry>>>,
    Option<Rc<RefCell<FileEntry>>>,
);

#[derive(Debug, PartialEq, Eq)]
enum FileEntry {
    File((String, usize, Option<Rc<RefCell<FileEntry>>>)),
    Dir(DirInfo),
}
impl FileEntry {
    fn parent(&self) -> Option<Rc<RefCell<FileEntry>>> {
        match self {
            FileEntry::Dir(dir_info) => {
                let (_dir_name, _items, parent) = dir_info;
                parent.clone()
            }
            _ => unreachable!(),
        }
    }

    fn items(&self) -> &Vec<Rc<RefCell<FileEntry>>> {
        match self {
            FileEntry::Dir((_dir_name, items, _parent)) => items,
            _ => unreachable!(),
        }
    }

    fn add_item(&mut self, item: FileEntry) {
        match self {
            FileEntry::Dir(dir_info) => {
                let (_dir_name, items, _parent) = dir_info;
                items.push(Rc::new(RefCell::new(item)));
            }
            _ => unreachable!(),
        }
    }

    fn size(&self) -> usize {
        match self {
            FileEntry::Dir((_, items, _)) => items.iter().map(|item| item.borrow().size()).sum(),
            FileEntry::File((_, size, _)) => *size,
        }
    }

    fn is_dir(&self) -> bool {
        matches!(self, FileEntry::Dir(_))
    }
}

fn parse(filename: &str) -> Rc<RefCell<FileEntry>> {
    let input = read_to_string(filename).expect("failed to read file");
    let mut root = None;
    let mut current = None;
    let mut lines_iter = input.lines().peekable();
    while lines_iter.peek().is_some() {
        let cmd = lines_iter.next().unwrap();
        if cmd.starts_with("$ cd") {
            let dir_name = cmd.strip_prefix("$ cd ").unwrap();
            if dir_name == "/" {
                if root.is_none() {
                    root = Some(Rc::new(RefCell::new(FileEntry::Dir((
                        dir_name.into(),
                        vec![],
                        None,
                    )))));
                }
                current = root.clone();
            } else if dir_name == ".." {
                if let Some(curr_dir) = current {
                    let curr_item = curr_dir.borrow();
                    current = curr_item.parent();
                }
            } else if let Some(curr_dir) = current {
                current = curr_dir.borrow()
                    .items()
                    .iter()
                    .find(|dir| matches!(&*dir.borrow(), FileEntry::Dir((name, _,_)) if name == dir_name))
                    .cloned()
            }
        } else if cmd == "$ ls" {
            while let Some(next_line) = lines_iter.peek() {
                if next_line.starts_with("$ ") {
                    break;
                }
                let item_str = lines_iter.next().unwrap();
                let item = if item_str.starts_with("dir ") {
                    let dir_name = item_str.strip_prefix("dir ").unwrap();
                    FileEntry::Dir((dir_name.into(), vec![], current.clone()))
                } else {
                    let (size, file_name) = item_str.split_once(' ').unwrap();
                    FileEntry::File((file_name.into(), size.parse().unwrap(), current.clone()))
                };
                if let Some(current_ref) = &current {
                    current_ref.borrow_mut().add_item(item);
                }
            }
        }
    }
    root.unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{find_size_of_dir_to_delete, find_sum_of_dirs_bellow_limit, parse};

    #[test]
    fn part1_test() {
        let root = parse("test-input.txt");
        assert_eq!(find_sum_of_dirs_bellow_limit(&root), 95437);
    }

    #[test]
    fn part2_test() {
        let root = parse("test-input.txt");
        assert_eq!(find_size_of_dir_to_delete(&root), 24933642);
    }
}
