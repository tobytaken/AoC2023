use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").unwrap();
    let content = std::io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let mut games: Vec<Game> = vec![];
    for line in content {
        games.push(parse_input(&line));
    }

    one(&games);
    two(&games);
}

fn one(games: &Vec<Game>) {
    let num_of_balls: HashMap<&str, u8> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();

    let mut total: u32 = 0;

    'games: for game in games {
        for draw in &game.draws {
            if draw.red > num_of_balls["red"]
                || draw.green > num_of_balls["green"]
                || draw.blue > num_of_balls["blue"]
            {
                continue 'games;
            }
        }
        total += game.id as u32;
    }
    println!("{}", total);
}

fn parse_input(s: &str) -> Game {
    let mut game: Game = Game {
        id: 0,
        draws: vec![],
    };
    let split: Vec<&str> = s[5..].split(&[':', ';']).map(|x| x.trim()).collect();
    game.id = split[0].parse::<u8>().unwrap();

    for str_draw in &split[1..] {
        let key_value_pairs: Vec<&str> = str_draw.split(',').collect();
        let mut draw: Draw = Draw {
            red: (0),
            green: (0),
            blue: (0),
        };

        for item in key_value_pairs {
            let kv: Vec<&str> = item.split_whitespace().collect();

            match kv[1] {
                "red" => draw.red = kv[0].parse().unwrap(),
                "green" => draw.green = kv[0].parse().unwrap(),
                "blue" => draw.blue = kv[0].parse().unwrap(),
                _ => println!("Fuck"),
            }
        }
        game.draws.push(draw);
    }
    return game;
}

fn two(games: &Vec<Game>) {
    let mut total: u32 = 0;

    for game in games {
        let mut l_red = 0;
        let mut l_green = 0;
        let mut l_blue = 0;

        for draw in &game.draws {
            l_red = cmp::max(draw.red, l_red);
            l_green = cmp::max(draw.green, l_green);
            l_blue = cmp::max(draw.blue, l_blue);
        }
        total += l_red as u32 * l_green as u32 * l_blue as u32;
    }
    println!("{}", total);
}

struct Game {
    id: u8,
    draws: Vec<Draw>,
}

struct Draw {
    red: u8,
    green: u8,
    blue: u8,
}
