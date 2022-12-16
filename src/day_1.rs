use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() {
    println!("Day 1");
    println!("-------------------------------------------------------------------------");

    let mut elves = parse();

    elves.sort_by_cached_key(|elf| elf.sum());

    //Part one
    println!(
        "The elf carrying the most calories is carrying {} fruit calories",
        elves.last().expect("Must have one elf").sum()
    );

    println!("-------------------------------------------------------------------------");

    //Part two
    println!(
        "The top three elves are carrying {} fruit calories",
        &elves[(elves.len() - 3)..]
            .iter()
            .fold(0, |acc, elf| acc + elf.sum())
    );
    println!("-------------------------------------------------------------------------");
}

fn parse() -> Vec<Elf> {
    let file = File::open("input/day_1.txt").expect("Failed to open day_1.txt");

    let mut elves = Vec::new();
    elves.push(Elf::new());

    for (_line_num, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(l) = line {
            if l == "" {
                elves.push(Elf::new());
            } else {
                let fruit = str::parse::<i32>(&l).expect("Failed to parse {line}@{_line_num}");
                if let Some(elf) = elves.last_mut() {
                    elf.add_fruit(fruit);
                }
            }
        }
    }

    return elves;
}

struct Elf {
    fruit: Vec<i32>,
}

impl Elf {
    fn new() -> Self {
        Elf { fruit: Vec::new() }
    }

    fn sum(&self) -> i32 {
        self.fruit.iter().sum()
    }

    fn add_fruit(&mut self, fruit: i32) {
        self.fruit.push(fruit);
    }
}
