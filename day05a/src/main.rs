use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Instruction {
    count: i32,
    from: i32,
    to: i32,
}

fn main() {
    let (stacks_str, instructions_str) = include_str!("../input.txt")
        .split("\n\n")
        .tuples()
        .next()
        .unwrap();

    let mut stacks = parse_stacks(stacks_str);
    let instructions = parse_instructions(instructions_str);
    run(&mut stacks, &instructions);

    let answer: String = stacks
        .iter()
        .map(|stack| *stack.last().unwrap() as char)
        .collect();

    println!("{}", answer);
}

fn parse_stacks(stacks_str: &str) -> Vec<Vec<u8>> {
    let mut stacks = Vec::new();

    let rows: Vec<&str> = stacks_str.lines().collect();
    let stack_count = (rows[0].len() + 1) / 4;
    let tallest_stack = rows.len() - 1;

    for i in 0..stack_count {
        let mut stack = Vec::new();

        for j in 0..tallest_stack {
            let crate_id = rows[rows.len() - 2 - j].as_bytes()[1 + i * 4];
            if crate_id != 32 {
                stack.push(crate_id);
            }
        }

        stacks.push(stack);
    }

    stacks
}

fn parse_instructions(instructions_str: &str) -> Vec<Instruction> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut instructions = Vec::new();

    for line in instructions_str.lines() {
        let caps = re.captures(line).unwrap();

        instructions.push(Instruction {
            count: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            from: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            to: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        })
    }

    instructions
}

fn run(stacks: &mut Vec<Vec<u8>>, instructions: &Vec<Instruction>) {
    for instruction in instructions {
        for _ in 0..instruction.count {
            let crate_id = stacks[instruction.from as usize - 1].pop().unwrap();
            stacks[instruction.to as usize - 1].push(crate_id);
        }
    }
}
