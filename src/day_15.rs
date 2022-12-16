use itertools::Itertools;

pub fn run() {
    println!("Day 15");

    draw_line();

    let input = include_str!("../input/day_15.txt");

    let sensors = input.lines().map(|l| Sensor::parse(l)).collect_vec();

    let intervals = sensors
        .iter()
        .filter_map(|s| s.get_interval_for_row(2000000))
        .collect_vec();

    let (mut merged, mut final_intervals) = (true, intervals);
    while merged {
        (merged, final_intervals) = merge(final_intervals);
    }

    let sum: i32 = final_intervals.iter().map(|i| i.len()).sum();

    println!("{}", sum);

    let b = 4000000;
    let limit = Interval { start: 0, end: b };

    'outer: for i in 0..b {
        let intervals = sensors
            .iter()
            .filter_map(|s| s.get_interval_for_row(i))
            .collect_vec();

        let (mut merged, mut final_intervals) = (true, intervals);

        while merged {
            (merged, final_intervals) = merge(final_intervals);
        }

        let size: i32 = final_intervals
            .iter_mut()
            .map(|i| {
                i.clamp(&limit);
                i
            })
            .map(|i| i.len())
            .sum();

        if size != b + 1 {
            for x in 0..b {
                let mut c = true;
                for this_i in final_intervals.iter() {
                    if this_i.contains(x) {
                        c = false;
                        break;
                    }
                }

                if c {
                    println!("{}, {}", x, i);

                    let xf = x as u64 * b as u64;
                    let freq = xf + i as u64;
                    println!("{}", freq);
                    break 'outer;
                }
            }
        }
    }
}

fn merge(intervals: Vec<Interval>) -> (bool, Vec<Interval>) {
    let mut final_intervals: Vec<Interval> = Vec::new();

    let mut om = false;

    for interval in intervals.iter() {
        let mut merged = false;
        for fi in final_intervals.iter_mut() {
            match fi.try_merge(&interval) {
                Ok(_) => {
                    merged = true;
                    break;
                }
                Err(_) => (),
            }
        }

        if !merged {
            final_intervals.push(interval.clone());
        }

        om |= merged;
    }

    (om, final_intervals)
}

#[derive(Clone, Copy)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn contains(&self, x: i32) -> bool {
        if x >= self.start && x <= self.end {
            true
        } else {
            false
        }
    }

    fn try_merge(&mut self, other: &Interval) -> Result<(), ()> {
        if other.start > self.end + 1 || self.start > other.end + 1 {
            Err(())
        } else {
            let os = self.start.min(other.start);
            let oe = self.end.max(other.end);
            self.start = os;
            self.end = oe;
            Ok(())
        }
    }

    fn clamp(&mut self, other: &Interval) {
        self.start = self.start.max(other.start);
        self.end = self.end.min(other.end);
    }

    fn len(&self) -> i32 {
        self.end + 1 - self.start
    }
}

struct Sensor {
    pos: (i32, i32),
    min_dist: u32,
}

impl Sensor {
    pub fn parse(s: &str) -> Self {
        let mut split = s.split_whitespace();
        split.next();
        split.next();

        let x1: i32 = str::parse(
            split
                .next()
                .unwrap()
                .split_once("=")
                .unwrap()
                .1
                .trim_end_matches(","),
        )
        .unwrap();

        let y1: i32 = str::parse(
            split
                .next()
                .unwrap()
                .split_once("=")
                .unwrap()
                .1
                .trim_end_matches(":"),
        )
        .unwrap();

        split.next();
        split.next();
        split.next();
        split.next();

        let x2: i32 = str::parse(
            split
                .next()
                .unwrap()
                .split_once("=")
                .unwrap()
                .1
                .trim_end_matches(","),
        )
        .unwrap();

        let y2: i32 = str::parse(split.next().unwrap().split_once("=").unwrap().1).unwrap();

        let min_dist = x1.abs_diff(x2) + y1.abs_diff(y2);

        Self {
            pos: (x1, y1),
            min_dist,
        }
    }

    fn get_interval_for_row(&self, row: i32) -> Option<Interval> {
        let off = self.pos.1.abs_diff(row);

        if off > self.min_dist {
            return None;
        }

        let extent = (self.min_dist - off) as i32;

        Some(Interval {
            start: self.pos.0 - extent,
            end: self.pos.0 + extent,
        })
    }
}

fn draw_line() {
    println!("-------------------------------------------------------------------------");
}
