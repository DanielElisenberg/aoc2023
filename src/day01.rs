fn solve_part_one() -> i32 {
    return std::fs::read_to_string("input/day01")
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let numbers = line
                .chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_string())
                .collect::<Vec<String>>();
            return format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
                .parse::<i32>()
                .unwrap();
        })
        .sum();
}

fn solve_part_two() -> i32 {
    let number_words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    return std::fs::read_to_string("input/day01")
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut numbers = Vec::new();
            for (index, char) in line.chars().enumerate() {
                if char.is_numeric() {
                    numbers.push(char.to_string());
                }
                for (number_value, number_word) in number_words.iter().enumerate() {
                    if index + number_word.len() > line.len() {
                        continue;
                    }
                    if line[index..index + number_word.len()] == number_word.to_string() {
                        numbers.push((number_value + 1).to_string());
                    }
                }
            }
            return format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
                .parse::<i32>()
                .unwrap();
        })
        .sum();
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
