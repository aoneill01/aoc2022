fn main() {
    let instructions = include_str!("../input.txt").lines().map(parse_line);
    let mut x = 1;
    let mut values = Vec::<i32>::new();
    values.push(x);

    for instruction in instructions {
        match instruction {
            ("addx", Some(arg)) => {
                values.push(x);
                x += arg;
                values.push(x);
            }
            ("noop", None) => {
                values.push(x);
            }
            _ => {
                panic!();
            }
        }
    }

    for row in 0..6 {
        for position in 0i32..40 {
            let x = values[position as usize + row * 40];

            if x - 1 <= position && x + 1 >= position {
                print!("#");
            } else {
                print!(" ");
            }
        }

        println!("");
    }
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
