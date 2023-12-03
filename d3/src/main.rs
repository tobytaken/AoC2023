use std::fs::File;
use std::io::BufRead;

fn main() {
    let f = File::open("input.txt").unwrap();
    let content: Vec<String> = std::io::BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    one(&content);
}

fn one(content: &Vec<String>) {
    let mut total = 0;
    let non_symbols = ['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    for (i, line) in content.iter().enumerate() {
        let mut num_start: usize = 0;
        let mut num_end: usize = 0;
        let mut within = false;
        let mut futlapperl = false;
        for (j, c) in line.chars().enumerate() {
            if c.is_digit(10)
                && !line
                    .chars()
                    .nth(j.checked_sub(1).unwrap_or(0))
                    .unwrap_or('.')
                    .is_digit(10)
            {
                num_start = j;
                within = true;
            } else if c.is_digit(10) && !line.chars().nth(j + 1).unwrap_or('.').is_digit(10) {
                num_end = j;
                within = false;
            } else if c.is_digit(10) {
                continue;
            } else if within {
                num_end = num_start;
                within = false;
                futlapperl = true;
            }

            if !within && num_end > num_start || futlapperl {
                futlapperl = false;
                let num: u32 = line[num_start..num_end + 1].parse().unwrap();

                if !non_symbols.contains(
                    &line
                        .chars()
                        .nth(num_start.checked_sub(1).unwrap_or(0))
                        .unwrap_or('.'),
                ) || !non_symbols.contains(&line.chars().nth(num_end + 1).unwrap_or('.'))
                {
                    total += num;
                    num_end = 0;
                } else if i == 0 {
                    for index in num_start.checked_sub(1).unwrap_or(0)..num_end + 2 {
                        if !non_symbols.contains(&content[i + 1].chars().nth(index).unwrap_or('.'))
                        {
                            total += num;
                            num_end = 0;
                            break;
                        }
                    }
                } else if i == content.len() - 1 {
                    for index in num_start.checked_sub(1).unwrap_or(0)..num_end + 2 {
                        if !non_symbols.contains(&content[i - 1].chars().nth(index).unwrap_or('.'))
                        {
                            total += num;
                            num_end = 0;
                            break;
                        }
                    }
                } else {
                    for index in num_start.checked_sub(1).unwrap_or(0)..num_end + 2 {
                        if !non_symbols.contains(&content[i - 1].chars().nth(index).unwrap_or('.'))
                            || !non_symbols
                                .contains(&content[i + 1].chars().nth(index).unwrap_or('.'))
                        {
                            total += num;
                            num_end = 0;
                            break;
                        }
                    }
                }

                num_end = 0;
            }
        }
    }
    println!("{}", total);
}
