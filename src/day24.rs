use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Coefficients {
    a: i128,
    b: i128,
    c: i128,
}

#[derive(Debug, Clone)]
struct Hail {
    position: (i128, i128, i128),
    velocity: (i128, i128, i128),
    coefficients: Coefficients,
}

impl Hail {
    fn new(position_vec: Vec<i128>, velocity_vec: Vec<i128>) -> Self {
        let p1 = (position_vec[0], position_vec[1], position_vec[2]);
        let velocity = (velocity_vec[0], velocity_vec[1], velocity_vec[2]);
        let p2 = (p1.0 + velocity.0, p1.1 + velocity.1, p1.2 + velocity.2);
        let coefficients = Coefficients {
            a: p1.1 - p2.1,
            b: p2.0 - p1.0,
            c: (p1.0 * p2.1) - (p2.0 * p1.1),
        };
        Hail {
            position: p1,
            velocity,
            coefficients,
        }
    }

    fn point_in_future_2d(&self, point: (i128, i128)) -> bool {
        let x_direction = (point.0 - self.position.0) / i128::abs(point.0 - self.position.0);
        let x_velocity_direction = (self.velocity.0) / i128::abs(self.velocity.0);
        x_direction == x_velocity_direction
    }
}

fn parse_input() -> Vec<Hail> {
    let file = std::fs::read_to_string("input/day24").unwrap();
    file.lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (position_str, velocity_str) = line.split_once(" @ ").unwrap();
            let position_vec = position_str
                .split(',')
                .map(|n| n.trim().parse::<i128>().unwrap())
                .collect::<Vec<i128>>();
            let velocity_vec = velocity_str
                .split(',')
                .map(|n| n.trim().parse::<i128>().unwrap())
                .collect::<Vec<i128>>();
            Hail::new(position_vec, velocity_vec)
        })
        .collect::<Vec<Hail>>()
}

fn hail_combinations(hails: Vec<Hail>) -> Vec<(Hail, Hail)> {
    hails
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, h1)| {
            (index..hails.len())
                .map(|index2| (h1.clone(), hails[index2].clone()))
                .collect::<Vec<(Hail, Hail)>>()
        })
        .flatten()
        .collect::<Vec<(Hail, Hail)>>()
}

fn find_intersection(h1: &Hail, h2: &Hail) -> Option<(i128, i128)> {
    let d = h1.coefficients.a * h2.coefficients.b - h1.coefficients.b * h2.coefficients.a;
    let dx = h1.coefficients.c * h2.coefficients.b - h1.coefficients.b * h2.coefficients.c;
    let dy = h1.coefficients.a * h2.coefficients.c - h1.coefficients.c * h2.coefficients.a;
    if d != 0 {
        Some((-1 * (dx / d), -1 * (dy / d)))
    } else {
        None
    }
}

fn solve_part_one() -> usize {
    const LOWER_BOUND: i128 = 200000000000000;
    const UPPER_BOUND: i128 = 400000000000000;
    let hails = parse_input();
    hail_combinations(hails)
        .into_iter()
        .filter_map(|(h1, h2)| match find_intersection(&h1, &h2) {
            Some((x, y)) => {
                if h1.point_in_future_2d((x, y))
                    && h2.point_in_future_2d((x, y))
                    && x >= LOWER_BOUND
                    && y >= LOWER_BOUND
                    && x <= UPPER_BOUND
                    && y <= UPPER_BOUND
                {
                    Some((x, y))
                } else {
                    None
                }
            }
            None => None,
        })
        .count()
}

fn solve_part_two() -> i128 {
    let hail_combinations = hail_combinations(parse_input());
    let mut potential_velocity_x: HashSet<i128> = HashSet::new();
    let mut potential_velocity_y: HashSet<i128> = HashSet::new();
    let mut potential_velocity_z: HashSet<i128> = HashSet::new();
    for (h1, h2) in hail_combinations.clone() {
        if h1.velocity.0 == h2.velocity.0 {
            let candidate = (-1000..1000)
                .filter(|velocity| {
                    *velocity != 0 as i128
                        && *velocity != h1.velocity.0
                        && (h2.position.0 - h1.position.0) % (velocity - h1.velocity.0) == 0
                })
                .collect::<HashSet<i128>>();
            if candidate.len() > 0 {
                if potential_velocity_x.len() == 0 {
                    potential_velocity_x = candidate;
                } else {
                    potential_velocity_x = potential_velocity_x
                        .intersection(&candidate)
                        .map(|x| *x)
                        .collect::<HashSet<i128>>();
                }
            }
        }
        if h1.velocity.1 == h2.velocity.1 {
            let candidate = (-1000..1000)
                .filter(|velocity| {
                    *velocity != 0 as i128
                        && *velocity != h1.velocity.1
                        && (h2.position.1 - h1.position.1) % (velocity - h1.velocity.1) == 0
                })
                .collect::<HashSet<i128>>();
            if candidate.len() > 0 {
                if potential_velocity_y.len() == 0 {
                    potential_velocity_y = candidate;
                } else {
                    potential_velocity_y = potential_velocity_y
                        .intersection(&candidate)
                        .map(|y| *y)
                        .collect::<HashSet<i128>>();
                }
            }
        }
        if h1.velocity.2 == h2.velocity.2 {
            let candidate = (-1000..1000)
                .filter(|velocity| {
                    *velocity != 0 as i128
                        && *velocity != h1.velocity.2
                        && (h2.position.2 - h1.position.2) % (velocity - h1.velocity.2) == 0
                })
                .collect::<HashSet<i128>>();
            if candidate.len() > 0 {
                if potential_velocity_z.len() == 0 {
                    potential_velocity_z = candidate;
                } else {
                    potential_velocity_z = potential_velocity_z
                        .intersection(&candidate)
                        .map(|z| *z)
                        .collect::<HashSet<i128>>();
                }
            }
        }
        if potential_velocity_x.len() == 1
            && potential_velocity_y.len() == 1
            && potential_velocity_z.len() == 1
        {
            break;
        }
    }
    let rock_velocity = (
        potential_velocity_x.into_iter().next().unwrap(),
        potential_velocity_y.into_iter().next().unwrap(),
        potential_velocity_z.into_iter().next().unwrap(),
    );

    let (h1, h2) = hail_combinations.into_iter().skip(1).next().unwrap();
    let h1_y_diverge_per_x =
        (h1.velocity.1 - rock_velocity.1) as f64 / (h1.velocity.0 - rock_velocity.0) as f64;
    let h2_y_diverge_per_x =
        (h2.velocity.1 - rock_velocity.1) as f64 / (h2.velocity.0 - rock_velocity.0) as f64;

    let h1_y_at_x_zero = h1.position.1 as f64 - h1_y_diverge_per_x * h1.position.0 as f64;
    let h2_y_at_x_zero = h2.position.1 as f64 - h2_y_diverge_per_x * h2.position.0 as f64;
    let x =
        f64::round((h2_y_at_x_zero - h1_y_at_x_zero) / (h1_y_diverge_per_x - h2_y_diverge_per_x));

    let h1_intersection_time =
        (x - h1.position.0 as f64) / (h1.velocity.0 as f64 - rock_velocity.0 as f64);
    let y = f64::round(
        h1.position.1 as f64
            + (h1.velocity.1 as f64 - rock_velocity.1 as f64) * h1_intersection_time,
    );
    let z = f64::round(
        h1.position.2 as f64
            + (h1.velocity.2 as f64 - rock_velocity.2 as f64) * h1_intersection_time,
    );
    x as i128 + y as i128 + z as i128
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
