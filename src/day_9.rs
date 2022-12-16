use std::collections::HashSet;

pub fn run() {
    println!("Day 9");
    draw_line();

    let input = include_str!("../input/day_9.txt");
    // let input = include_str!("../input/day_9_s.txt");

    let mut positions = HashSet::<(i32, i32)>::new();

    let mut head = (0, 0);
    let mut tails = [(0, 0); 9];

    for line in input.lines() {
        let l = *tails.last().unwrap();
        positions.insert(l);

        let (dir, amt) = line.split_once(' ').unwrap();

        let amt = str::parse::<i32>(amt).unwrap();

        for _ in 0..amt {
            match dir {
                "R" => head = (head.0 + 1, head.1),
                "L" => head = (head.0 - 1, head.1),
                "U" => head = (head.0, head.1 + 1),
                "D" => head = (head.0, head.1 - 1),
                _ => {
                    panic!("F")
                }
            }

            move_tail(&mut head, &mut tails[0]);
            for i in 0..(tails.len() - 1) {
                let p = tails[i];
                move_tail(&p, &mut tails[i + 1]);
            }
            let l = *tails.last().unwrap();
            positions.insert(l);
        }
    }

    let c = positions.len();

    println!("The end of the tail touches {c} positions");
}

fn move_tail(head: &(i32, i32), tail: &mut (i32, i32)) {
    if tail.0 < head.0 - 1 && tail.1 == head.1 {
        tail.0 += 1;
    } else if tail.0 < head.0 - 1 {
        tail.0 += 1;
        tail.1 += (head.1 - tail.1).signum();
    }

    if tail.0 > head.0 + 1 && tail.1 == head.1 {
        tail.0 -= 1;
    } else if tail.0 > head.0 + 1 {
        tail.0 -= 1;
        tail.1 += (head.1 - tail.1).signum();
    }

    if tail.1 > head.1 + 1 && tail.0 == head.0 {
        tail.1 -= 1;
    } else if tail.1 > head.1 + 1 {
        tail.1 -= 1;
        tail.0 += (head.0 - tail.0).signum();
    }

    if tail.1 < head.1 - 1 && tail.0 == head.0 {
        tail.1 += 1;
    } else if tail.1 < head.1 - 1 {
        tail.1 += 1;
        tail.0 += (head.0 - tail.0).signum();
    }
}

fn draw_line() {
    println!("-------------------------------------------------------------------------");
}
