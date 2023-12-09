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
            loop {
                let differences = next_line
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<i32>>();
                next_line = differences.clone();
                prediction = prediction + next_line[next_line.len() - 1];
                if differences.into_iter().all(|d| d == 0) {
                    break;
                }
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
            loop {
                let differences = next_line
                    .windows(2)
                    .map(|w| w[1] - w[0])
                    .collect::<Vec<i32>>();
                next_line = differences.clone();
                history.push(next_line[0]);
                if differences.into_iter().all(|d| d == 0) {
                    break;
                }
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
