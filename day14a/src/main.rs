use std::collections::HashSet;

fn main() {
    let rocks = parse_input();
    let mut sand = HashSet::<(i32, i32)>::new();

    while drop_sand(&rocks, &mut sand) {}

    println!("{}", sand.len());
}

fn drop_sand(rocks: &HashSet<(i32, i32)>, sand: &mut HashSet<(i32, i32)>) -> bool {
    let mut loc = (500, 0);

    while loc.1 < 500 {
        let down = (loc.0, loc.1 + 1);
        let down_left = (loc.0 - 1, loc.1 + 1);
        let down_right = (loc.0 + 1, loc.1 + 1);

        if !rocks.contains(&down) && !sand.contains(&down) {
            loc = down;
            continue;
        }

        if !rocks.contains(&down_left) && !sand.contains(&down_left) {
            loc = down_left;
            continue;
        }

        if !rocks.contains(&down_right) && !sand.contains(&down_right) {
            loc = down_right;
            continue;
        }

        sand.insert(loc);
        return true;
    }

    false
}

fn parse_input() -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();

    for line in include_str!("../input.txt").lines() {
        let mut points = line.split(" -> ");

        let mut prev = parse_pair(points.next().unwrap());
        while let Some(next) = points.next() {
            let next = parse_pair(next);
            if prev.0 == next.0 {
                let mut cur = prev;
                while cur != next {
                    result.insert(cur);
                    if cur.1 < next.1 {
                        cur.1 += 1;
                    } else {
                        cur.1 -= 1;
                    }
                }
                result.insert(cur);
            } else {
                let mut cur = prev;
                while cur != next {
                    result.insert(cur);
                    if cur.0 < next.0 {
                        cur.0 += 1;
                    } else {
                        cur.0 -= 1;
                    }
                }
                result.insert(cur);
            }
            prev = next;
        }
    }

    result
}

fn parse_pair(input: &str) -> (i32, i32) {
    let mut parts = input.split(",");
    (
        parts.next().unwrap().parse::<i32>().unwrap(),
        parts.next().unwrap().parse::<i32>().unwrap(),
    )
}
