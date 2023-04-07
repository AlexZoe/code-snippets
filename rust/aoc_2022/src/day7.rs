use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

struct FsType {
    size: usize,
    name: String,
    entries: Option<Vec<Rc<RefCell<FsType>>>>,
    parent: Option<Weak<RefCell<FsType>>>,
}

pub struct FileSystem {
    root: Rc<RefCell<FsType>>,
    //    dir_size_list: BTreeSet<usize>,
    dir_size_list: Vec<usize>,
}

impl FileSystem {
    pub fn new(layout_cmds: &str) -> Self {
        let mut fs = Self {
            root: Rc::new(RefCell::new(FsType {
                size: 0,
                name: "/".to_string(),
                entries: Some(Vec::new()),
                parent: None,
            })),
            dir_size_list: Vec::new(),
        };

        let mut current_level = fs.root.clone();

        for cmd in layout_cmds.lines() {
            let components: Vec<&str> = cmd.split(" ").collect();

            match components[0] {
                "$" => match components[1] {
                    "cd" => match components[2] {
                        ".." => {
                            let tmp =
                                Weak::upgrade(&current_level.borrow_mut().parent.as_ref().unwrap())
                                    .unwrap();
                            current_level = tmp;
                        }
                        "/" => current_level = fs.root.clone(),
                        _ => {
                            let index = current_level
                                .borrow_mut()
                                .entries
                                .as_mut()
                                .unwrap()
                                .iter()
                                .position(|x| x.borrow_mut().name == components[2])
                                .unwrap();
                            let tmp = current_level.borrow_mut().entries.as_ref().unwrap()
                                [index as usize]
                                .clone();
                            current_level = tmp;
                        }
                    },
                    "ls" => (),
                    _ => unreachable!(),
                },
                "dir" => current_level
                    .borrow_mut()
                    .entries
                    .as_mut()
                    .unwrap()
                    .push(Rc::new(RefCell::new(FsType {
                        size: 0,
                        name: components[1].to_owned(),
                        entries: Some(Vec::new()),
                        parent: Some(Rc::downgrade(&current_level)),
                    }))),

                _ => current_level
                    .borrow_mut()
                    .entries
                    .as_mut()
                    .unwrap()
                    .push(Rc::new(RefCell::new(FsType {
                        size: components[0].parse::<usize>().unwrap(),
                        name: components[1].to_owned(),
                        entries: None,
                        parent: Some(Rc::downgrade(&current_level)),
                    }))),
            }
        }

        let root_size = update_dir_sizes(
            fs.root.borrow().entries.as_ref().unwrap(),
            &mut fs.dir_size_list,
        );

        fs.root.borrow_mut().size = root_size;
        fs.dir_size_list.push(root_size);
        fs
    }

    pub fn print_layout(&self) {
        println!("- {}", self.root.borrow().name);
        print_internal(self.root.borrow().entries.as_ref().unwrap(), 0);
    }

    pub fn print_dir_sizes(&self) {
        println!("sizes: {:?}", self.dir_size_list);
    }

    pub fn dir_size_list(&self) -> Vec<usize> {
        self.dir_size_list.clone()
    }
}

fn update_dir_sizes(group: &Vec<Rc<RefCell<FsType>>>, list: &mut Vec<usize>) -> usize {
    let mut size = 0;
    for i in group {
        size += i.borrow().size;
        if i.borrow().entries.is_some() {
            let dir_size = update_dir_sizes(i.borrow().entries.as_ref().unwrap(), list);
            i.borrow_mut().size = dir_size;
            //            list.insert(dir_size);
            list.push(dir_size);
            size += dir_size;
        }
    }
    size
}

fn print_internal(group: &Vec<Rc<RefCell<FsType>>>, indent: usize) {
    for i in group {
        let repeated = "\t".repeat(indent);
        if i.borrow_mut().entries.is_some() {
            println!(
                "{} - {} (dir, size={})",
                repeated,
                i.borrow().name,
                i.borrow().size
            );
            print_internal(i.borrow().entries.as_ref().unwrap(), indent + 1);
        } else {
            println!(
                "{} - {} (file, size={})",
                repeated,
                i.borrow().name,
                i.borrow().size
            );
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::file;
    use super::*;

    #[test]
    fn print_file_sizes() {
        let fs = FileSystem::new(
            "322 file
32470 kfsadlj",
        );
        fs.print_layout();
        fs.print_dir_sizes();
    }

    #[test]
    fn print_sample_input() {
        let string = file::get_string_from_file("assets/day7_sample.txt");
        let fs = FileSystem::new(&string);
        fs.print_layout();
        fs.print_dir_sizes();
    }

    #[test]
    fn return_sample_dir_sum() {
        let string = file::get_string_from_file("assets/day7_sample.txt");
        let fs = FileSystem::new(&string);
        let mut sorted: Vec<usize> = fs.dir_size_list();
        sorted.sort();

        let max: usize = 100_000;
        assert_eq!(
            95441,
            sorted.iter().take_while(|x| x <= &&max).sum::<usize>()
        );
    }
}
