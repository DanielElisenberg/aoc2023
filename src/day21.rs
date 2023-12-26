use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct GardenPatch {
    rocks: HashSet<(i64, i64)>,
    start_point: (i64, i64),
}

fn parse_map(expand: i64) -> GardenPatch {
    let mut rocks = HashSet::new();
    let files = std::fs::read_to_string("input/day21").unwrap();
    let lines = files.split("\n").map(|s| s.trim()).collect::<Vec<&str>>();
    let mut start_point = (0, 0);
    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                rocks.insert((x as i64, y as i64));
            }
            if c == 'S' {
                start_point = (x as i64, y as i64);
            }
        })
    });
    let size = lines.len() as i64;
    rocks.clone().into_iter().for_each(|(x, y)| {
        (1..expand).for_each(|expand_step| {
            let expand_to = expand_step as i64 * (size - 1);
            rocks.insert((x + expand_to, y));
            rocks.insert((x - expand_to, y));
        });
    });
    rocks.clone().into_iter().for_each(|(x, y)| {
        (1..expand).for_each(|expansion_step| {
            let expand_to = expansion_step as i64 * (size - 1);
            rocks.insert((x, y + expand_to));
            rocks.insert((x, y - expand_to));
        })
    });
    GardenPatch { rocks, start_point }
}

fn find_neighbours(x: i64, y: i64) -> Vec<(i64, i64)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn count_distances(garden_patch: &GardenPatch, search_bounds: i64) -> HashMap<i64, i64> {
    let mut distance_counts: HashMap<i64, i64> = HashMap::new();
    let mut visited = HashSet::new();
    let mut search_from = Vec::from([(garden_patch.start_point, 0)]);
    visited.insert(garden_patch.start_point);
    distance_counts.insert(0, 1);
    while search_from.len() > 0 {
        search_from.sort_by(|a, b| a.1.cmp(&b.1));
        let (current, steps) = search_from.remove(0);
        if steps > search_bounds {
            continue;
        }
        let neighbours = find_neighbours(current.0, current.1)
            .into_iter()
            .filter(|n| !garden_patch.rocks.contains(n) && !visited.contains(&n))
            .collect::<Vec<(i64, i64)>>();
        for neighbour in neighbours {
            visited.insert(neighbour);
            search_from.push((neighbour, steps + 1));
            if distance_counts.contains_key(&(steps + 1)) {
                *distance_counts.get_mut(&(steps + 1)).unwrap() += 1;
            } else {
                distance_counts.insert(steps + 1, 1);
            }
        }
    }
    distance_counts
}

fn garden_plots_reached(distance_counts: &HashMap<i64, i64>, steps_taken: i64) -> i64 {
    distance_counts
        .clone()
        .into_iter()
        .filter_map(|(distance, count)| {
            if (distance % 2 == steps_taken % 2) && distance <= steps_taken {
                Some(count)
            } else {
                None
            }
        })
        .sum::<i64>()
}

fn solve_quadratic_equation(samples: Vec<i64>, n: i64) -> i64 {
    let a = (samples[2] - (2 * samples[1]) + samples[0]) / 2;
    let b = samples[1] - samples[0] - a;
    let c = samples[0];
    a * i64::pow(n, 2) + b * n + c
}

fn solve_part_one() -> i64 {
    garden_plots_reached(&count_distances(&parse_map(0), 64), 64)
}

fn solve_part_two() -> i64 {
    let distance_counts = count_distances(&parse_map(3), 65 + 131 + 131);
    let samples = vec![
        garden_plots_reached(&distance_counts, 65),
        garden_plots_reached(&distance_counts, 65 + 131),
        garden_plots_reached(&distance_counts, 65 + 131 + 131),
    ];
    solve_quadratic_equation(samples, (26501365 - 65) / 131)
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
