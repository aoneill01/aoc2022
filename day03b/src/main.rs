use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let mut count = 0;

    for (a, b, c) in include_str!("../input.txt").lines().tuples() {
        count += common_item_type(a, b, c).unwrap();
    }

    println!("{}", count);
}

fn common_item_type(a: &str, b: &str, c: &str) -> Option<i32> {
    let mut counts: [i32; 52] = [0; 52];

    for pack in vec![a, b, c] {
        let unique_priorities: HashSet<u8> =
            HashSet::from_iter(pack.as_bytes().iter().map(|item| priority(*item)));

        for p in unique_priorities {
            counts[p as usize - 1] += 1;

            if counts[p as usize - 1] == 3 {
                return Some(p.into());
            }
        }
    }

    None
}

fn priority(item: u8) -> u8 {
    if item >= 97 {
        item - 96
    } else {
        item - 65 + 27
    }
}
