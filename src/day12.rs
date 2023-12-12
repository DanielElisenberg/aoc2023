use std::collections::HashMap;

fn read_condition_string(condition: &str) -> Condition {
    condition
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn recursive_find(
    cache: &mut HashMap<(Vec<char>, i64, Vec<i32>), i64>,
    spring_string: Vec<char>,
    condition: Vec<i32>,
    this_group: i32,
) -> i64 {
    if condition.len() == 0 {
        return if spring_string.contains(&'#') { 0 } else { 1 };
    }
    if spring_string.len() == 0 && this_group == condition[0] && condition.len() == 1 {
        return 1;
    }
    if spring_string.len() == 0 {
        return 0;
    }
    if this_group > condition[0] {
        return 0;
    }
    let cache_key = (spring_string.clone(), this_group as i64, condition.clone());
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }
    let matches = match spring_string[0] {
        '#' => recursive_find(
            cache,
            spring_string.clone().iter().skip(1).map(|c| *c).collect(),
            condition.clone(),
            this_group + 1,
        ),
        '.' => {
            if this_group == 0 {
                recursive_find(
                    cache,
                    spring_string.clone().iter().skip(1).map(|c| *c).collect(),
                    condition.clone(),
                    this_group,
                )
            } else if this_group == condition[0] {
                recursive_find(
                    cache,
                    spring_string.clone().iter().skip(1).map(|c| *c).collect(),
                    condition.clone().iter().skip(1).map(|i| *i).collect(),
                    0,
                )
            } else {
                0
            }
        }
        '?' => {
            let dot_case = if this_group == 0 {
                recursive_find(
                    cache,
                    spring_string.clone().iter().skip(1).map(|c| *c).collect(),
                    condition.clone(),
                    this_group,
                )
            } else if this_group == condition[0] {
                recursive_find(
                    cache,
                    spring_string.clone().iter().skip(1).map(|c| *c).collect(),
                    condition.clone().iter().skip(1).map(|i| *i).collect(),
                    0,
                )
            } else {
                0
            };
            dot_case
                + recursive_find(
                    cache,
                    spring_string.iter().skip(1).map(|c| *c).collect(),
                    condition.clone(),
                    this_group + 1,
                )
        }
        _ => panic!("Unknown character"),
    };
    cache.insert(cache_key, matches);
    matches
}

type Condition = Vec<i32>;

fn parse_spring_strings(unfold: bool) -> Vec<(Vec<char>, Condition)> {
    let file = std::fs::read_to_string("input/day12").unwrap();
    file.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split_line = line.split(" ").collect::<Vec<&str>>();
            let mut condition = read_condition_string(split_line[1]);
            let mut spring_string = Vec::new();
            split_line[0].chars().for_each(|c| {
                if spring_string.len() == 0 {
                    spring_string.push(c)
                } else if !(spring_string[spring_string.len() - 1] == '.' && c == '.') {
                    spring_string.push(c);
                }
            });
            if unfold {
                let repeat = spring_string.clone();
                (0..4).for_each(|_| {
                    spring_string.push('?');
                    spring_string.extend(repeat.clone());
                });
                let condition_repeat = condition.clone();
                (0..4).for_each(|_| condition.extend(condition_repeat.clone()));
            }
            (spring_string, condition)
        })
        .collect()
}

fn solve_part_one() -> Vec<i64> {
    let mut cache = HashMap::new();
    parse_spring_strings(false)
        .into_iter()
        .map(|(spring_string, condition)| recursive_find(&mut cache, spring_string, condition, 0))
        .collect()
}

fn solve_part_two() -> Vec<i64> {
    let mut cache = HashMap::new();
    parse_spring_strings(true)
        .into_iter()
        .map(|(spring_string, condition)| recursive_find(&mut cache, spring_string, condition, 0))
        .collect()
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one().into_iter().sum::<i64>());
    println!("Part 2: {}", solve_part_two().into_iter().sum::<i64>());
}
