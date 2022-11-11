fn main() {
    let values = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut count: u32 = 0;

    for i in 0..values.len() - 1 {
        count += if values[i + 1] > values[i] { 1 } else { 0};
    }

    println!("{}", count);
}
