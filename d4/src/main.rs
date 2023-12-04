use std::fs::File;
use std::io::{BufRead, BufReader};

struct Card {
    win_nums: Vec<u32>,
    have_nums: Vec<u32>,
}
fn main() {
    let f = File::open("input.txt").unwrap();
    let content: Vec<String> = BufReader::new(f)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    one(&content);
    two(&content);
}

fn one(content: &[String]) {
    let cards = parse_cards(&content);
    let score = get_score(&cards);

    println!("Score: {}", score);
}
fn two(content: &[String]) {}

fn parse_cards(content: &[String]) -> Vec<Card> {
    let mut cards: Vec<Card> = vec![];

    for line in content {
        let a: &str = &line[line.find(':').unwrap() + 1..].trim();
        let both: Vec<&str> = a.split('|').collect();

        let win_nums: Vec<u32> = both[0]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let have_nums: Vec<u32> = both[1]
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        cards.push(Card {
            win_nums: (win_nums),
            have_nums: (have_nums),
        })
    }
    cards
}

fn get_score(cards: &Vec<Card>) -> u32 {
    let mut score = 0;
    for card in cards {
        let mut matches: u32 = 0;
        for num in &card.win_nums {
            if card.have_nums.contains(num) {
                matches += 1;
            }
        }
        if matches > 0 {
            score += 2u32.pow(matches - 1);
        }
    }
    score
}
