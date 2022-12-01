fn main() {
    let mut elf_calories: Vec<i32> = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| s.lines().map(|cal| cal.parse::<i32>().unwrap()).sum())
        .collect();

    elf_calories.sort();
    elf_calories.reverse();

    println!("{}", elf_calories.iter().take(3).sum::<i32>());
}
