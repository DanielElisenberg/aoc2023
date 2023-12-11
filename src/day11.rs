fn parse_star_map() -> Vec<(i64, i64)> {
    let file = std::fs::read_to_string("input/day11").unwrap();
    file.split("\n")
        .filter(|s| !s.is_empty())
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(column, char)| {
                    if char == '#' {
                        Some((row as i64, column as i64))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(i64, i64)>>()
        })
        .collect::<Vec<(i64, i64)>>()
}

fn get_max_y(star_map: &Vec<(i64, i64)>) -> i64 {
    star_map.iter().map(|(y, _)| *y).max().unwrap()
}

fn get_max_x(star_map: &Vec<(i64, i64)>) -> i64 {
    star_map.iter().map(|(_, x)| *x).max().unwrap()
}

fn get_manhattan_distance(star1: &(i64, i64), star2: &(i64, i64)) -> i64 {
    return (star1.0 - star2.0).abs() + (star1.1 - star2.1).abs();
}

fn expand_universe(star_map: &Vec<(i64, i64)>, expand_by: i64) -> Vec<(i64, i64)> {
    let mut expanded_star_map = star_map.clone();
    let mut y = 0;
    while y < get_max_y(&expanded_star_map) {
        if expanded_star_map.iter().all(|star| star.0 != y) {
            expanded_star_map = expanded_star_map
                .iter()
                .map(|star| {
                    if star.0 > y {
                        (star.0 + expand_by, star.1)
                    } else {
                        *star
                    }
                })
                .collect();
            y += expand_by
        }
        y += 1;
    }
    let mut x = 0;
    while x < get_max_x(&expanded_star_map) {
        if expanded_star_map.iter().all(|star| star.1 != x) {
            expanded_star_map = expanded_star_map
                .iter()
                .map(|star| {
                    if star.1 > x {
                        (star.0, star.1 + expand_by)
                    } else {
                        *star
                    }
                })
                .collect();
            x += expand_by
        }
        x += 1;
    }
    return expanded_star_map;
}
fn solve_part_one() -> i64 {
    let star_map = parse_star_map();
    let expanded_star_map = expand_universe(&star_map, 1);
    let mut combined_distances = 0;
    (0..expanded_star_map.len()).for_each(|i| {
        (i + 1..expanded_star_map.len()).for_each(|j| {
            combined_distances +=
                get_manhattan_distance(&expanded_star_map[i], &expanded_star_map[j]);
        });
    });
    return combined_distances;
}

fn solve_part_two() -> i64 {
    let star_map = parse_star_map();
    let expanded_star_map = expand_universe(&star_map, 999999);
    let mut combined_distances = 0;
    (0..expanded_star_map.len()).for_each(|i| {
        (i + 1..expanded_star_map.len()).for_each(|j| {
            combined_distances +=
                get_manhattan_distance(&expanded_star_map[i], &expanded_star_map[j]);
        });
    });
    return combined_distances;
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
