use itertools::Itertools;

pub fn run() {
    println!("Day 13");
    draw_line();

    let input = include_str!("../input/day_13.txt");

    let sum: usize = input
        .lines()
        .filter(|l| !l.is_empty())
        .tuple_windows()
        .step_by(2)
        .map(|(l1, l2)| (parse(l1), parse(l2)))
        .enumerate()
        .filter_map(|(c, (s1, s2))| {
            if s1.compare(&s2).unwrap() {
                Some(c + 1)
            } else {
                None
            }
        })
        .sum();

    println!("{}", sum);

    let mut signals = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse(l))
        .collect_vec();

    let sep_1 = parse("[[2]]");
    let sep_2 = parse("[[6]]");

    signals.push(sep_1.clone());
    signals.push(sep_2.clone());

    signals.sort();

    let one = signals
        .iter()
        .enumerate()
        .find(|(_, s)| **s == sep_1)
        .unwrap()
        .0
        + 1;
    let two = signals
        .iter()
        .enumerate()
        .find(|(_, s)| **s == sep_2)
        .unwrap()
        .0
        + 1;
    let product = one * two;

    println!("{}", product);
}

fn parse(s: &str) -> Signal {
    let mut parser = SignalParser::new(s);
    parser.parse()
}

struct SignalParser {
    line: Vec<char>,
    pos: usize,
}

impl SignalParser {
    fn new(line: &str) -> Self {
        SignalParser {
            line: line.chars().collect_vec(),
            pos: 0,
        }
    }

    fn parse(&mut self) -> Signal {
        self.parse_array()
    }

    fn parse_array(&mut self) -> Signal {
        if !(self.line[self.pos] == '[') {
            panic!("List must start with [");
        }

        self.pos += 1;

        let mut v = Vec::new();

        loop {
            v.push(match self.line[self.pos] {
                ']' => {
                    self.pos += 1;
                    break;
                }
                '[' => self.parse_array(),
                '0'..='9' => self.parse_number(),
                _ => panic!("Unexpected character"),
            });

            if self.line[self.pos] == ']' {
                self.pos += 1;
                break;
            }

            if !(self.line[self.pos] == ',') {
                panic!("List elements must be separated by ',' @ {}", self.pos);
            }
            self.pos += 1;
        }

        Signal::List(v)
    }

    fn parse_number(&mut self) -> Signal {
        let start = self.pos;
        while self.line[self.pos + 1].is_digit(10) {
            self.pos += 1;
        }

        let sub_string: String = self.line[start..=self.pos].iter().collect();
        let num: u32 = sub_string.parse().unwrap();

        self.pos += 1;

        Signal::Integer(num)
    }
}

#[derive(Clone)]
enum Signal {
    List(Vec<Signal>),
    Integer(u32),
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        let c = self.compare(other);

        if let None = c {
            true
        } else {
            false
        }
    }
}

impl Eq for Signal {}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let c = self.compare(other);

        match c {
            Some(true) => std::cmp::Ordering::Less,
            Some(false) => std::cmp::Ordering::Greater,
            None => std::cmp::Ordering::Equal,
        }
    }
}

impl Signal {
    fn compare(&self, other: &Signal) -> Option<bool> {
        match (self, other) {
            (Signal::Integer(left), Signal::Integer(right)) => {
                if left == right {
                    None
                } else {
                    Some(left < right)
                }
            }
            (Signal::List(left), Signal::List(right)) => {
                for i in 0..usize::max(left.len(), right.len()) {
                    let l = left.get(i);
                    let r = right.get(i);

                    let out = match (l, r) {
                        (Some(l), Some(r)) => l.compare(r),
                        (None, None) => None,
                        (None, Some(_)) => Some(true),
                        (Some(_), None) => Some(false),
                    };

                    if let Some(o) = out {
                        return Some(o);
                    }
                }
                None
            }
            pair @ (Signal::List(_), Signal::Integer(_)) => {
                let right = Signal::List(vec![pair.1.clone()]);
                pair.0.compare(&right)
            }
            pair @ (Signal::Integer(_), Signal::List(_)) => {
                let left = Signal::List(vec![pair.0.clone()]);
                left.compare(&pair.1)
            }
        }
    }

    fn _show(&self) {
        self._show_inner();
        println!();
    }

    fn _show_inner(&self) {
        match self {
            Signal::List(v) => {
                print!("[");
                for item in v {
                    item._show_inner();
                    print!(",");
                }
                print!("]");
            }
            Signal::Integer(i) => print!("{}", i),
        }
    }
}

fn draw_line() {
    println!("-------------------------------------------------------------------------");
}
