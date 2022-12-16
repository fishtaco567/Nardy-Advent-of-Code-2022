use std::collections::HashMap;

pub fn run() {
    let mut root = File::new();

    let input = include_str!("../input/day_7.txt");

    let mut cur_pos = Vec::new();
    for line in input.lines() {
        if line.chars().next().unwrap() == '$' {
            //user command
            let mut things = line.split_whitespace().skip(1);
            match things.next() {
                Some("cd") => {
                    let next = things.next().unwrap();

                    if next == ".." {
                        cur_pos.pop();
                    } else {
                        cur_pos.push(next)
                    }
                }
                Some("ls") => {
                    //implied
                }
                Some(_) => panic!("Unknown command"),
                None => panic!("No command"),
            }
        } else {
            let file = File::from_str(line);

            let mut cur_file = &mut root;
            for pos in cur_pos.iter() {
                if cur_file.children.contains_key(pos.clone()) {
                    cur_file = cur_file.children.get_mut(pos.clone()).unwrap();
                } else {
                    let f = File::from_descriptor(FileDescriptor::Directory(pos.to_string()));
                    let f = cur_file.add_child(f);
                    cur_file = f;
                }
            }

            cur_file.add_child(file);
        }
    }

    let mut under_100k = 0;
    root.count_under_100k(&mut under_100k);
    println!("{under_100k}");

    let total_size = root.size();
    println!("{total_size}");
    let available = 70000000 - total_size;
    println!("{available}");
    let req_delete = 30000000 - available;
    println!("{req_delete}");

    let mut smallest = u32::MAX;
    root.find_smallest_over(&mut smallest, req_delete);
    println!("{smallest}");
}

#[derive(Clone, Debug)]
enum FileDescriptor {
    File(u32, String),
    Directory(String),
}

#[derive(Debug)]
struct File {
    descriptor: FileDescriptor,
    children: HashMap<String, File>,
}

impl File {
    fn new() -> Self {
        Self {
            descriptor: FileDescriptor::Directory("/".to_owned()),
            children: HashMap::new(),
        }
    }

    fn from_descriptor(descriptor: FileDescriptor) -> Self {
        Self {
            descriptor: descriptor.clone(),
            children: HashMap::new(),
        }
    }

    fn from_str(s: &str) -> Self {
        let (first, second) = s.split_once(' ').unwrap();
        let descriptor = if first == "dir" {
            FileDescriptor::Directory(second.to_owned())
        } else {
            FileDescriptor::File(str::parse(first).unwrap(), second.to_owned())
        };

        Self::from_descriptor(descriptor)
    }

    fn name(&self) -> String {
        match &self.descriptor {
            FileDescriptor::File(_, s) => s.clone(),
            FileDescriptor::Directory(s) => s.clone(),
        }
    }

    fn add_child(&mut self, f: File) -> &mut File {
        let name = f.name();
        self.children.insert(name.clone(), f);

        self.children.get_mut(&name).unwrap()
    }

    fn size(&self) -> u32 {
        let self_size = if let FileDescriptor::File(s, _) = self.descriptor {
            s
        } else {
            0
        };
        self.children.values().map(|v| v.size()).sum::<u32>() + self_size
    }

    fn count_under_100k(&self, c: &mut u32) {
        let size = self.size();

        if let FileDescriptor::Directory(_) = self.descriptor {
            if size <= 100_000 {
                *c += size;
            }
        }

        self.children.values().for_each(|f| f.count_under_100k(c));
    }

    fn find_smallest_over(&self, smallest: &mut u32, over: u32) {
        let size = self.size();
        if size > over && size < *smallest {
            *smallest = size;
        }

        self.children
            .values()
            .for_each(|f| f.find_smallest_over(smallest, over));
    }
}
