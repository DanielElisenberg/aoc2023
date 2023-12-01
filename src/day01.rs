fn solve_part_one() {
    let file = std::fs::read_to_string("input/day01").unwrap();
    let lines = file.split("\n");
    let calibration_sum: i32 = lines
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

    println!("Part 1: {}", calibration_sum);
}

fn solve_part_two() {
    let file = std::fs::read_to_string("input/day01").unwrap();
    let lines = file.split("\n");
    let number_words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let calibration_sum: i32 = lines
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

    println!("Part 2: {}", calibration_sum);
}

pub fn solve() {
    solve_part_one();
    solve_part_two();
}
