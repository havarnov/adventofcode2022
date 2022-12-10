
enum Instruction {
    AddX(isize),
    NoOp,
}

fn parse_instruction(line: &str) -> Instruction {
    let mut split = line.split(' ');
    match split.next() {
        Some("noop") => Instruction::NoOp,
        Some("addx") => Instruction::AddX(split.next().expect("addx num").parse::<isize>().expect("parse addx num as isize")),
        _ => panic!("shouldn't happen"),
    }
}

fn main() {
    let lines = std::io::stdin().lines().collect::<Result<Vec<_>,_>>().expect("couldn't read lines");

    // part 1

    let mut sum = 0isize;
    let mut cycle = 0isize;
    let mut register = 1isize;

    for instruction in lines.iter().map(|line| parse_instruction(&line)) {
        use Instruction::*;
        match instruction {
            NoOp => {
                cycle += 1;
                if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
                    sum += register * cycle;
                }
            },
            AddX(x) => {
                for _ in 0..2 {
                    cycle += 1;
                    if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
                        sum += register * cycle;
                    }
                }

                register += x;
            },
        }
    }

    println!("{}", sum);

    // part 2

    let mut res = Vec::new();
    let mut cycle = 0isize;
    let mut register = 1isize;

    for instruction in lines.iter().map(|line| parse_instruction(&line)) {
        use Instruction::*;
        match instruction {
            NoOp => {
                cycle += 1;
                let pos = (cycle - 1) % 40;
                if pos >= register - 1 && pos <= register + 1 {
                    // set
                    res.push(true);
                } else {
                    // not set
                    res.push(false);
                }
            },
            AddX(x) => {
                for _ in 0..2 {
                    cycle += 1;
                    let pos = (cycle -1) % 40;
                    if pos >= register - 1 && pos <= register+ 1 {
                        // set
                        res.push(true);
                    } else {
                        // not set
                        res.push(false);
                    }
                }
                register += x;
            },
        }
    }

    for (idx, p) in res.iter().enumerate() {
        if idx % 40 == 0 {
            println!("");
        }

        if *p {
            print!("#");
        } else {
            print!(".");
        }
    }
}

