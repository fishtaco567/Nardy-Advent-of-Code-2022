use std::collections::HashSet;

pub fn run() {
    println!("Day 8");
    println!("-------------------------------------------------------------------------");

    let input = include_str!("../input/day_8.txt");

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut grid = vec!['0'; width * height];

    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            grid[i + j * width] = c;
        }
    }

    let mut seen = HashSet::new();

    for i in 0..width {
        let mut max_height = '\0';
        for j in 0..height {
            if grid[i + j * width] > max_height {
                max_height = grid[i + j * width];
                seen.insert((i, j));
            }
        }
    }

    for i in 0..width {
        let mut max_height = '\0';
        for j in (0..height).rev() {
            if grid[i + j * width] > max_height {
                max_height = grid[i + j * width];
                seen.insert((i, j));
            }
        }
    }

    for j in 0..width {
        let mut max_height = '\0';
        for i in 0..height {
            if grid[i + j * width] > max_height {
                max_height = grid[i + j * width];
                seen.insert((i, j));
            }
        }
    }

    for j in 0..width {
        let mut max_height = '\0';
        for i in (0..height).rev() {
            if grid[i + j * width] > max_height {
                max_height = grid[i + j * width];
                seen.insert((i, j));
            }
        }
    }

    let o = seen.len();

    println!("The number of trees you can see from the edges is {o}");
    println!("-------------------------------------------------------------------------");

    let mut best = 0;
    for i in 1..(width - 1) {
        for j in 1..(height - 1) {
            let u = count_dir(&grid, width, height, (0, 1), (i, j));
            let d = count_dir(&grid, width, height, (0, -1), (i, j));
            let r = count_dir(&grid, width, height, (1, 0), (i, j));
            let l = count_dir(&grid, width, height, (-1, 0), (i, j));
            let s = u * d * r * l;

            if s > best {
                best = s;
            }
        }
    }
    println!("The best place has a scenic score of {best}");
    println!("-------------------------------------------------------------------------");
}

fn count_dir(
    grid: &Vec<char>,
    width: usize,
    height: usize,
    dir: (i32, i32),
    pos: (usize, usize),
) -> u32 {
    let base_height = grid[pos.0 + pos.1 * width];

    let mut base_pos = pos;

    let mut c = 0;
    loop {
        base_pos.0 = (base_pos.0 as i32 + dir.0) as usize;
        base_pos.1 = (base_pos.1 as i32 + dir.1) as usize;

        c += 1;

        if grid[base_pos.0 + base_pos.1 * width] >= base_height {
            break;
        }

        if c > 1000000 {
            panic!("too many");
        }

        if base_pos.0 == 0 || base_pos.1 == 0 || base_pos.0 >= width - 1 || base_pos.1 >= height - 1
        {
            break;
        }
    }

    c
}
