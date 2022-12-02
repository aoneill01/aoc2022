#[derive(Debug, Clone, Copy)]
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
            "A" => Some(Shape::Rock),
            "B" => Some(Shape::Paper),
            "C" => Some(Shape::Scissors),
            _ => None,
        }
    }

    fn strategy(&self) -> (Shape, Shape, Shape) {
        match self {
            Shape::Rock => (Shape::Paper, Shape::Rock, Shape::Scissors),
            Shape::Paper => (Shape::Scissors, Shape::Paper, Shape::Rock),
            Shape::Scissors => (Shape::Rock, Shape::Scissors, Shape::Paper),
        }
    }

    fn strategy_for(&self, letter: &str) -> Option<Shape> {
        let s = self.strategy();
        match letter {
            "X" => Some(s.2),
            "Y" => Some(s.1),
            "Z" => Some(s.0),
            _ => None,
        }
    }
}

fn main() {
    let result: i32 = include_str!("../input.txt")
        .lines()
        .map(|game| {
            let mut shapes = game.split_whitespace();
            let opponent = Shape::parse(shapes.next().unwrap()).unwrap();
            (
                opponent,
                opponent.strategy_for(shapes.next().unwrap()).unwrap(),
            )
        })
        .map(|(a, b)| b.score(a))
        .sum();

    println!("{}", result);
}
