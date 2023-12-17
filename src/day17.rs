use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Path {
    position: (i32, i32),
    my_path: Vec<(i32, i32)>,
    heat_loss: i32,
    direction: Direction,
    direction_count: i32,
}

fn parse_city_map() -> Vec<Vec<i32>> {
    let file = std::fs::read_to_string("input/day17").unwrap();
    file.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect()
}

fn get_neighbours(
    current: &Path,
    city_bounds: (usize, usize),
) -> Vec<((i32, i32), Direction, i32)> {
    let mut surrounding = Vec::new();
    if current.position.0 > 0 {
        surrounding.push(((current.position.0 - 1, current.position.1), Direction::Up));
    }
    if current.position.0 < city_bounds.0 as i32 - 1 {
        surrounding.push((
            (current.position.0 + 1, current.position.1),
            Direction::Down,
        ));
    }
    if current.position.1 > 0 {
        surrounding.push((
            (current.position.0, current.position.1 - 1),
            Direction::Left,
        ));
    }
    if current.position.1 < city_bounds.1 as i32 - 1 {
        surrounding.push((
            (current.position.0, current.position.1 + 1),
            Direction::Right,
        ));
    }
    surrounding
        .iter()
        .map(|(position, direction)| {
            (
                *position,
                *direction,
                if *direction == current.direction {
                    current.direction_count + 1
                } else {
                    1
                },
            )
        })
        .collect()
}

fn initiate_paths() -> Vec<Path> {
    Vec::from([Path {
        position: (0, 0),
        my_path: Vec::from([(0, 0)]),
        heat_loss: 0,
        direction: Direction::Right,
        direction_count: 0,
    }])
}

fn solve_part_one() -> i32 {
    let city_map = parse_city_map();
    let city_bounds = (city_map.len(), city_map[0].len());
    let mut visited = HashSet::new();
    let mut paths = initiate_paths();
    loop {
        paths.sort_by_key(|p| p.heat_loss);
        let shortest = paths.remove(0);
        if shortest.position == (city_bounds.0 as i32 - 1, city_bounds.1 as i32 - 1) {
            return shortest.heat_loss;
        }
        let neighbours = get_neighbours(&shortest, city_bounds)
            .iter()
            .filter(|(p, d, dc)| {
                !visited.contains(&(*p, *d, *dc)) && *dc < 4 && !shortest.my_path.contains(p)
            })
            .map(|(p, d, dc)| (*p, *d, *dc))
            .collect::<Vec<((i32, i32), Direction, i32)>>();
        for (neighbor, direction, direction_count) in neighbours {
            visited.insert((neighbor, direction, direction_count));
            let mut n_path = shortest.my_path.clone();
            n_path.push(neighbor);
            paths.push(Path {
                position: neighbor,
                my_path: n_path,
                heat_loss: shortest.heat_loss + city_map[neighbor.0 as usize][neighbor.1 as usize],
                direction,
                direction_count,
            });
        }
    }
}

fn solve_part_two() -> i32 {
    let city_map = parse_city_map();
    let city_bounds = (city_map.len(), city_map[0].len());
    let mut visited = HashSet::new();
    let mut paths = initiate_paths();
    loop {
        paths.sort_by_key(|p| p.heat_loss);
        let shortest = paths.remove(0);
        if shortest.position == (city_bounds.0 as i32 - 1, city_bounds.1 as i32 - 1) {
            if shortest.direction_count < 4 {
                continue;
            }
            return shortest.heat_loss;
        }
        let neighbours = get_neighbours(&shortest, city_bounds)
            .iter()
            .filter(|(p, d, dc)| {
                !visited.contains(&(*p, *d, *dc))
                    && *dc < 11
                    && !shortest.my_path.contains(p)
                    && (!(shortest.direction_count < 4 && *d != shortest.direction)
                        || shortest.direction_count == 0)
            })
            .map(|(p, d, dc)| (*p, *d, *dc))
            .collect::<Vec<((i32, i32), Direction, i32)>>();
        for (neighbor, direction, direction_count) in neighbours {
            visited.insert((neighbor, direction, direction_count));
            let mut n_path = shortest.my_path.clone();
            n_path.push(neighbor);
            paths.push(Path {
                position: neighbor,
                my_path: n_path,
                heat_loss: shortest.heat_loss + city_map[neighbor.0 as usize][neighbor.1 as usize],
                direction,
                direction_count,
            });
        }
    }
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
