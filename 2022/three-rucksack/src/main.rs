use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    puzzle_one();
    puzzle_two();
}

fn puzzle_two() {
    let mut sum = 0;
    let mut elf_group: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            match line {
                Ok(str) => {
                    elf_group.push(str);
                    println!("vector: {:?} with count: {:?}", elf_group, elf_group.len());
                    if elf_group.len() == 3 {
                        println!("got three!");
                        let elf_one: HashSet<_> = elf_group[0].chars().collect();
                        let elf_2: HashSet<_> = elf_group[1].chars().collect();
                        let elf_3: HashSet<_> = elf_group[2].chars().collect();

                        // No need to perform a third intersection because we know
                        // that items in the first intersection, if found in the first
                        // elf's bag, match the criteria of being in all three bagz
                        for common in elf_2.intersection(&elf_3) {
                            if elf_one.contains(common) {
                                println!("Found a match in: {:?}", common);
                                // Totally hacking around "char cannot be dereferenced" for now
                                sum += get_priority(common.clone());
                            }
                        }

                        // Since we were processing a string this time but ignored it, add it to next
                        elf_group = Vec::new();
                    }
                }
                Err(_) => todo!(),
            }
        }
    }

    println!("Sum of items for puzzle two: {}", sum);
}

fn puzzle_one() {
    // split the string in half
    // find the shared item
    // Calculate its priority
    // A-Z === 65-90, a-z 97-122
    // so for A-Z value can be assigned by ascii val - 38, for a-z value can be assigned by ascii val - 96
    // sum the priorities
    let mut sum = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            match line {
                Ok(str) => {
                    // Since we know we are dealing with ascii strings this is cool
                    // otherwise we'd want to use `str.chars().count();` and then cut
                    // using  different strategy (maybe a Vector of chars())
                    let compartment_cut_point = str.len() / 2;
                    let chars = str.chars().collect::<Vec<_>>();

                    // TODO: not sure why this had to be mutable
                    let mut chars_iter = chars.chunks(compartment_cut_point);
                    let compartment_one: HashSet<_> =
                        chars_iter.next().unwrap().into_iter().collect();
                    let compartment_two: HashSet<_> =
                        chars_iter.next().unwrap().into_iter().collect();

                    let priority = compartment_one
                        .intersection(&compartment_two)
                        // still need to understand a bit better about the need to deference here
                        // https://stackoverflow.com/a/43828348
                        .fold(0, |acc, item| acc + get_priority(**item));

                    sum += priority;
                }
                Err(_) => todo!(),
            }
        }
    }

    println!("Sum of items: {}", sum);
}

fn get_priority(item: char) -> u32 {
    match item as u32 {
        val @ 65..=90 => val as u32 - 38,
        val @ 97..=122 => val - 96,
        _other => todo!(),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
