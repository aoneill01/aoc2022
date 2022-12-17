use std::collections::HashSet;

fn main() {
    let right = (1, 0);
    let left = (-1, 0);
    let down = (0, -1);

    let mut cave: HashSet<(i32, i32)> =
        HashSet::from_iter(vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]);
    let mut height = 0;
    let mut jet_index = 0;
    // let jets: Vec<char> = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".chars().collect();
    let jets: Vec<char> = include_str!("../input.txt").trim().chars().collect();

    /*
    drop 1818 blocks
    every 1725 blocks increases height by 2734

    579,710,143 * 2,734 = 1,584,927,530,962
    drop 1000000000000 - 1725 * 579,710,143 blocks = 3325 blocks
     */

    for step in 0..(3325) {
        let mut rock = create_rock((step % 5) as u32, height);

        loop {
            let jet = jets[jet_index];
            jet_index = (jet_index + 1) % jets.len();

            let moved = move_rock(&rock, if jet == '>' { right } else { left });
            if is_valid(&moved, &cave) {
                rock = moved;
            }

            let moved = move_rock(&rock, down);
            if is_valid(&moved, &cave) {
                rock = moved;
            } else {
                height = std::cmp::max(height, *rock.iter().map(|(_, y)| y).max().unwrap());
                cave.extend(rock.iter());
                // for h in rock.iter().map(|(_, y)| y) {
                //     if is_row(*h, &cave) && step % 5 == 3 && jet_index == 423 {
                //         println!("{} {} {} {}", step % 5, jet_index, step, height);
                //     }
                // }
                break;
            }
        }
    }

    println!("{}", height);
}

fn create_rock(rock_type: u32, height: i32) -> Vec<(i32, i32)> {
    match rock_type {
        0 => {
            vec![
                (2, height + 4),
                (3, height + 4),
                (4, height + 4),
                (5, height + 4),
            ]
        }
        1 => {
            vec![
                (3, height + 6),
                (2, height + 5),
                (3, height + 5),
                (4, height + 5),
                (3, height + 4),
            ]
        }
        2 => {
            vec![
                (4, height + 6),
                (4, height + 5),
                (2, height + 4),
                (3, height + 4),
                (4, height + 4),
            ]
        }
        3 => {
            vec![
                (2, height + 7),
                (2, height + 6),
                (2, height + 5),
                (2, height + 4),
            ]
        }
        4 => {
            vec![
                (2, height + 5),
                (3, height + 5),
                (2, height + 4),
                (3, height + 4),
            ]
        }
        _ => panic!("Unexpected rock type"),
    }
}

fn move_rock(rock: &Vec<(i32, i32)>, direction: (i32, i32)) -> Vec<(i32, i32)> {
    rock.iter()
        .map(move |(x, y)| (x + direction.0, y + direction.1))
        .collect()
}

fn is_valid(rock: &Vec<(i32, i32)>, cave: &HashSet<(i32, i32)>) -> bool {
    !rock
        .iter()
        .any(move |pos| pos.0 < 0 || pos.0 > 6 || cave.contains(pos))
}

fn is_row(height: i32, cave: &HashSet<(i32, i32)>) -> bool {
    (0..7)
        .map(move |x| (x, height))
        .all(move |pos| cave.contains(&pos))
}
