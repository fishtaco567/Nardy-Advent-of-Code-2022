pub fn run() {
    println!("Day 10");

    draw_line();

    let mut debug = false;

    // debug = true;

    let input = if debug {
        include_str!("../input/day_10_s.txt")
    } else {
        include_str!("../input/day_10.txt")
    };

    let mut instructions = Vec::new();

    for line in input.lines() {
        match line {
            "noop" => {
                instructions.push(Instruction::Noop);
            }
            _ => {
                let (inst, num) = line.split_once(' ').unwrap();
                debug_assert!(inst == "addx");

                let num = str::parse::<i32>(num).unwrap();

                instructions.push(Instruction::Add(num));
            }
        }
    }

    let mut reg_x = 1;
    let mut cycles = 0;

    let mut signal_acc = 0;

    for inst in instructions.iter() {
        let n_cycles = inst.cycles();

        for _ in 0..n_cycles {
            cycles += 1;

            if (cycles - 20) % 40 == 0 {
                signal_acc += reg_x * cycles;
            }
        }

        inst.execute(&mut reg_x);
    }

    println!("{signal_acc} {cycles}");

    let mut grid = ['.'; 40 * 6];

    let mut reg_x = 1;
    let mut cycles = 0;

    for inst in instructions.iter() {
        let n_cycles = inst.cycles();

        for _ in 0..n_cycles {
            let (px, py) = (cycles % 40, cycles / 40);

            let idx = (px + py * 40) as usize;

            if i32::abs(px - reg_x) <= 1 && idx <= grid.len() {
                grid[idx] = '#';
            }

            cycles += 1;
        }

        inst.execute(&mut reg_x);
    }

    let mut x = 0;
    for g in grid {
        print!("{}", g);
        x += 1;

        if x >= 40 {
            println!("");
            x = 0;
        }
    }
}

enum Instruction {
    Add(i32),
    Noop,
}

impl Instruction {
    fn cycles(&self) -> u32 {
        match self {
            Instruction::Add(_) => 2,
            Instruction::Noop => 1,
        }
    }

    fn execute(&self, reg_x: &mut i32) {
        match self {
            Instruction::Add(x) => *reg_x += x,
            Instruction::Noop => {}
        }
    }
}

fn draw_line() {
    println!("-------------------------------------------------------------------------");
}
