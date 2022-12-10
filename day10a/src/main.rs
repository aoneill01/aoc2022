// struct Program<'a> {
//     x: i32,
//     lines: Iterator<Item = &'a str>,
// }

// impl Program {
//     fn new(input: &'a str) -> Self {
//         Self {
//             x: 1,
//             lines: include_str!(input).lines(),
//         }
//     }
// }

// struct AddX {
//     cycle: u32,
//     previous: i32,
//     increment: i32,
// }

// impl AddX {
//     fn new(previous: i32, increment: i32) -> Self {
//         Self {
//             cycle: 0,
//             previous,
//             increment,
//         }
//     }
// }

// impl Iterator for AddX {
//     type Item = i32;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.cycle += 1;

//         if self.cycle < 2 {
//             Some(self.previous)
//         } else if self.cycle == 2 {
//             Some(self.previous + self.increment)
//         } else {
//             None
//         }
//     }
// }

// struct NoOp {
//     cycle: u32,
//     previous: i32,
// }

// impl NoOp {
//     fn new(previous: i32) -> Self {
//         Self { cycle: 0, previous }
//     }
// }

// impl Iterator for NoOp {
//     type Item = i32;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.cycle += 1;

//         if self.cycle == 1 {
//             Some(self.previous)
//         } else {
//             None
//         }
//     }
// }

fn main() {
    let instructions = include_str!("../input.txt").lines().map(parse_line);
    let mut x = 1;
    let mut cycle = 1;
    let mut next_target_cycle = 20;
    let mut sum = 0;

    for instruction in instructions {
        match instruction {
            ("addx", Some(arg)) => {
                cycle += 2;
                if cycle > next_target_cycle {
                    println!("{}", x);
                    sum += next_target_cycle * x;
                    next_target_cycle += 40;
                }
                x += arg;
            }
            ("noop", None) => {
                cycle += 1;
                if cycle > next_target_cycle {
                    println!("{}", x);
                    sum += next_target_cycle * x;
                    next_target_cycle += 40;
                }
            }
            _ => {
                panic!();
            }
        }
    }
    println!("{}", sum);
}

fn parse_line(line: &str) -> (&str, Option<i32>) {
    let mut split = line.split_whitespace();
    let instruction = split.next().unwrap();
    let mut arg = None;
    if instruction == "addx" {
        arg = split.next().unwrap().parse::<i32>().ok();
    }

    (instruction, arg)
}
