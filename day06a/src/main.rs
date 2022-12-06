fn main() {
    let datastream = include_str!("../input.txt").as_bytes();

    for i in 3..datastream.len() {
        let a = datastream[i - 3];
        let b = datastream[i - 2];
        let c = datastream[i - 1];
        let d = datastream[i];
        if a != b && a != c && a != d && b != c && b != d && c != d {
            println!("{}", i + 1);
            break;
        }
    }
}
