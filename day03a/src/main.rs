fn main() {
    let result: i32 = include_str!("../input.txt")
        .lines()
        .map(extra_item_type)
        .sum();

    println!("{}", result);
}

fn extra_item_type(line: &str) -> i32 {
    let mut seen: [bool; 52] = [false; 52];
    let (compartment_1, compartment_2) = line.split_at(line.len() / 2);
    let (compartment_1, compartment_2) = (compartment_1.as_bytes(), compartment_2.as_bytes());

    for item in compartment_1 {
        seen[priority(*item) as usize - 1] = true;
    }

    let mut duplicate = None;
    for item in compartment_2 {
        let p = priority(*item);
        if seen[p as usize - 1] {
            duplicate = Some(p);
            break;
        }
    }

    duplicate.unwrap() as i32
}

fn priority(item: u8) -> u8 {
    if item >= 97 {
        item - 96
    } else {
        item - 65 + 27
    }
}
