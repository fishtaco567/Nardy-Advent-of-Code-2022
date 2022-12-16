pub fn run() {
    println!("Day 3");
    println!("-------------------------------------------------------------------------");

    let input = include_str!("../input/day_3.txt");

    //part 1
    let mut sum = 0;

    for line in input.lines() {
        let o = find_overlapping_in_line(line);

        sum += char_to_score(o);
    }

    println!("The sum of the scores of the incorrect items in the elves' bag is {sum}");

    println!("-------------------------------------------------------------------------");

    let mut sum = 0;

    for line in input
        .lines()
        .zip(input.lines().skip(1))
        .zip(input.lines().skip(2))
        .step_by(3)
    {
        let o = find_overlapping_in_three(line);

        sum += char_to_score(o);
    }

    println!("The sum of scores of the items common to the group's bags is {sum}");

    println!("-------------------------------------------------------------------------");
}

fn find_overlapping_in_line(line: &str) -> char {
    let (one, two) = line.split_at(line.len() / 2);

    for c in one.chars() {
        if two.contains(c) {
            return c;
        }
    }

    panic!("Didn't find overlapping!");
}

fn find_overlapping_in_three(lines: ((&str, &str), &str)) -> char {
    let ((one, two), three) = lines;

    for c in one.chars() {
        if two.contains(c) && three.contains(c) {
            return c;
        }
    }

    panic!("Didn't find overlapping in three");
}

fn char_to_score(c: char) -> u32 {
    if c >= 'A' && c <= 'Z' {
        return c as u32 - 0x41 + 27;
    } else if c >= 'a' && c <= 'z' {
        return c as u32 - 0x61 + 1;
    }

    panic!("Char out of range");
}
