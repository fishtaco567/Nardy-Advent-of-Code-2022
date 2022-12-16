use std::str::Lines;

use itertools::{Chunk, Itertools};

pub fn run() {
    println!("Day 11");
    draw_line();

    let test = false;
    // test = true;

    let input = if test {
        include_str!("../input/day_11_s.txt")
    } else {
        include_str!("../input/day_11.txt")
    };

    let monkeys_base = input
        .lines()
        .chunks(7)
        .into_iter()
        .map(|c| parse_monkey(c))
        .collect::<Vec<Monkey>>();
    let mut monkeys = monkeys_base.clone();

    let rounds = 20;

    let mut throw = Vec::<(u64, usize)>::new();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();

            for mut item in monkey.items.drain(..) {
                monkey.inspected += 1;

                match monkey.operation {
                    Operation::Add(x) => item += x,
                    Operation::Multiply(x) => item *= x,
                    Operation::AddSelf => item += item,
                    Operation::MultiplySelf => item *= item,
                }
                item = item / 3;

                if item % monkey.test == 0 {
                    throw.push((item, monkey.throw_true));
                } else {
                    throw.push((item, monkey.throw_false));
                }
            }

            for th in throw.drain(..) {
                monkeys[th.1].items.push(th.0);
            }
        }
    }

    let monkey_business = monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .collect::<Vec<u64>>()[..2]
        .iter()
        .fold(1, |acc, el| acc * el);
    println!("{}", monkey_business);

    let mut monkeys = monkeys_base.clone();
    let rounds = 10000;

    let factor = monkeys.iter().map(|m| m.test).fold(1, |acc, t| acc * t);

    let mut throw = Vec::<(u64, usize)>::new();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();

            for mut item in monkey.items.drain(..) {
                monkey.inspected += 1;

                match monkey.operation {
                    Operation::Add(x) => item += x,
                    Operation::Multiply(x) => item *= x,
                    Operation::AddSelf => item += item,
                    Operation::MultiplySelf => item *= item,
                }

                // dbg!(item);
                item = item % factor;

                if item % monkey.test == 0 {
                    throw.push((item, monkey.throw_true));
                } else {
                    throw.push((item, monkey.throw_false));
                }
            }

            for th in throw.drain(..) {
                monkeys[th.1].items.push(th.0);
            }
        }
    }

    let monkey_business = monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .collect::<Vec<u64>>()[..2]
        .iter()
        .fold(1, |acc, el| acc * el);
    println!("{}", monkey_business);
}

fn parse_monkey(mut s: Chunk<Lines>) -> Monkey {
    s.next();

    let items = s
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split(',')
        .map(|s| s.trim())
        .map(|s| str::parse(s).unwrap())
        .collect_vec();

    let mut op_str = s
        .next()
        .unwrap()
        .split_once('=')
        .unwrap()
        .1
        .trim()
        .split_whitespace();

    let _first = op_str.next().unwrap();
    let op = op_str.next().unwrap();
    let last = op_str.next().unwrap();

    let operation = match op {
        "+" => {
            if last == "old" {
                Operation::AddSelf
            } else {
                Operation::Add(str::parse(last).unwrap())
            }
        }
        "*" => {
            if last == "old" {
                Operation::MultiplySelf
            } else {
                Operation::Multiply(str::parse(last).unwrap())
            }
        }
        _ => panic!(),
    };

    let test = str::parse(s.next().unwrap().split_whitespace().last().unwrap()).unwrap();

    let throw_true = str::parse(s.next().unwrap().split_whitespace().last().unwrap()).unwrap();
    let throw_false = str::parse(s.next().unwrap().split_whitespace().last().unwrap()).unwrap();

    Monkey {
        items,
        operation,
        test,
        throw_true,
        throw_false,
        inspected: 0,
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    throw_true: usize,
    throw_false: usize,
    inspected: u64,
}

#[derive(Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    AddSelf,
    MultiplySelf,
}

fn draw_line() {
    println!("-------------------------------------------------------------------------");
}
