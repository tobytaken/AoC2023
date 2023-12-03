use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
fn main() {
    let f = File::open("input.txt").expect("Failed to open input.txt");
    let content: Vec<String> = std::io::BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    one(&content);
    two(&content);
}

fn one(content: &[String]) {
    let mut total = 0;

    for line in content {
        let mut first = 0;
        let mut last = 420;
        let mut flag = false;

        for c in line.chars() {
            if c.is_numeric() && !flag {
                first = c.to_digit(10).unwrap();
                flag = true;
            } else if c.is_numeric() {
                last = c.to_digit(10).unwrap();
            }
        }
        if last == 420 {
            last = first;
        }
        total += first * 10 + last;
    }

    println!("{}", total);
}

fn two(content: &[String]) {
    let mut total = 0;
    let map: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    for line in content {
        let mut digits: Vec<u32> = vec![];

        for index in 0..line.len() {
            if line.chars().nth(index).unwrap().is_digit(10) {
                digits.push(line.chars().nth(index).unwrap().to_digit(10).unwrap());
            }
            for (key, value) in &map {
                if line[index..].starts_with(key) {
                    digits.push(*value);
                }
            }
        }
        total += digits.first().unwrap() * 10 + digits.last().unwrap();
    }

    println!("{}", total);
}
