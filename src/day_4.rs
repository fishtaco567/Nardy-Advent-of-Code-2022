pub fn run() {
    let input = include_str!("../input/day_4.txt");

    println!("Day 4");
    println!("-------------------------------------------------------------------------");

    //Part 1
    let mut num_pt_1 = 0;
    let mut num_pt_2 = 0;
    for line in input.lines() {
        let ranges = process_line(line);

        if ranges.0.contains(&ranges.1) || ranges.1.contains(&ranges.0) {
            num_pt_1 += 1;
        }

        if ranges.0.overlaps(&ranges.1) {
            num_pt_2 += 1;
        }
    }

    println!("{num_pt_1}");
    println!("{num_pt_2}");
}

fn process_line(s: &str) -> (SectionRange, SectionRange) {
    let mut pair = s.split(',');
    let p1 = pair.next().unwrap();
    let p2 = pair.next().unwrap();

    (process_range(p1), process_range(p2))
}

fn process_range(s: &str) -> SectionRange {
    let mut pair = s.split('-');
    let p1 = pair.next().unwrap();
    let p2 = pair.next().unwrap();

    SectionRange {
        lower: str::parse(p1).unwrap(),
        upper: str::parse(p2).unwrap(),
    }
}

#[derive(Debug)]
struct SectionRange {
    lower: u32,
    upper: u32,
}

impl SectionRange {
    fn contains(&self, other: &SectionRange) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }

    fn overlaps(&self, other: &SectionRange) -> bool {
        self.lower <= other.upper && self.upper >= other.lower
    }
}
