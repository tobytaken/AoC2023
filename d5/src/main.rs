use ::std::fs::File;
use ::std::io::{BufRead, BufReader};
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

    one(&seeds, &maps);
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
fn two() {}
