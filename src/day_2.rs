use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() {
    println!("Day 2");
    println!("-------------------------------------------------------------------------");

    let file = File::open("input/day_2.txt").expect("input/day_2.txt must exist");

    let buf_read = BufReader::new(file);

    let mut naive_points = 0;
    let mut real_points = 0;

    for line in buf_read.lines() {
        if let Ok(line) = line {
            let mut parts = line.split_whitespace();
            let p1 = parts.next().expect("Expected format is '[other] [me]'");
            let p2 = parts.next().expect("Expected format is '[other] [me]'");

            //Part 1
            let other = Move::parse_from(p1);
            let me = Move::parse_from(p2);

            let outcome = me.beats(&other);
            naive_points += me.points() + outcome.points();

            //Part 2
            let other = other; //We got this right - don't reparse
            let outcome = Outcome::parse_from(p2);

            let me = other.get_for_outcome(&outcome);
            real_points += me.points() + outcome.points();
        }
    }

    //Part 1
    println!("I would get {naive_points} points incorrectly following the strategy guide");

    println!("-------------------------------------------------------------------------");

    //Part 2
    println!("I would get {real_points} points correctly following the stretegy guide");
    println!("-------------------------------------------------------------------------");
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn points(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }

    fn parse_from(s: &str) -> Outcome {
        assert!(s.len() == 1);

        match s.chars().next().unwrap() {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Illegal character"),
        }
    }
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn parse_from(s: &str) -> Self {
        assert!(s.len() == 1);

        match s.chars().next().unwrap() {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Illegal character"),
        }
    }

    fn beats(&self, other: &Self) -> Outcome {
        match self {
            Move::Rock => match other {
                Move::Rock => Outcome::Draw,
                Move::Paper => Outcome::Lose,
                Move::Scissors => Outcome::Win,
            },
            Move::Paper => match other {
                Move::Rock => Outcome::Win,
                Move::Paper => Outcome::Draw,
                Move::Scissors => Outcome::Lose,
            },
            Move::Scissors => match other {
                Move::Rock => Outcome::Lose,
                Move::Paper => Outcome::Win,
                Move::Scissors => Outcome::Draw,
            },
        }
    }

    fn get_for_outcome(&self, outcome: &Outcome) -> Self {
        match self {
            Move::Rock => match outcome {
                Outcome::Win => Move::Paper,
                Outcome::Lose => Move::Scissors,
                Outcome::Draw => Move::Rock,
            },
            Move::Paper => match outcome {
                Outcome::Win => Move::Scissors,
                Outcome::Lose => Move::Rock,
                Outcome::Draw => Move::Paper,
            },
            Move::Scissors => match outcome {
                Outcome::Win => Move::Rock,
                Outcome::Lose => Move::Paper,
                Outcome::Draw => Move::Scissors,
            },
        }
    }

    fn points(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}
