use std::time::Instant;

use itertools::Itertools;

pub fn run() {
    println!("Day 14");
    draw_line();

    let input = include_str!("../input/day_14.txt");

    let mut grid = Grid::new(input, false);

    let fallen = pour_sand(&mut grid);
    println!("{}", fallen);
    //grid.show();

    let mut other_grid = Grid::new(input, true);

    let fallen = pour_sand(&mut other_grid);

    println!("{}", fallen);
    //other_grid.show();
}

fn pour_sand(grid: &mut Grid) -> u32 {
    let source = (500, 0);

    let mut fallen = 0;

    let mut fallen_out = false;
    while !fallen_out {
        let mut sand_pos = source;

        while let Some(fall_pos) = check_fall(grid, sand_pos) {
            match fall_pos {
                FallPos::Left => sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1),
                FallPos::Center => sand_pos = (sand_pos.0, sand_pos.1 + 1),
                FallPos::Right => sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1),
            }

            if !grid.contains((sand_pos.0, sand_pos.1 + 1)) {
                fallen_out = true;
                break;
            }
        }

        if !fallen_out {
            grid.set(sand_pos, Tile::Sand);
            fallen += 1;
        }

        if sand_pos == source {
            break;
        }
    }

    fallen
}

fn check_fall(grid: &Grid, pos: (usize, usize)) -> Option<FallPos> {
    if grid.empty((pos.0, pos.1 + 1)) {
        Some(FallPos::Center)
    } else if grid.empty((pos.0 - 1, pos.1 + 1)) {
        Some(FallPos::Left)
    } else if grid.empty((pos.0 + 1, pos.1 + 1)) {
        Some(FallPos::Right)
    } else {
        None
    }
}

enum FallPos {
    Left,
    Center,
    Right,
}

struct Grid {
    grid: Vec<Tile>,
    width: usize,
    height: usize,
    min: (usize, usize),
}

impl Grid {
    pub fn new(s: &str, include_floor: bool) -> Self {
        let mut paths = s.lines().map(|l| Path::new(l)).collect_vec();

        let mut bounds = Bounds {
            min: (500, 0),
            max: (500, 0),
        };

        for path in paths.iter() {
            bounds.expand_by(&path.bounds());
        }

        if include_floor {
            let y = bounds.max.1 + 2;
            let bottom_path = Path {
                path: vec![(500 - y, y), (500 + y, y)],
            };
            bounds.expand_by(&bottom_path.bounds());
            paths.push(bottom_path);
        }

        //One would be safe but 2 would be fine
        bounds.min.0 -= 2;
        bounds.max.0 += 2;

        let min = bounds.min;

        let width = bounds.width() + 1;
        let height = bounds.height() + 1;

        let mut grid = Grid {
            grid: vec![Tile::Empty; width * height],
            width,
            height,
            min,
        };

        for path in paths.iter() {
            for (pt1, pt2) in path.path.iter().tuple_windows() {
                let min_x = pt1.0.min(pt2.0);
                let min_y = pt1.1.min(pt2.1);
                let max_x = pt1.0.max(pt2.0);
                let max_y = pt1.1.max(pt2.1);
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        grid.set((x, y), Tile::Rock);
                    }
                }
            }
        }

        grid
    }

    fn empty(&self, pos: (usize, usize)) -> bool {
        let off_pos = self.off_pos(pos);
        self.grid[off_pos.0 + off_pos.1 * self.width] == Tile::Empty
    }

    fn set(&mut self, pos: (usize, usize), tile: Tile) {
        let off_pos = self.off_pos(pos);
        self.grid[off_pos.0 + off_pos.1 * self.width] = tile;
    }

    fn off_pos(&self, pos: (usize, usize)) -> (usize, usize) {
        (pos.0 - self.min.0, pos.1 - self.min.1)
    }

    fn contains(&self, pos: (usize, usize)) -> bool {
        let off_pos = self.off_pos(pos);

        off_pos.0 > 0 && off_pos.0 < self.width && off_pos.1 > 0 && off_pos.1 < self.height
    }

    fn show(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.grid[x + y * self.width] {
                    Tile::Empty => print!("."),
                    Tile::Rock => print!("#"),
                    Tile::Sand => print!("o"),
                }
            }
            println!();
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Rock,
    Sand,
}

struct Path {
    path: Vec<(usize, usize)>,
}

impl Path {
    fn new(s: &str) -> Self {
        Self {
            path: s
                .split(" -> ")
                .map(|p| {
                    p.split(",")
                        .map(|n| str::parse::<usize>(n).unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_vec(),
        }
    }

    fn bounds(&self) -> Bounds {
        let mut min_x = usize::MAX;
        let mut max_x = usize::MIN;
        let mut max_y = usize::MIN;

        for pt in self.path.iter() {
            if pt.0 < min_x {
                min_x = pt.0;
            }

            if pt.0 > max_x {
                max_x = pt.0;
            }

            if pt.1 > max_y {
                max_y = pt.1;
            }
        }

        Bounds {
            min: (min_x, 0),
            max: (max_x, max_y),
        }
    }
}

struct Bounds {
    min: (usize, usize),
    max: (usize, usize),
}

impl Bounds {
    fn expand_by(&mut self, other: &Bounds) {
        self.min.0 = self.min.0.min(other.min.0);
        self.min.1 = self.min.1.min(other.min.1);
        self.max.0 = self.max.0.max(other.max.0);
        self.max.1 = self.max.1.max(other.max.1);
    }

    fn width(&self) -> usize {
        self.max.0 - self.min.0
    }

    fn height(&self) -> usize {
        self.max.1 - self.min.1
    }
}

fn draw_line() {
    println!("-------------------------------------------------------------------------");
}
