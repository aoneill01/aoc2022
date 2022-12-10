use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let instructions = include_str!("../input.txt").lines().map(parse_line);

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::new();

    for (dir, dist) in instructions {
        for _ in 0..dist {
            match dir {
                "U" => head.1 -= 1,
                "D" => head.1 += 1,
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                _ => {}
            }

            tail = update_tail(tail, head);
            visited.insert(tail);
        }
    }

    println!("{}", visited.len());
}

fn parse_line(line: &str) -> (&str, i32) {
    let (dir, dist) = line.split(" ").tuples::<(&str, &str)>().next().unwrap();
    (dir, dist.parse::<i32>().unwrap())
}

fn update_tail(tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    let mut updated = tail.clone();
    let delta_x = tail.0 - head.0;
    let delta_y = tail.1 - head.1;

    if delta_x < -1 || delta_x > 1 || delta_y < -1 || delta_y > 1 {
        if delta_y <= -1 {
            updated.1 += 1;
        }
        if delta_y >= 1 {
            updated.1 -= 1;
        }
        if delta_x <= -1 {
            updated.0 += 1;
        }
        if delta_x >= 1 {
            updated.0 -= 1;
        }
    }

    updated
}
