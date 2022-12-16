use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::Iter,
};

pub fn run() {
    println!("Day 12");
    draw_line();

    let input = include_str!("../input/day_12.txt");

    let grid = CharGrid::from_text(input);

    let start = grid.start();
    let path_len = path(&grid, start);

    println!("{}", path_len.unwrap());

    let a_pos = grid.character_list(b'a');

    let mut shortest_path = u32::MAX;
    for a in a_pos {
        let path_len = path(&grid, a);

        if let Some(p) = path_len {
            if p < shortest_path {
                shortest_path = p;
            }
        }
    }
    println!("{}", shortest_path);
}

struct CharGrid {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl CharGrid {
    fn from_text(text: &'static str) -> Self {
        let width = text.lines().next().unwrap().len();
        let height = text.lines().count();

        Self {
            grid: text
                .chars()
                .map(|c| c as u32 as u8)
                .filter(|b| *b != b'\n' && *b != b'\r')
                .collect(),
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x < self.width && y < self.height {
            Some(self.grid[x + y * self.width])
        } else {
            None
        }
    }

    fn start(&self) -> (usize, usize) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(x, y).unwrap() == b'S' {
                    return (x, y);
                }
            }
        }

        panic!("Start not found");
    }

    fn end(&self) -> (usize, usize) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.get(x, y).unwrap() == b'E' {
                    return (x, y);
                }
            }
        }

        panic!("Start not found");
    }

    fn show(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}",
                    char::from_u32(self.get(x, y).unwrap() as u32).unwrap()
                );
            }
            println!("");
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> Neighbors {
        Neighbors {
            grid: self,
            x,
            y,
            idx: 0,
        }
    }

    fn character_list(&self, find: u8) -> CharacterList {
        CharacterList {
            grid: self,
            find,
            idx: 0,
        }
    }
}

struct CharacterList<'a> {
    grid: &'a CharGrid,
    find: u8,
    idx: usize,
}

impl<'a> Iterator for CharacterList<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.grid.grid.len() {
            return None;
        }

        for x in self.idx..self.grid.grid.len() {
            if self.grid.grid[x] == self.find {
                self.idx = x + 1;
                return Some((x % self.grid.width, x / self.grid.width));
            }
        }

        None
    }
}

struct Neighbors<'a> {
    grid: &'a CharGrid,
    x: usize,
    y: usize,
    idx: u32,
}

struct Cell {
    pos: (usize, usize),
    element: Option<u8>,
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        match self.idx {
            0 => {
                self.idx += 1;
                Some(Cell {
                    pos: (self.x, self.y + 1),
                    element: self.grid.get(self.x, self.y + 1),
                })
            }
            1 => {
                if self.y > 0 {
                    self.idx += 1;
                    Some(Cell {
                        pos: (self.x, self.y - 1),
                        element: self.grid.get(self.x, self.y - 1),
                    })
                } else {
                    self.idx += 1;
                    Some(Cell {
                        pos: (self.x, self.y),
                        element: None,
                    })
                }
            }
            2 => {
                self.idx += 1;
                Some(Cell {
                    pos: (self.x + 1, self.y),
                    element: self.grid.get(self.x + 1, self.y),
                })
            }
            3 => {
                if self.x > 0 {
                    self.idx += 1;
                    Some(Cell {
                        pos: (self.x - 1, self.y),
                        element: self.grid.get(self.x - 1, self.y),
                    })
                } else {
                    self.idx += 1;
                    Some(Cell {
                        pos: (self.x, self.y),
                        element: None,
                    })
                }
            }
            _ => None,
        }
    }
}

struct OpenElement {
    pos: (usize, usize),
    element: u8,
    dist: u32,
}

fn path(grid: &CharGrid, start: (usize, usize)) -> Option<u32> {
    let end = grid.end();

    let mut open_set = VecDeque::new();
    let mut explored = HashSet::new();

    open_set.push_back(OpenElement {
        pos: start,
        element: b'a',
        dist: 0,
    });
    explored.insert(start);

    while !open_set.is_empty() {
        let current = open_set.pop_front().unwrap();

        if current.pos == end {
            return Some(current.dist);
        }

        for neighbor in grid.neighbors(current.pos.0, current.pos.1) {
            if explored.contains(&neighbor.pos) {
                continue;
            }

            match neighbor.element {
                Some(x)
                    if (x != b'E' && x <= current.element + 1)
                        || (x == b'E' && current.element >= b'y') =>
                {
                    explored.insert(neighbor.pos);
                    open_set.push_back(OpenElement {
                        pos: neighbor.pos,
                        element: x,
                        dist: current.dist + 1,
                    });
                }
                _ => {}
            }
        }
    }

    None
}

fn draw_line() {
    println!("-------------------------------------------------------------------------");
}
