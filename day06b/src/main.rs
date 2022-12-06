use std::collections::HashSet;

fn main() {
    let datastream = include_str!("../input.txt").as_bytes();

    for i in 13..datastream.len() {
        let set: HashSet<&u8> = HashSet::from_iter(datastream[i - 13..=i].iter());

        if set.len() == 14 {
            println!("{}", i + 1);
            break;
        }
    }
}
