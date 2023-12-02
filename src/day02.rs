fn parse_all_games() -> Vec<(i32, Vec<Vec<(String, i32)>>)> {
    return std::fs::read_to_string("input/day02")
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let split_line = line.split(":").collect::<Vec<&str>>();
            let game_number = split_line[0].split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            let game_sets = split_line[1]
                .split(";")
                .map(|set| {
                    return set
                        .split(",")
                        .map(|colored_cubes| {
                            let split_colored_cubes =
                                colored_cubes.split(" ").collect::<Vec<&str>>();
                            return (
                                split_colored_cubes[2].to_string(),
                                split_colored_cubes[1].parse::<i32>().unwrap(),
                            );
                        })
                        .collect::<Vec<(String, i32)>>();
                })
                .collect::<Vec<Vec<(String, i32)>>>();
            return (game_number, game_sets);
        })
        .collect::<Vec<(i32, Vec<Vec<(String, i32)>>)>>();
}

fn solve_one() -> i32 {
    return parse_all_games()
        .into_iter()
        .map(|game| {
            let (game_number, game_sets) = game;
            let mut impossible_game = false;
            game_sets.into_iter().for_each(|set| {
                if impossible_game {
                    return;
                }
                for cube in set {
                    let (color, count) = cube;
                    match color.as_str() {
                        "red" => {
                            if count > 12 {
                                impossible_game = true
                            }
                        }
                        "green" => {
                            if count > 13 {
                                impossible_game = true
                            }
                        }
                        "blue" => {
                            if count > 14 {
                                impossible_game = true
                            }
                        }
                        _ => panic!("unknown color"),
                    }
                }
            });
            return if impossible_game { 0 } else { game_number };
        })
        .sum();
}

fn solve_two() -> i32 {
    return parse_all_games()
        .into_iter()
        .map(|game| {
            let (_, game_sets) = game;
            let mut heighest_red = 0;
            let mut heighest_green = 0;
            let mut heighest_blue = 0;
            game_sets.into_iter().for_each(|set| {
                for cube in set {
                    let (color, count) = cube;
                    match color.as_str() {
                        "red" => {
                            if count > heighest_red {
                                heighest_red = count;
                            }
                        }
                        "green" => {
                            if count > heighest_green {
                                heighest_green = count;
                            }
                        }
                        "blue" => {
                            if count > heighest_blue {
                                heighest_blue = count;
                            }
                        }
                        _ => panic!("unknown color"),
                    }
                }
            });
            return heighest_red * heighest_green * heighest_blue;
        })
        .sum();
}

pub fn solve() {
    println!("Part 1: {}", solve_one());
    println!("Part 2: {}", solve_two());
}
