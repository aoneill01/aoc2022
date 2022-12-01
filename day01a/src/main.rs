fn main() {
    let result = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| s.lines().map(|cal| cal.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap_or(-1);

    println!("{}", result);
}
