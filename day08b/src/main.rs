fn main() {
    let trees: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let rows = trees.len();
    let cols = trees[0].len();

    let mut best_score: Option<u32> = None;

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            let score = scenic_score(&trees, row, col, rows, cols);
            if best_score.is_none() || score > best_score.unwrap() {
                best_score = Some(score);
            }
        }
    }

    println!("{}", best_score.unwrap());
}

fn scenic_score(trees: &Vec<Vec<u32>>, row: usize, col: usize, rows: usize, cols: usize) -> u32 {
    let height = trees[row][col];

    let mut dist_1: u32 = 0;
    for r in (row + 1)..rows {
        dist_1 += 1;
        if trees[r][col] >= height {
            break;
        }
    }

    let mut dist_2: u32 = 0;
    for r in (0..row).rev() {
        dist_2 += 1;
        if trees[r][col] >= height {
            break;
        }
    }

    let mut dist_3: u32 = 0;
    for c in (col + 1)..cols {
        dist_3 += 1;
        if trees[row][c] >= height {
            break;
        }
    }

    let mut dist_4: u32 = 0;
    for c in (0..col).rev() {
        dist_4 += 1;
        if trees[row][c] >= height {
            break;
        }
    }

    dist_1 * dist_2 * dist_3 * dist_4
}
