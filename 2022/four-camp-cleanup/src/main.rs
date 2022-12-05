use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
    puzzle_one();
    puzzle_two();
}

#[derive(Debug)]
struct CleanupArea {
    start: u32,
    end: u32,
}

impl CleanupArea {
    fn completely_overlaps(area_one: CleanupArea, area_two: CleanupArea) -> bool {
        println!("Processing: {:?} and {:?}", area_one, area_two);
        ((area_one.start <= area_two.start) && (area_one.end >= area_two.end))
            || ((area_two.start <= area_one.start) && (area_two.end >= area_one.end))
    }

    fn any_overlap(area_one: CleanupArea, area_two: CleanupArea) -> bool {
        let area_one_range = area_one.start..=area_one.end;
        let area_two_range = area_two.start..=area_two.end;

        area_one_range.contains(&area_two.start)
            || area_one_range.contains(&area_two.start)
            || area_two_range.contains(&area_one.start)
            || area_two_range.contains(&area_one.end)
    }
}

impl FromStr for CleanupArea {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split("-").collect::<Vec<&str>>()[..] {
            [begin_raw, end_raw] => {
                let start = match begin_raw.parse::<u32>() {
                    Ok(parsed_val) => parsed_val,
                    Err(_) => todo!(),
                };

                let end = match end_raw.parse::<u32>() {
                    Ok(parsed_val) => parsed_val,
                    Err(_) => todo!(),
                };

                Ok(CleanupArea { start, end })
            }
            _ => todo!(),
        }
    }
}

fn puzzle_one() {
    let mut overlapping_assignment_count = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            match line {
                Ok(str) => match str.split(",").collect::<Vec<&str>>()[..] {
                    [first_area_raw, second_area_raw] => {
                        let first_area = CleanupArea::from_str(first_area_raw).unwrap();
                        let second_area = CleanupArea::from_str(second_area_raw).unwrap();

                        if CleanupArea::completely_overlaps(first_area, second_area) {
                            println!("COMPLETE OVERLAP FOUND");
                            overlapping_assignment_count += 1;
                        }
                    }
                    _ => todo!(),
                },
                Err(_) => todo!(),
            }
        }
    }
    println!(
        "Number of completely overlapping assignments is: {}",
        overlapping_assignment_count
    );
}

fn puzzle_two() {
    let mut any_overlapping_count = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            match line {
                Ok(str) => match str.split(",").collect::<Vec<&str>>()[..] {
                    [first_area_raw, second_area_raw] => {
                        let first_area = CleanupArea::from_str(first_area_raw).unwrap();
                        let second_area = CleanupArea::from_str(second_area_raw).unwrap();

                        if CleanupArea::any_overlap(first_area, second_area) {
                            println!("ANY OVERLAP FOUND");
                            any_overlapping_count += 1;
                        }
                    }
                    _ => todo!(),
                },
                Err(_) => todo!(),
            }
        }
    }

    println!(
        "Number of overlapping assignments is: {}",
        any_overlapping_count
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
