use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    cards: String,
    score: i32,
    jokers_are_wild: bool,
}

fn get_card_value(card: &char, jokers_are_wild: bool) -> i32 {
    return match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if jokers_are_wild {
                1
            } else {
                11
            }
        }
        'T' => 10,
        _ => card.to_digit(10).unwrap() as i32,
    };
}

fn get_card_occurences(cards: &str) -> HashMap<char, i32> {
    cards
        .chars()
        .into_iter()
        .fold(HashMap::new(), |mut acc, card| {
            *acc.entry(card).or_insert(0) += 1;
            return acc;
        })
}

fn hand_type_score(hand: &Hand) -> i32 {
    let mut card_occurences = get_card_occurences(&hand.cards);
    if hand.jokers_are_wild
        && card_occurences
            .keys()
            .collect::<Vec<&char>>()
            .contains(&&'J')
    {
        if card_occurences[&'J'] == 5 {
            return 7;
        } else {
            let max_key = card_occurences
                .iter()
                .filter(|&x| *x.0 != 'J')
                .max_by_key(|x| x.1)
                .unwrap();
            card_occurences = get_card_occurences(&hand.cards.replace('J', &max_key.0.to_string()));
        }
    }
    match card_occurences.values().collect::<Vec<&i32>>() {
        v if v.contains(&&5) => 7,
        v if v.contains(&&4) => 6,
        v if v.contains(&&3) && v.contains(&&2) => 5,
        v if v.contains(&&3) => 4,
        v if v
            .clone()
            .into_iter()
            .filter(|&&x| x == 2)
            .collect::<Vec<&i32>>()
            .len()
            == 2 =>
        {
            3
        }
        v if v.contains(&&2) => 2,
        v if v.len() == 5 => 1,
        _ => panic!("{}", format!("invalid hand: {:?}", hand.cards)),
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_cards = self.cards.chars().collect::<Vec<char>>();
        let other_cards = other.cards.chars().collect::<Vec<char>>();

        if hand_type_score(&self) == hand_type_score(&other) {
            for i in 0..self.cards.len() {
                if self_cards[i] != other_cards[i] {
                    return get_card_value(&self_cards[i], self.jokers_are_wild)
                        .cmp(&get_card_value(&other_cards[i], other.jokers_are_wild));
                }
            }
        }
        return hand_type_score(&self).cmp(&hand_type_score(&other));
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        return hand_type_score(&self) == hand_type_score(&other);
    }
}

impl Eq for Hand {}

fn parse_hands(jokers_are_wild: bool) -> Vec<Hand> {
    let file = std::fs::read_to_string("input/day07").unwrap();
    let mut hands = file
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let [cards_str, score_str] = line.split(" ").collect::<Vec<&str>>()[..] else {
                panic!();
            };
            return Hand {
                cards: cards_str.to_string(),
                score: score_str.parse::<i32>().unwrap(),
                jokers_are_wild,
            };
        })
        .collect::<Vec<Hand>>();
    hands.sort_by(|a, b| a.cmp(&b));
    hands
}

fn solve_part_one() -> i32 {
    let hands = parse_hands(false);
    let hand_scores = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            return (index as i32 + 1) * hand.score;
        })
        .sum();
    return hand_scores;
}

pub fn solve_part_two() -> i32 {
    let hands = parse_hands(true);
    let hand_scores = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            return (index as i32 + 1) * hand.score;
        })
        .sum();
    return hand_scores;
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
