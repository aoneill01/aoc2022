use std::collections::HashSet;

fn main() {
    let trees: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let rows = trees.len();
    let cols = trees[0].len();
    let mut visible = HashSet::new();

    for row in 0..rows {
        let mut largest: Option<u32> = None;

        for col in 0..cols {
            if largest.is_none() || trees[row][col] > largest.unwrap() {
                largest = Some(trees[row][col]);
                visible.insert((row, col));
            }
        }

        let mut largest: Option<u32> = None;

        for col in (0..cols).rev() {
            if largest.is_none() || trees[row][col] > largest.unwrap() {
                largest = Some(trees[row][col]);
                visible.insert((row, col));
            }
        }
    }

    for col in 0..cols {
        let mut largest: Option<u32> = None;

        for row in 0..rows {
            if largest.is_none() || trees[row][col] > largest.unwrap() {
                largest = Some(trees[row][col]);
                visible.insert((row, col));
            }
        }

        let mut largest: Option<u32> = None;

        for row in (0..rows).rev() {
            if largest.is_none() || trees[row][col] > largest.unwrap() {
                largest = Some(trees[row][col]);
                visible.insert((row, col));
            }
        }
    }

    println!("{}", visible.len());
}
