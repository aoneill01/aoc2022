use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let directions = vec![
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];

    let scans: HashSet<(i32, i32, i32)> =
        HashSet::from_iter(include_str!("../input.txt").lines().map(|line| {
            line.split(",")
                .tuples()
                .map(|(x, y, z)| (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()))
                .next()
                .unwrap()
        }));

    let mut surface_area = 0;
    for scan in &scans {
        for adjacent in directions
            .iter()
            .map(|(d_x, d_y, d_z)| (scan.0 + d_x, scan.1 + d_y, scan.2 + d_z))
        {
            if !scans.contains(&adjacent) {
                surface_area += 1;
            }
        }
    }
    println!("{}", surface_area);
}
