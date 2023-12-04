use std::fs::File;
use std::io::BufRead;
use std::usize;

fn main() {
    let f = File::open("input.txt").unwrap();
    let content: Vec<String> = std::io::BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    one(&content);
    two(&content);
}

fn one(content: &[String]) {
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

fn two(content: &[String]) {
    let mut total = 0;
    let stars = parse_stars(&content);
    let nums = parse_nums(&content);

    for star in stars {
        match get_nums_from_star(&star, &nums) {
            Some(stars) => total += stars.0 * stars.1,
            None => continue,
        }
    }
    println!("{}", total);
}

fn parse_nums(str: &[String]) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];
    let mut num_start: usize = 0;
    let mut num_end: usize = 0;
    let mut within = false;
    let mut done = false;

    for (l, line) in str.iter().enumerate() {
        if within && !done {
            num_end = line.len();
            let value: u32 = str.iter().nth(l - 1).unwrap()[num_start..num_end]
                .parse()
                .unwrap();
            numbers.push(Number {
                value: (value),
                l: (l - 1),
                c: (num_start),
                len: (num_end - num_start),
            })
        }
        num_start = 0;
        num_end = 0;
        within = false;
        done = false;

        for (c, char) in line.chars().enumerate() {
            if !within && char.is_digit(10) && !done {
                num_start = c;
                within = true;
                done = false;
            } else if within && !char.is_digit(10) && !done {
                num_end = c;
                within = false;
                done = true;
            }

            if !within && done && !char.is_digit(10) {
                let value: u32 = line[num_start..num_end].parse().unwrap();
                done = false;

                numbers.push(Number {
                    value: (value),
                    l: (l),
                    c: (num_start),
                    len: (num_end - num_start),
                })
            }
        }
    }
    numbers
}

fn parse_stars(str: &[String]) -> Vec<(usize, usize)> {
    let mut stars: Vec<(usize, usize)> = vec![];

    for (l, line) in str.iter().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char == '*' {
                stars.push((l, c));
            }
        }
    }
    stars
}

fn get_nums_from_star((l, c): &(usize, usize), nums: &Vec<Number>) -> Option<(u32, u32)> {
    let mut found: Vec<u32> = vec![];
    for num in nums {
        if num.l != l - 1 && num.l != l + 1 && num.l != *l {
            continue;
        }

        if (num.c.saturating_sub(1)..=num.c + num.len).contains(c) {
            found.push(num.value);
        }
    }

    if found.len() == 2 {
        return Some((found[0], found[1]));
    }
    if found.len() > 2 {
        println!("wtf");
    }
    None
}

struct Number {
    value: u32,
    l: usize,
    c: usize,
    len: usize,
}
