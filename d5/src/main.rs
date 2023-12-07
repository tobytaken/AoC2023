use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let f = File::open("input.txt").unwrap();
    let content: Vec<String> = BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let seeds: Vec<u64> = content[0]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut skip_next = false;
    let maps: Vec<Vec<&str>> = content.iter().fold(vec![vec![]], |mut acc, line| {
        if line.is_empty() {
            acc.push(vec![]);
            skip_next = true;
        } else if skip_next {
            skip_next = false;
        } else {
            acc.last_mut().unwrap().push(line);
        }
        acc
    });

    let maps_num: Vec<Vec<Vec<u64>>> =
        content.iter().skip(1).fold(vec![vec![]], |mut acc, line| {
            if line.is_empty() {
                acc.push(vec![]);
                skip_next = true;
            } else if skip_next {
                skip_next = false;
            } else {
                let numbers = line
                    .split_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                acc.last_mut().unwrap().push(numbers);
            }
            acc
        });

    one(&seeds, &maps);
    two(&seeds, &maps_num);
}

fn one(seeds: &[u64], maps: &Vec<Vec<&str>>) {
    let mut locations: Vec<u64> = vec![];
    for seed in seeds {
        let mut current = *seed;
        for map in &maps[1..] {
            for line in map {
                let nums: Vec<u64> = line
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();

                if current >= nums[1] && current < nums[1] + nums[2] {
                    current = current - nums[1] + nums[0];
                    break;
                }
            }
        }
        locations.push(current);
    }
    locations.sort_unstable();
    println!("{}", locations[0]);
}
fn two(seeds: &[u64], maps: &Vec<Vec<Vec<u64>>>) {
    let mut locations: Vec<u64> = vec![];

    for index in (0..seeds.len()).step_by(2) {
        let start: usize = seeds[index] as usize;
        let len: usize = seeds[index + 1] as usize;
        for seed in start..start + len {
            let mut current = seed as usize;
            for map in &maps[1..] {
                for line in map {
                    if current >= line[1] as usize && current < line[1] as usize + line[2] as usize
                    {
                        current = current - line[1] as usize + line[0] as usize;
                        break;
                    }
                }
            }
            locations.push(current as u64);
        }
    }

    let mut min: u64 = u64::MAX;

    for location in locations {
        min = std::cmp::min(min, location);
    }
    println!("{}", min);
}
