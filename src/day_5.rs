pub fn run() {
    println!("DAY 5");
    println!("-------------------------------------------------------------------------");

    let input = include_str!("../input/day_5.txt");
    let (setup, commands) = input.split_once("\r\n\r\n").unwrap();

    let mut stacks = Stacks::parse(setup);
    let mut stacks_part_2 = stacks.clone();

    for line in commands.lines() {
        let inst = Instruction::parse(line);
        stacks.do_instruction_9000(&inst);
        stacks_part_2.do_instruction_9001(&inst);
    }

    stacks.print_tops();
    stacks_part_2.print_tops();
}

#[derive(Clone)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn parse(s: &str) -> Self {
        let mut iter = s.lines().rev();

        let columns = (iter.next().unwrap().len() + 1) / 4;

        let mut stacks = Vec::new();

        for _ in 0..columns {
            stacks.push(Vec::<char>::new());
        }

        for line in iter {
            let mut c_iter = line.chars();

            let mut i = 0;

            loop {
                if let Some(c) = c_iter.next() {
                    if c == '[' {
                        stacks[i].push(c_iter.next().unwrap());
                    } else {
                        c_iter.next();
                    }
                    c_iter.next();
                    c_iter.next();
                } else {
                    break;
                }

                i += 1;
            }
        }

        Self { stacks }
    }

    fn do_instruction_9000(&mut self, inst: &Instruction) {
        for _ in 0..inst.amt {
            let p = self.stacks[inst.from].pop();

            if let Some(c) = p {
                self.stacks[inst.to].push(c);
            }
        }
    }

    fn do_instruction_9001(&mut self, inst: &Instruction) {
        let stack_size = self.stacks[inst.from].len();
        let mut s = self.stacks[inst.from].split_off(stack_size - inst.amt as usize);
        self.stacks[inst.to].append(&mut s);
    }

    fn print_tops(&self) {
        for col in self.stacks.iter() {
            print!("{}", col.last().unwrap());
        }

        println!("");
    }
}

struct Instruction {
    amt: u32,
    from: usize,
    to: usize,
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let mut split = s.split_whitespace();
        split.next();

        let amt = split.next().unwrap();

        split.next();

        let from = split.next().unwrap();

        split.next();

        let to = split.next().unwrap();

        Self {
            amt: str::parse(amt).unwrap(),
            from: str::parse::<usize>(from).unwrap() - 1,
            to: str::parse::<usize>(to).unwrap() - 1,
        }
    }
}
