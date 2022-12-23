use std::collections::HashSet;

fn main() {
    let mut elves: HashSet<(i32, i32)> = include_str!("../input.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect();

    let mut day = 1;
    loop {
        let next = step(&elves, day - 1);
        if next == elves {
            println!("{}", day);
            return;
        }
        elves = next;
        day += 1;
    }
}

fn step(elves: &HashSet<(i32, i32)>, round: i32) -> HashSet<(i32, i32)> {
    let mut result: HashSet<(i32, i32)> = HashSet::new();

    let mut invalid: HashSet<(i32, i32)> = HashSet::new();
    let mut dest: HashSet<(i32, i32)> = HashSet::new();

    for elf in elves {
        let p = proposal(*elf, elves, round);
        if !dest.insert(p) {
            invalid.insert(p);
        }
    }

    for elf in elves {
        let p = proposal(*elf, elves, round);
        if invalid.contains(&p) {
            result.insert(*elf);
        } else {
            result.insert(p);
        }
    }

    result
}

fn proposal(elf: (i32, i32), elves: &HashSet<(i32, i32)>, round: i32) -> (i32, i32) {
    if !elves.contains(&(elf.0, elf.1 - 1))
        && !elves.contains(&(elf.0 + 1, elf.1 - 1))
        && !elves.contains(&(elf.0 + 1, elf.1))
        && !elves.contains(&(elf.0 + 1, elf.1 + 1))
        && !elves.contains(&(elf.0, elf.1 + 1))
        && !elves.contains(&(elf.0 - 1, elf.1 + 1))
        && !elves.contains(&(elf.0 - 1, elf.1))
        && !elves.contains(&(elf.0 - 1, elf.1 - 1))
    {
        return elf;
    }
    for i in 0..4 {
        match (i + round) % 4 {
            0 => {
                if !elves.contains(&(elf.0, elf.1 - 1))
                    && !elves.contains(&(elf.0 + 1, elf.1 - 1))
                    && !elves.contains(&(elf.0 - 1, elf.1 - 1))
                {
                    return (elf.0, elf.1 - 1);
                }
            }
            1 => {
                if !elves.contains(&(elf.0, elf.1 + 1))
                    && !elves.contains(&(elf.0 + 1, elf.1 + 1))
                    && !elves.contains(&(elf.0 - 1, elf.1 + 1))
                {
                    return (elf.0, elf.1 + 1);
                }
            }
            2 => {
                if !elves.contains(&(elf.0 - 1, elf.1))
                    && !elves.contains(&(elf.0 - 1, elf.1 - 1))
                    && !elves.contains(&(elf.0 - 1, elf.1 + 1))
                {
                    return (elf.0 - 1, elf.1);
                }
            }
            3 => {
                if !elves.contains(&(elf.0 + 1, elf.1))
                    && !elves.contains(&(elf.0 + 1, elf.1 - 1))
                    && !elves.contains(&(elf.0 + 1, elf.1 + 1))
                {
                    return (elf.0 + 1, elf.1);
                }
            }
            _ => panic!("should not be here!"),
        }
    }

    elf
}
