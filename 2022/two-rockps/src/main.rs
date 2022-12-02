use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

// In which I abuse enums and enumerate all the possible combinations of hands
// I know there are some mathematical tricks to calculating Rock Paper Scissors
// results more succinctly, but I think this approach is more understandable
// if you don't know the trick. I prefer a match over nested if/else
fn main() {
    follow_strategy_guide();
    follow_pt_two();
}

fn follow_strategy_guide() {
    let mut my_score: u32 = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            match line {
                Ok(str) => {
                    // TODO: this is the hackiest thing ever
                    let them: Shape = str.chars().nth(0).unwrap().to_string().parse().unwrap();
                    let me: Shape = str.chars().nth(2).unwrap().to_string().parse().unwrap();
                    my_score += me as u32;
                    my_score += match (me, them) {
                        (Shape::Rock, Shape::Rock) => GameResult::Draw as u32,
                        (Shape::Rock, Shape::Paper) => GameResult::Lose as u32,
                        (Shape::Rock, Shape::Scissors) => GameResult::Win as u32,
                        (Shape::Paper, Shape::Rock) => GameResult::Win as u32,
                        (Shape::Paper, Shape::Paper) => GameResult::Draw as u32,
                        (Shape::Paper, Shape::Scissors) => GameResult::Lose as u32,
                        (Shape::Scissors, Shape::Rock) => GameResult::Lose as u32,
                        (Shape::Scissors, Shape::Paper) => GameResult::Win as u32,
                        (Shape::Scissors, Shape::Scissors) => GameResult::Draw as u32,
                    }
                }
                Err(_) => todo!(),
            }
        }
    }

    println!("My final score: {}", my_score);
}

fn follow_pt_two() {
    let mut my_score: u32 = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            match line {
                Ok(str) => {
                    // TODO: this is the hackiest thing ever
                    let them: Shape = str.chars().nth(0).unwrap().to_string().parse().unwrap();
                    let desired_end: GameResult =
                        str.chars().nth(2).unwrap().to_string().parse().unwrap();
                    my_score += desired_end as u32;
                    my_score += match (them, desired_end) {
                        (Shape::Rock, GameResult::Win) => Shape::Paper as u32,
                        (Shape::Rock, GameResult::Draw) => Shape::Rock as u32,
                        (Shape::Rock, GameResult::Lose) => Shape::Scissors as u32,
                        (Shape::Paper, GameResult::Win) => Shape::Scissors as u32,
                        (Shape::Paper, GameResult::Draw) => Shape::Paper as u32,
                        (Shape::Paper, GameResult::Lose) => Shape::Rock as u32,
                        (Shape::Scissors, GameResult::Win) => Shape::Rock as u32,
                        (Shape::Scissors, GameResult::Draw) => Shape::Scissors as u32,
                        (Shape::Scissors, GameResult::Lose) => Shape::Paper as u32,
                    }
                }
                Err(_) => todo!(),GameResult
            }
        }
    }

    println!("My final score: {}", my_score);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

enum GameResult {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
