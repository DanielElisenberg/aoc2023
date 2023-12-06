fn get_one_integer(line: &str) -> i64 {
    line.chars()
        .filter(|s| s.is_numeric())
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}

fn get_all_integers(line: &str) -> Vec<i32> {
    line.split(" ")
        .filter_map(|s| match s.parse::<i32>() {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .collect::<Vec<i32>>()
}

fn solve_part_one() -> i32 {
    let file = std::fs::read_to_string("input/day06").unwrap();
    let lines = file
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    let split_time_line = get_all_integers(lines[0]);
    let split_distance_line = get_all_integers(lines[1]);
    (0..split_time_line.len())
        .map(|i| {
            let time = split_time_line[i];
            let distance = split_distance_line[i];
            (0..time)
                .map(|held_button_for| {
                    let race_time = time - held_button_for;
                    let raced_distance = held_button_for * race_time;
                    raced_distance
                })
                .filter(|raced_distance| *raced_distance > distance)
                .collect::<Vec<i32>>()
                .len() as i32
        })
        .product()
}

fn solve_part_two() -> i32 {
    let file = std::fs::read_to_string("input/day06").unwrap();
    let lines = file
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    let time = get_one_integer(lines[0]);
    let distance = get_one_integer(lines[1]);
    return (0..time)
        .map(|held_button_for| {
            let race_time = time - held_button_for;
            let raced_distance = held_button_for * race_time;
            raced_distance
        })
        .filter(|raced_distance| *raced_distance > distance)
        .collect::<Vec<i64>>()
        .len() as i32;
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
