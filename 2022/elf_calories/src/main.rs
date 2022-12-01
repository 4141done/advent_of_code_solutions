use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    top_elf();
    top_three_elves();
    top_three_elves_matchy();
}

fn top_elf() {
    let mut most = 0;
    let mut current = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            match line {
                Ok(str) if str.is_empty() => {
                    if current > most {
                        most = current;
                    }
                    current = 0;
                }
                Ok(str) => {
                    let as_int: i32 = str.parse().unwrap();
                    current = current + as_int;
                }
                Err(_) => todo!(),
            }
        }
    }

    println!("Elf with most calories has: {}", most);
}

// Maybe this is more Rustacular?  Not sure.
fn top_three_elves() {
    let mut most = [0, 0, 0];
    let mut current = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            match line {
                Ok(str) if str.is_empty() => {
                    if current > most[0] {
                        most[2] = most[1];
                        most[1] = most[0];
                        most[0] = current;
                    } else if current > most[1] {
                        most[2] = most[1];
                        most[1] = current;
                    } else if current > most[2] {
                        most[2] = current;
                    }
                    current = 0;
                }
                Ok(str) => {
                    let as_int: i32 = str.parse().unwrap();
                    current = current + as_int;
                }
                Err(_) => todo!(),
            }
        }
    }
    let sum: i32 = most.iter().sum();
    println!("Sum is: {}", sum);
}

// This is probably me writing "Elixir-flavored Rust" but here we go.
// I like this one better though..
fn top_three_elves_matchy() {
    let mut most = [0, 0, 0];
    let mut current = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            match line {
                Ok(str) if str.is_empty() && current > most[0] => {
                    most[2] = most[1];
                    most[1] = most[0];
                    most[0] = current;
                    current = 0;
                }
                Ok(str) if str.is_empty() && current > most[1] => {
                    most[2] = most[1];
                    most[1] = current;
                    current = 0;
                }
                Ok(str) if str.is_empty() && current > most[2] => {
                    most[2] = current;
                    current = 0;
                }
                Ok(str) if str.is_empty() => {
                    current = 0;
                }
                Ok(str) => {
                    let as_int: i32 = str.parse().unwrap();
                    current = current + as_int;
                }
                Err(_) => todo!(),
            }
        }
    }
    let sum: i32 = most.iter().sum();
    println!("Sum is: {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
