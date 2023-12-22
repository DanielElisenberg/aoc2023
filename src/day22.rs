use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Brick {
    identifier: i64,
    start: (i64, i64, i64),
    stop: (i64, i64, i64),
    size: (i64, i64, i64),
}

impl Brick {
    fn new(identifier: i64, start: (i64, i64, i64), stop: (i64, i64, i64)) -> Brick {
        let size = (stop.0 - start.0, stop.1 - start.1, stop.2 - start.2);
        Brick {
            identifier,
            start,
            stop,
            size,
        }
    }
    fn as_cubes(&self) -> Vec<(i64, i64, i64)> {
        match self.size {
            (0, 0, 0) => vec![(self.start.0, self.start.1, self.start.2)],
            (0, 0, length) => (0..i64::abs(length) + 1)
                .map(|z| {
                    (
                        self.start.0,
                        self.start.1,
                        self.start.2 + (length / i64::abs(length)) * z,
                    )
                })
                .collect(),
            (0, length, 0) => (0..i64::abs(length) + 1)
                .map(|y| {
                    (
                        self.start.0,
                        self.start.1 + (length / i64::abs(length)) * y,
                        self.start.2,
                    )
                })
                .collect(),
            (length, 0, 0) => (0..i64::abs(length) + 1)
                .map(|x| {
                    (
                        self.start.0 + (length / i64::abs(length)) * x,
                        self.start.1,
                        self.start.2,
                    )
                })
                .collect(),
            _ => panic!("Invalid brick size"),
        }
    }
    fn intersects(&self, other: &Brick) -> bool {
        self.as_cubes()
            .into_iter()
            .any(|cube| other.as_cubes().contains(&cube))
    }

    fn lowest_z(&self) -> i64 {
        vec![self.start.2, self.stop.2].into_iter().min().unwrap()
    }
}

fn parse_bricks() -> Vec<Brick> {
    let file = std::fs::read_to_string("input/day22").unwrap();
    let mut identifier = 0;
    file.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let bounds = l.split("~").collect::<Vec<&str>>();
            let start: Vec<i64> = bounds[0]
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            let stop: Vec<i64> = bounds[1]
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            identifier += 1;
            Brick::new(
                identifier,
                (start[0], start[1], start[2]),
                (stop[0], stop[1], stop[2]),
            )
        })
        .collect()
}

fn drop_bricks(bricks: Vec<Brick>) -> Vec<Brick> {
    let mut dropped_bricks = bricks.clone();
    let mut changed = true;
    while changed {
        changed = false;
        dropped_bricks.sort_by(|a, b| a.lowest_z().cmp(&b.lowest_z()));
        dropped_bricks = dropped_bricks
            .clone()
            .into_iter()
            .map(|b| {
                let z = b.lowest_z();
                if z == 0 {
                    return b;
                }
                let dropped_brick = Brick::new(
                    b.identifier,
                    (b.start.0, b.start.1, b.start.2 - 1),
                    (b.stop.0, b.stop.1, b.stop.2 - 1),
                );
                if dropped_bricks
                    .clone()
                    .into_iter()
                    .any(|db| db.identifier != b.identifier && db.intersects(&dropped_brick))
                {
                    b
                } else {
                    changed = true;
                    dropped_brick
                }
            })
            .collect();
    }
    return dropped_bricks;
}
fn generate_dependency_map(bricks: Vec<Brick>) -> HashMap<i64, Vec<i64>> {
    bricks
        .clone()
        .into_iter()
        .map(|b| {
            (
                b.identifier.clone(),
                bricks
                    .clone()
                    .into_iter()
                    .filter(|bb| {
                        let new_bb = Brick::new(
                            bb.identifier,
                            (b.start.0, b.start.1, b.start.2 - 1),
                            (b.stop.0, b.stop.1, b.stop.2 - 1),
                        );
                        bb.intersects(&new_bb) && bb.identifier != b.identifier
                    })
                    .map(|bb| bb.identifier)
                    .collect(),
            )
        })
        .collect()
}

fn bricks_left_after_chain_reaction(
    remove_brick: i64,
    dependency_map: HashMap<i64, Vec<i64>>,
) -> Vec<i64> {
    let mut remove_bricks = vec![remove_brick];
    let mut dependencies = dependency_map.clone();
    while remove_bricks.len() > 0 {
        dependencies = dependencies
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    v.into_iter()
                        .filter(|ib| !remove_bricks.contains(ib))
                        .collect::<Vec<i64>>(),
                )
            })
            .collect::<HashMap<i64, Vec<i64>>>();

        remove_bricks = dependencies
            .clone()
            .into_iter()
            .filter(|(_, v)| v.len() == 0)
            .map(|(k, _)| k)
            .collect();
        dependencies = dependencies
            .into_iter()
            .filter(|(_, v)| v.len() > 0)
            .collect();
    }
    dependencies
        .into_iter()
        .map(|(k, _)| k)
        .collect::<Vec<i64>>()
}
fn solve_part_one() -> i64 {
    let bricks = drop_bricks(parse_bricks());
    let dependency_map: HashMap<i64, Vec<i64>> = generate_dependency_map(bricks.clone());
    let sole_dependant_bricks: HashSet<i64> = dependency_map
        .clone()
        .into_iter()
        .filter(|(_, v)| v.len() == 1)
        .map(|(_, v)| v)
        .flatten()
        .collect();
    bricks
        .into_iter()
        .filter(|b| !sole_dependant_bricks.contains(&b.identifier))
        .count() as i64
}

fn solve_part_two() -> i64 {
    let bricks = drop_bricks(parse_bricks());
    let dependency_map: HashMap<i64, Vec<i64>> = generate_dependency_map(bricks.clone());
    let dependency_map_no_roots = dependency_map
        .clone()
        .into_iter()
        .filter(|(_, v)| v.len() > 0)
        .collect::<HashMap<i64, Vec<i64>>>();
    let roots = dependency_map
        .clone()
        .into_iter()
        .filter(|(_, v)| v.len() == 0)
        .map(|(k, _)| k)
        .collect::<Vec<i64>>();
    bricks
        .clone()
        .into_iter()
        .map(|b| {
            bricks.len() as i64
                - bricks_left_after_chain_reaction(b.identifier, dependency_map_no_roots.clone())
                    .len() as i64
                - roots.len() as i64
        })
        .sum()
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
