fn parse_input() -> Vec<Vec<i32>> {
    let file = std::fs::read_to_string("input/day09").unwrap();
    file.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(" ")
                .filter(|s| !s.is_empty())
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn solve_part_one() -> i32 {
    return parse_input()
        .into_iter()
        .map(|line| {
            let mut next_line = line.clone();
            let mut prediction = next_line[next_line.len() - 1];
            while !next_line.clone().into_iter().all(|d| d == 0) {
                next_line = next_line
                    .clone()
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<i32>>();
                prediction = prediction + next_line[next_line.len() - 1];
            }
            prediction
        })
        .sum();
}

fn solve_part_two() -> i32 {
    return parse_input()
        .into_iter()
        .map(|line| {
            let mut next_line = line.clone();
            let mut history = Vec::from([next_line[0]]);
            while !next_line.clone().into_iter().all(|d| d == 0) {
                next_line = next_line
                    .clone()
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<i32>>();
                history.push(next_line[0]);
            }
            let mut prediction = 0;
            (0..(history.len() - 1))
                .rev()
                .for_each(|i| prediction = history[i] - prediction);
            prediction
        })
        .sum();
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
