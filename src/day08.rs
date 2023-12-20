use std::collections::HashMap;

fn greatest_common_divisor(a: i64, b: i64) -> i64 {
    let mut num1 = a;
    let mut num2 = b;
    if num1 == num2 {
        return num1;
    }
    if num2 > num1 {
        let tmp = num1;
        num1 = num2;
        num2 = tmp;
    }
    while num2 > 0 {
        let tmp = num1;
        num1 = num2;
        num2 = tmp % num2;
    }
    return num1;
}

pub fn lowest_common_multiple(numbers: Vec<i64>) -> i64 {
    let mut lowest_common = numbers[0];
    numbers.into_iter().for_each(|number| {
        lowest_common = lowest_common * (number / greatest_common_divisor(lowest_common, number));
    });
    return lowest_common;
}

fn solve_part_one() -> i32 {
    let file = std::fs::read_to_string("input/day08").unwrap();
    let split_file = file.split("\n\n").collect::<Vec<&str>>();
    let mut instructions = split_file[0].chars().collect::<Vec<char>>();
    let mut network_map = HashMap::new();
    split_file[1]
        .split("\n")
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let [key, value_string] = line.split(" = ").collect::<Vec<&str>>()[..] else {
                panic!("Could not parse line: {}", line);
            };
            let value = value_string.split(", ").collect::<Vec<&str>>();
            let left_value = &value[0][1..];
            let right_value = &value[1][..value[1].len() - 1];
            network_map.insert(key, (left_value, right_value));
        });
    let mut current_location = "AAA";
    let mut steps = 0;
    while current_location != "ZZZ" {
        current_location = match instructions[0] {
            'R' => network_map[&current_location].1,
            'L' => network_map[&current_location].0,
            _ => panic!("Invalid instruction: {}", instructions[0]),
        };
        steps += 1;
        instructions.rotate_left(1);
    }
    return steps;
}

fn solve_part_two() -> i64 {
    let file = std::fs::read_to_string("input/day08").unwrap();
    let split_file = file.split("\n\n").collect::<Vec<&str>>();
    let mut instructions = split_file[0].chars().collect::<Vec<char>>();
    let mut network_map = HashMap::new();
    split_file[1]
        .split("\n")
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let [key, value_string] = line.split(" = ").collect::<Vec<&str>>()[..] else {
                panic!("Could not parse line: {}", line);
            };
            let value = value_string.split(", ").collect::<Vec<&str>>();
            let left_value = &value[0][1..];
            let right_value = &value[1][..value[1].len() - 1];
            network_map.insert(key, (left_value, right_value));
        });
    let mut current_locations = network_map
        .keys()
        .filter(|&&key| key.chars().collect::<Vec<char>>()[2] == 'A')
        .map(|&key| key)
        .collect::<Vec<&str>>();
    let mut steps_to_z = Vec::from([0, 0, 0, 0, 0, 0]);
    let mut steps = 0;
    while !steps_to_z.iter().all(|x| *x > 0) {
        current_locations = current_locations
            .iter()
            .enumerate()
            .map(|(index, location)| {
                if steps_to_z[index] > 0 {
                    return *location;
                };
                let next_location = match instructions[0] {
                    'R' => network_map[location].1,
                    'L' => network_map[location].0,
                    _ => panic!("Invalid instruction: {}", instructions[0]),
                };
                if next_location.chars().collect::<Vec<char>>()[2] == 'Z' {
                    steps_to_z[index] = steps + 1;
                }
                return next_location;
            })
            .collect::<Vec<&str>>();
        steps += 1;
        instructions.rotate_left(1);
    }
    return lowest_common_multiple(steps_to_z);
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
