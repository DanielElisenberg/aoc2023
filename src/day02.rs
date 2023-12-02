enum Color {
    Red,
    Green,
    Blue,
}

struct Cube {
    color: Color,
    count: i32,
}

struct Set {
    cubes: Vec<Cube>,
}

struct Game {
    number: i32,
    sets: Vec<Set>,
}

fn parse_set(set_string: &str) -> Set {
    return Set {
        cubes: set_string
            .split(",")
            .map(|colored_cubes| {
                let split_colored_cubes = colored_cubes.split(" ").collect::<Vec<&str>>();
                return Cube {
                    color: match split_colored_cubes[2] {
                        "red" => Color::Red,
                        "green" => Color::Green,
                        "blue" => Color::Blue,
                        _ => panic!("unknown color {}", split_colored_cubes[2]),
                    },
                    count: split_colored_cubes[1].parse::<i32>().unwrap(),
                };
            })
            .collect::<Vec<Cube>>(),
    };
}

fn parse_all_games() -> Vec<Game> {
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
                .map(|set| parse_set(set))
                .collect::<Vec<Set>>();
            return Game {
                number: game_number,
                sets: game_sets,
            };
        })
        .collect::<Vec<Game>>();
}

fn solve_one() -> i32 {
    return parse_all_games()
        .into_iter()
        .map(|game| {
            let impossible_game = game.sets.into_iter().any(|set| {
                for cube in set.cubes {
                    match cube.color {
                        Color::Red => {
                            if cube.count > 12 {
                                return true;
                            }
                        }
                        Color::Green => {
                            if cube.count > 13 {
                                return true;
                            }
                        }
                        Color::Blue => {
                            if cube.count > 14 {
                                return true;
                            }
                        }
                    }
                }
                return false;
            });
            return if impossible_game { 0 } else { game.number };
        })
        .sum();
}

fn solve_two() -> i32 {
    return parse_all_games()
        .into_iter()
        .map(|game| {
            let mut heighest_red = 0;
            let mut heighest_green = 0;
            let mut heighest_blue = 0;
            game.sets.into_iter().for_each(|set| {
                for cube in set.cubes {
                    match cube.color {
                        Color::Red => {
                            if cube.count > heighest_red {
                                heighest_red = cube.count;
                            }
                        }
                        Color::Green => {
                            if cube.count > heighest_green {
                                heighest_green = cube.count;
                            }
                        }
                        Color::Blue => {
                            if cube.count > heighest_blue {
                                heighest_blue = cube.count;
                            }
                        }
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
