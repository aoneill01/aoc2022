#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn shape_value(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn outcome_value(&self, other: Shape) -> i32 {
        match self {
            Shape::Rock => match other {
                Shape::Rock => 3,
                Shape::Paper => 0,
                Shape::Scissors => 6,
            },
            Shape::Paper => match other {
                Shape::Rock => 6,
                Shape::Paper => 3,
                Shape::Scissors => 0,
            },
            Shape::Scissors => match other {
                Shape::Rock => 0,
                Shape::Paper => 6,
                Shape::Scissors => 3,
            },
        }
    }

    fn score(&self, other: Shape) -> i32 {
        self.outcome_value(other) + self.shape_value()
    }

    fn parse(letter: &str) -> Option<Shape> {
        match letter {
            "A" | "X" => Some(Shape::Rock),
            "B" | "Y" => Some(Shape::Paper),
            "C" | "Z" => Some(Shape::Scissors),
            _ => None,
        }
    }
}

fn main() {
    let result: i32 = include_str!("../input.txt")
        .lines()
        .map(|game| {
            let mut shapes = game.split_whitespace();
            (
                Shape::parse(shapes.next().unwrap()).unwrap(),
                Shape::parse(shapes.next().unwrap()).unwrap(),
            )
        })
        .map(|(a, b)| b.score(a))
        .sum();

    println!("{}", result);
}
