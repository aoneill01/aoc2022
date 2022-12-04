use itertools::Itertools;
use std::ops::Range;

fn main() {
    let answer = include_str!("../input.txt")
        .lines()
        .map(parse_line)
        .filter(is_overlap)
        .count();

    println!("{}", answer);
}

fn parse_line(line: &str) -> (Range<i32>, Range<i32>) {
    let (left, right) = line.split(",").tuples().next().unwrap();
    (to_range(left), to_range(right))
}

fn to_range(input: &str) -> Range<i32> {
    let (start_str, end_str) = input.split("-").tuples().next().unwrap();
    Range {
        start: start_str.parse::<i32>().unwrap(),
        end: end_str.parse::<i32>().unwrap() + 1,
    }
}

fn is_overlap((a, b): &(Range<i32>, Range<i32>)) -> bool {
    a.start < b.end && a.end - 1 >= b.start
}
