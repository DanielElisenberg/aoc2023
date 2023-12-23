use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Open,
    RightSlope,
    LeftSlope,
    DownSlope,
    UpSlope,
}

fn parse_map() -> HashMap<(i32, i32), Tile> {
    let file = std::fs::read_to_string("input/day23").unwrap();
    file.lines()
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => ((x as i32, y as i32), Tile::Wall),
                    '.' => ((x as i32, y as i32), Tile::Open),
                    '>' => ((x as i32, y as i32), Tile::RightSlope),
                    '<' => ((x as i32, y as i32), Tile::LeftSlope),
                    'v' => ((x as i32, y as i32), Tile::DownSlope),
                    '^' => ((x as i32, y as i32), Tile::UpSlope),
                    _ => panic!("Invalid character {}", c),
                })
                .collect::<Vec<((i32, i32), Tile)>>()
        })
        .flatten()
        .collect::<HashMap<(i32, i32), Tile>>()
}
fn find_neighbours(x: i32, y: i32, map: &HashMap<(i32, i32), Tile>) -> Vec<(i32, i32)> {
    let mut neighbours = Vec::new();
    match map.get(&(x + 1, y)) {
        Some(Tile::Open) => neighbours.push((x + 1, y)),
        Some(Tile::RightSlope) => {
            neighbours.push((x + 1, y));
        }
        _ => {}
    }
    match map.get(&(x - 1, y)) {
        Some(Tile::Open) => neighbours.push((x - 1, y)),
        Some(Tile::LeftSlope) => {
            neighbours.push((x - 1, y));
        }
        _ => {}
    }
    match map.get(&(x, y + 1)) {
        Some(Tile::Open) => neighbours.push((x, y + 1)),
        Some(Tile::DownSlope) => {
            neighbours.push((x, y + 1));
        }
        _ => {}
    }
    match map.get(&(x, y - 1)) {
        Some(Tile::Open) => neighbours.push((x, y - 1)),
        Some(Tile::UpSlope) => {
            neighbours.push((x, y - 1));
        }
        _ => {}
    }
    return neighbours;
}

fn solve_part_one() -> i32 {
    let map = parse_map();
    let max_y = map.keys().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_x = map.keys().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let mut visited = Vec::from([(1, 0)]);
    let mut search_from = Vec::from([((1, 0), 0)]);
    while search_from.len() > 0 {
        search_from.sort_by(|a, b| b.1.cmp(&a.1));
        let (current, steps) = search_from.remove(0);
        if current.1 == max_y && current.0 == max_x - 1 {
            return steps;
        }
        find_neighbours(current.0, current.1, &map)
            .into_iter()
            .for_each(|n| {
                if visited.contains(&n) {
                    return;
                }
                visited.push(n);
                search_from.push((n, steps + 1));
            });
    }
    0
}

fn generate_simple_graph(
    start: (i32, i32),
    goal: (i32, i32),
    map: &HashMap<(i32, i32), Tile>,
) -> HashMap<((i32, i32), (i32, i32)), i32> {
    let crossroads = map
        .clone()
        .into_iter()
        .filter_map(|(k, v)| {
            if k == goal || k == start {
                Some(k)
            } else if v == Tile::Open && find_neighbours(k.0, k.1, map).len() > 2 {
                Some(k)
            } else {
                None
            }
        })
        .collect::<Vec<(i32, i32)>>();
    let mut simple_graph = HashMap::new();
    crossroads.into_iter().for_each(|from| {
        let mut search_from: Vec<((i32, i32), i32)> = find_neighbours(from.0, from.1, map)
            .iter()
            .map(|n| (*n, 1))
            .collect();
        let mut visited = Vec::from([from]);
        while search_from.len() > 0 {
            let (current, steps) = search_from.remove(0);
            visited.push(current);
            if current == goal || current == start {
                simple_graph.insert((from, current), steps);
                continue;
            }
            let neighbours = find_neighbours(current.0, current.1, map)
                .into_iter()
                .filter(|n| !visited.contains(n))
                .collect::<Vec<(i32, i32)>>();
            if neighbours.len() == 0 {
                continue;
            }
            if neighbours.len() > 1 {
                simple_graph.insert((from, current), steps);
                continue;
            }
            visited.push(neighbours[0]);
            search_from.push((neighbours[0], steps + 1));
        }
    });
    simple_graph
}

fn find_longest_path(
    current: (i32, i32),
    goal: (i32, i32),
    visited: Vec<(i32, i32)>,
    total_distance: i32,
    simple_graph: &HashMap<((i32, i32), (i32, i32)), i32>,
) -> i32 {
    if current == goal {
        return total_distance;
    }
    let results = simple_graph
        .clone()
        .into_iter()
        .filter_map(|((from, to), distance)| {
            if from == current {
                Some((to, distance))
            } else {
                None
            }
        })
        .filter(|(to, _)| !visited.contains(&to))
        .map(|(to, distance)| {
            let mut new_visited = visited.clone();
            new_visited.push(to);
            find_longest_path(
                to,
                goal,
                new_visited,
                total_distance + distance,
                simple_graph,
            )
        })
        .collect::<Vec<i32>>();
    if results.len() == 0 {
        return 0;
    } else {
        *results.iter().max().unwrap()
    }
}

fn solve_part_two() -> i32 {
    let map = parse_map()
        .into_iter()
        .map(|(k, v)| (k, if v == Tile::Wall { v } else { Tile::Open }))
        .collect::<HashMap<(i32, i32), Tile>>();
    let max_y = map.keys().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_x = map.keys().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let simple_graph = generate_simple_graph((1, 0), (max_x - 1, max_y), &map);
    find_longest_path((1, 0), (max_x - 1, max_y), vec![(1, 0)], 0, &simple_graph)
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
