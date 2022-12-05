use std::f32::consts::E;

enum Shape {
    Rock,
    Paper,
    Scissors
}

enum GameResult {
    Win,
    Lose,
    Draw
}

impl Shape {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Shape::Rock),
            'X' => Some(Shape::Rock),
            'B' => Some(Shape::Paper),
            'Y' => Some(Shape::Paper),
            'C' => Some(Shape::Scissors),
            'Z' => Some(Shape::Scissors),
            _ => None
        }
    }

    fn score(&self) -> u64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }

    fn play(&self, other: &Self) -> GameResult {
        match self {
            Shape::Rock => match other {
                Shape::Rock => GameResult::Draw,
                Shape::Paper => GameResult::Lose,
                Shape::Scissors => GameResult::Win,
            }
            Shape::Paper => match other {
                Shape::Rock => GameResult::Win,
                Shape::Paper => GameResult::Draw,
                Shape::Scissors => GameResult::Lose,
            }
            Shape::Scissors => match other {
                Shape::Rock => GameResult::Lose,
                Shape::Paper => GameResult::Win,
                Shape::Scissors => GameResult::Draw,
            }
        }
    }
}

impl GameResult {
    fn score(&self) -> u64 {
        match self {
            GameResult::Win => 6,
            GameResult::Lose => 0,
            GameResult::Draw => 3,
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("data/02.txt")?;
    let mut sum: u64 = 0;
    for line in file.lines() {
        let v: Vec<char> = line.chars().collect();
        let other = Shape::from_char(v[0]).unwrap();
        let me = Shape::from_char(v[2]).unwrap();

        sum += me.score() + me.play(&other).score();
    }
    println!("{}", sum);
    Ok(())
}