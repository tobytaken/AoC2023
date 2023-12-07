use std::fs::File;
use std::io::{BufRead, BufReader};

struct Race {
    time: u64,
    distance: u64,
}
fn main() {
    let f = File::open("input.txt").unwrap();
    let content: Vec<String> = BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let races: Vec<Race> = parse_races(&content);
    let single_race: Race = parse_single_race(&content);

    one(&races);
    two(&single_race);
}
fn parse_races(content: &[String]) -> Vec<Race> {
    let mut races: Vec<Race> = vec![];
    let times: Vec<u64> = content[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<u64> = content[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    for (t, time) in times.iter().enumerate() {
        races.push(Race {
            time: (*time),
            distance: (distances[t]),
        });
    }
    races
}

fn parse_single_race(content: &[String]) -> Race {
    let mut total_time = String::new();
    let mut total_distance = String::new();

    for time in content[0].split_whitespace().skip(1).collect::<Vec<&str>>() {
        total_time += time;
    }

    for distance in content[1].split_whitespace().skip(1).collect::<Vec<&str>>() {
        total_distance += distance;
    }

    Race {
        time: total_time.parse().unwrap(),
        distance: total_distance.parse().unwrap(),
    }
}
fn one(races: &[Race]) {
    let mut total: u32 = 1;
    for race in races {
        let mut ways_to_win: u32 = 0;
        for hold_time in 0..race.time {
            let run_time = race.time - hold_time;
            let distance = run_time * hold_time;
            if distance > race.distance {
                ways_to_win += 1;
            }
        }
        total *= ways_to_win;
    }

    println!("{}", total);
}
fn two(race: &Race) {
    let mut ways_to_win: u64 = 0;
    for hold_time in 0..race.time {
        let run_time = race.time - hold_time;
        let distance = run_time * hold_time;
        if distance > race.distance {
            ways_to_win += 1;
        }
    }

    println!("{}", ways_to_win);
}
