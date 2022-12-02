use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

// In which I abuse enums and enumerate the entire combination of hands from one side.
fn main() {
    follow_pt_two();
}

fn follow_strategy_guide() {
    let mut my_score: u32 = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            match line {
                Ok(str) => {
                    // TODO: this is the hackiest thing ever
                    let them: OpponentShape =
                        str.chars().nth(0).unwrap().to_string().parse().unwrap();
                    let me: MyShape = str.chars().nth(2).unwrap().to_string().parse().unwrap();
                    println!("Them {:?} <-> Me: {:?}", them, me);
                    my_score += me as u32;
                    my_score += match (me, them) {
                        (MyShape::Rock, OpponentShape::Rock) => ResultScore::Draw as u32,
                        (MyShape::Rock, OpponentShape::Paper) => ResultScore::Loss as u32,
                        (MyShape::Rock, OpponentShape::Scissors) => ResultScore::Win as u32,
                        (MyShape::Paper, OpponentShape::Rock) => ResultScore::Win as u32,
                        (MyShape::Paper, OpponentShape::Paper) => ResultScore::Draw as u32,
                        (MyShape::Paper, OpponentShape::Scissors) => ResultScore::Loss as u32,
                        (MyShape::Scissors, OpponentShape::Rock) => ResultScore::Loss as u32,
                        (MyShape::Scissors, OpponentShape::Paper) => ResultScore::Win as u32,
                        (MyShape::Scissors, OpponentShape::Scissors) => ResultScore::Draw as u32,
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
                    // TODO: There's probably a better way to do things
                    let them: OpponentShape =
                        str.chars().nth(0).unwrap().to_string().parse().unwrap();
                    let desired_end: DesiredEnd =
                        str.chars().nth(2).unwrap().to_string().parse().unwrap();
                    println!("Them {:?} <-> DesiredEnd: {:?}", them, desired_end);
                    my_score += desired_end as u32;
                    my_score += match (them, desired_end) {
                        (OpponentShape::Rock, DesiredEnd::Win) => MyShape::Paper as u32,
                        (OpponentShape::Rock, DesiredEnd::Draw) => MyShape::Rock as u32,
                        (OpponentShape::Rock, DesiredEnd::Lose) => MyShape::Scissors as u32,
                        (OpponentShape::Paper, DesiredEnd::Win) => MyShape::Scissors as u32,
                        (OpponentShape::Paper, DesiredEnd::Draw) => MyShape::Paper as u32,
                        (OpponentShape::Paper, DesiredEnd::Lose) => MyShape::Rock as u32,
                        (OpponentShape::Scissors, DesiredEnd::Win) => MyShape::Rock as u32,
                        (OpponentShape::Scissors, DesiredEnd::Draw) => MyShape::Scissors as u32,
                        (OpponentShape::Scissors, DesiredEnd::Lose) => MyShape::Paper as u32,
                    }
                }
                Err(_) => todo!(),
            }
        }
    }

    println!("My final score: {}", my_score);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DesiredEnd {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

impl FromStr for DesiredEnd {
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum MyShape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for MyShape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OpponentShape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for OpponentShape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
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
