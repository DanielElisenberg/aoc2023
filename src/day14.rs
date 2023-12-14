use std::collections::HashMap;

type Point = (i64, i64);

#[derive(Clone, Copy, PartialEq)]
enum RockType {
    Rolling,
    Stable,
}

struct Platform {
    rocks: HashMap<Point, RockType>,
    bounds: Point,
}

impl Platform {
    fn from_file(file_name: &str) -> Platform {
        let mut rocks = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;
        std::fs::read_to_string(file_name)
            .unwrap()
            .lines()
            .filter(|s| !s.is_empty())
            .enumerate()
            .for_each(|(y, line)| {
                max_y = if y > max_y { y } else { max_y };
                line.chars().enumerate().for_each(|(x, c)| {
                    max_x = if x > max_x { x } else { max_x };
                    match c {
                        '#' => rocks.insert((x as i64, y as i64), RockType::Stable),
                        'O' => rocks.insert((x as i64, y as i64), RockType::Rolling),
                        _ => Option::None,
                    };
                });
            });
        Platform {
            rocks,
            bounds: (max_x as i64, max_y as i64),
        }
    }

    fn rotate_platform_clockwise(&mut self) {
        self.rocks = self
            .rocks
            .iter()
            .map(|((x, y), rock)| ((self.bounds.1 - *y, *x), *rock))
            .collect::<HashMap<Point, RockType>>();
        self.bounds = (self.bounds.1, self.bounds.0);
    }

    fn tilt_platform_north(&mut self) {
        self.rocks
            .clone()
            .iter()
            .filter(|(_, rock)| **rock == RockType::Rolling)
            .for_each(|((x, y), _)| {
                let mut new_point = (*x, *y);
                let mut search_point = (*x, *y - 1);
                loop {
                    if search_point.1 < 0 {
                        break;
                    }
                    match self.rocks.get(&(search_point.0, search_point.1)) {
                        Some(RockType::Stable) => {
                            break;
                        }
                        Some(RockType::Rolling) => {
                            search_point = (search_point.0, search_point.1 - 1);
                        }
                        None => {
                            new_point = (search_point.0, search_point.1);
                            search_point = (search_point.0, search_point.1 - 1);
                        }
                    }
                }
                self.rocks.remove(&(*x, *y));
                self.rocks.insert(new_point, RockType::Rolling);
            })
    }

    fn run_tilt_cycle(&mut self) {
        self.tilt_platform_north();
        self.rotate_platform_clockwise();
        self.tilt_platform_north();
        self.rotate_platform_clockwise();
        self.tilt_platform_north();
        self.rotate_platform_clockwise();
        self.tilt_platform_north();
        self.rotate_platform_clockwise();
    }
}

fn get_north_load(rocks: &HashMap<Point, RockType>, bounds: &Point) -> i64 {
    rocks
        .iter()
        .filter(|(_, rock)| **rock == RockType::Rolling)
        .map(|((_, y), _)| (bounds.1 + 1) as i64 - *y)
        .sum()
}

fn solve_part_one() -> i64 {
    let mut platform = Platform::from_file("input/day14");
    platform.tilt_platform_north();
    get_north_load(&platform.rocks, &platform.bounds)
}

fn solve_part_two() -> i64 {
    let mut platform = Platform::from_file("input/day14");
    let mut states = Vec::new();
    loop {
        platform.run_tilt_cycle();
        if states.contains(&platform.rocks) {
            break;
        }
        states.push(platform.rocks.clone());
    }
    let loop_index = states
        .iter()
        .position(|state| state == &platform.rocks)
        .unwrap();
    let loop_len = states.len() - loop_index;
    let leftover = (1000000000 - states.len() as i64) % loop_len as i64;
    states
        .iter()
        .skip(loop_index)
        .enumerate()
        .filter_map(|(i, state)| {
            if i as i64 == leftover - 1 {
                Some(get_north_load(state, &platform.bounds))
            } else {
                None
            }
        })
        .sum::<i64>()
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
