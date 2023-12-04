use std::collections::VecDeque;
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

    let cards = parse_cards(&content);

    one(&cards);
    two(&cards);
}

fn one(cards: &Vec<Card>) {
    let score = get_score(&cards);

    println!("Part 1: {}", score);
}

fn two(cards: &Vec<Card>) {
    let mut total: u32 = 0;
    let mut queue: VecDeque<usize> = (0..cards.len()).collect();

    while let Some(card_index) = queue.pop_front() {
        total += 1;
        let card = &cards[card_index];
        let matches = get_matches(card);
        for i in 0..matches {
            let index = card_index + i as usize + 1;
            if index < cards.len() {
                queue.push_back(index);
            }
        }
    }
    println!("Part 2: {}", total);
}

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
        let matches: u32 = get_matches(&card);

        if matches > 0 {
            score += 2u32.pow(matches - 1);
        }
    }
    score
}

fn get_matches(card: &Card) -> u32 {
    card.win_nums
        .iter()
        .filter(|num| card.have_nums.contains(num))
        .count() as u32
}
