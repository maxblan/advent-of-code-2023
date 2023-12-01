use std::fs::File;
use std::io::{ self, BufRead };
use std::path::Path;

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> i32 {
    let mut sum = 0;

    let mut numbers_found: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(ip) = line {
                for character in ip.chars() {
                    if let Some(digit) = character.to_digit(10) {
                        numbers_found.push(digit);
                    }
                }

                if let (Some(first), Some(last)) = (numbers_found.first(), numbers_found.last()) {
                    let number: i32 = format!("{}{}", first, last).parse().unwrap();
                    sum += number;
                }
                numbers_found.clear();
            }
        }
    }
    return sum;
}

fn part_two() -> i32 {
    let mut sum = 0;

    let number_words = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ];

    let mut numbers_found: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(ip) = line {
                for (index, character) in ip.chars().enumerate() {
                    if let Some(digit) = character.to_digit(10) {
                        numbers_found.push(digit);
                    } else {
                        for (word, value) in &number_words {
                            if ip[index..].starts_with(word) {
                                numbers_found.push(*value);
                                break;
                            }
                        }
                    }
                }

                if let (Some(first), Some(last)) = (numbers_found.first(), numbers_found.last()) {
                    let number: i32 = format!("{}{}", first, last).parse().unwrap();
                    sum += number;
                }
                numbers_found.clear();
            }
        }
    }
    return sum;
}

/// This function has been copied from [here](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach).
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
