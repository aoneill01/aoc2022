fn main() {
    let total: i64 = include_str!("../input.txt").lines().map(to_number).sum();
    println!("{}", to_snafu(total));
}

fn to_number(snafu: &str) -> i64 {
    let mut result: i64 = 0;
    let mut pow: i64 = 1;

    for digit in snafu.chars().rev() {
        match digit {
            '1' => result += 1 as i64 * pow,
            '2' => result += 2 as i64 * pow,
            '-' => result -= 1 as i64 * pow,
            '=' => result -= 2 as i64 * pow,
            _ => {}
        }
        pow *= 5;
    }

    result
}

fn to_snafu(number: i64) -> String {
    let mut result = String::new();
    let mut remainder = number;

    while remainder > 0 {
        let digit = remainder % 5;
        remainder /= 5;

        match digit {
            0 => result.insert(0, '0'),
            1 => result.insert(0, '1'),
            2 => result.insert(0, '2'),
            3 => {
                result.insert(0, '=');
                remainder += 1;
            }
            4 => {
                result.insert(0, '-');
                remainder += 1;
            }
            _ => {}
        }
    }

    result
}
