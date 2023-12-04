use std::collections::HashMap;

fn recursive_scratch(card_number: i32, scratch_cards: &HashMap<i32, Vec<i32>>) -> i32 {
    if !scratch_cards.contains_key(&card_number) {
        panic!("card not found in scratch cards {}", card_number);
    }
    return 1 + scratch_cards[&card_number]
        .clone()
        .into_iter()
        .map(|number| recursive_scratch(number, &scratch_cards))
        .sum::<i32>();
}

fn solve_part_two() -> i32 {
    let mut scratch_cards = HashMap::new();
    let my_cards = std::fs::read_to_string("input/day04")
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split_line = line.split(":").collect::<Vec<&str>>();
            let card_number = split_line[0]
                .split(" ")
                .filter(|part| !part.is_empty())
                .collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            let winning_numbers = split_line[1].split("|").collect::<Vec<&str>>()[0]
                .split(" ")
                .filter(|number| !number.is_empty())
                .collect::<Vec<&str>>();
            let my_matches = split_line[1].split("|").collect::<Vec<&str>>()[1]
                .split(" ")
                .filter(|number| !number.is_empty() && winning_numbers.contains(number))
                .collect::<Vec<&str>>()
                .len();
            scratch_cards.insert(
                card_number,
                (card_number + 1..card_number + my_matches as i32 + 1).collect::<Vec<i32>>(),
            );
            return card_number;
        })
        .collect::<Vec<i32>>();

    return my_cards
        .into_iter()
        .map(|card_number| recursive_scratch(card_number, &scratch_cards))
        .sum();
}

fn solve_part_one() -> i32 {
    return std::fs::read_to_string("input/day04")
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let card_info = line.split(":").collect::<Vec<&str>>()[1];
            let winning_numbers = card_info.split("|").collect::<Vec<&str>>()[0]
                .split(" ")
                .filter(|number| !number.is_empty())
                .collect::<Vec<&str>>();
            println!("{:?}", winning_numbers);
            let mut my_score = 0;
            card_info.split("|").collect::<Vec<&str>>()[1]
                .split(" ")
                .filter(|number| !number.is_empty() && winning_numbers.contains(number))
                .for_each(|_| {
                    if my_score == 0 {
                        my_score += 1
                    } else {
                        my_score *= 2
                    }
                });
            return my_score;
        })
        .sum();
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
