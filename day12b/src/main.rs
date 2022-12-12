use pathfinding::prelude::dijkstra;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

fn main() {
    let elevation: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let start = find_pos(&elevation, 'E').unwrap();
    let end_pos = lowest_pos(&elevation);

    let successors = move |p: &Pos| -> Vec<(Pos, i32)> {
        let mut neighbors = Vec::new();
        let curr = height(elevation[p.0][p.1]);

        if p.0 > 0 {
            if curr <= height(elevation[p.0 - 1][p.1]) + 1 {
                neighbors.push((Pos(p.0 - 1, p.1), 1));
            }
        }
        if p.0 < elevation.len() - 1 {
            if curr <= height(elevation[p.0 + 1][p.1]) + 1 {
                neighbors.push((Pos(p.0 + 1, p.1), 1));
            }
        }
        if p.1 > 0 {
            if curr <= height(elevation[p.0][p.1 - 1]) + 1 {
                neighbors.push((Pos(p.0, p.1 - 1), 1));
            }
        }
        if p.1 < elevation[p.0].len() - 1 {
            if curr <= height(elevation[p.0][p.1 + 1]) + 1 {
                neighbors.push((Pos(p.0, p.1 + 1), 1));
            }
        }

        neighbors
    };

    let result = dijkstra(&start, successors, move |p| end_pos.contains(p));
    println!("{:?}", result.unwrap().1);
}

fn find_pos(elevation: &Vec<Vec<char>>, target: char) -> Option<Pos> {
    for row in 0..elevation.len() {
        for col in 0..elevation[0].len() {
            if elevation[row][col] == target {
                return Some(Pos(row, col));
            }
        }
    }

    None
}

fn lowest_pos(elevation: &Vec<Vec<char>>) -> HashSet<Pos> {
    let mut result: HashSet<Pos> = HashSet::new();
    let lowest_height = height('a');

    for row in 0..elevation.len() {
        for col in 0..elevation[row].len() {
            if height(elevation[row][col]) == lowest_height {
                result.insert(Pos(row, col));
            }
        }
    }

    result
}

fn height(letter: char) -> u32 {
    if letter == 'S' {
        'a' as u32
    } else if letter == 'E' {
        'z' as u32
    } else {
        letter as u32
    }
}
