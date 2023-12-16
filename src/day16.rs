use std::collections::HashSet;

type Point = (i32, i32);

#[derive(Clone, Copy, Debug)]
struct Beam {
    position: Point,
    velocity: Point,
    terminated: bool,
}

impl Beam {
    fn new(positions: Point, velocity: Point) -> Self {
        Self {
            position: positions,
            velocity,
            terminated: false,
        }
    }
    fn step(&mut self) {
        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );
    }
    fn terminate(&mut self) {
        self.terminated = true;
    }
}

fn parse_contraption() -> Vec<Vec<char>> {
    let file = std::fs::read_to_string("input/day16").unwrap();
    file.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

fn run_simulation(contraption: &Vec<Vec<char>>, first_beam: Beam) -> i32 {
    let mut beams = Vec::from([first_beam]);
    let mut visited = Vec::from([(first_beam.position, first_beam.velocity)]);
    while beams.clone().iter().any(|b| !b.terminated) {
        let mut new_beams = Vec::new();
        for beam in beams.iter_mut() {
            if beam.terminated {
                continue;
            }
            let current_position = beam.position;
            let tile = contraption[current_position.0 as usize][current_position.1 as usize];
            beam.velocity = match (tile, beam.velocity) {
                ('.', _) => beam.velocity,
                ('-', (0, _)) => beam.velocity,
                ('-', (_, 0)) => {
                    let mut new_beam = beam.clone();
                    new_beam.velocity = (0, -1);
                    new_beams.push(new_beam);
                    (0, 1)
                }
                ('|', (_, 0)) => beam.velocity,
                ('|', (0, _)) => {
                    let mut new_beam = beam.clone();
                    new_beam.velocity = (-1, 0);
                    new_beams.push(new_beam);
                    (1, 0)
                }
                ('/', (1, _)) => (0, -1),
                ('/', (-1, _)) => (0, 1),
                ('/', (_, 1)) => (-1, 0),
                ('/', (_, -1)) => (1, 0),
                ('\\', (1, _)) => (0, 1),
                ('\\', (-1, _)) => (0, -1),
                ('\\', (_, 1)) => (1, 0),
                ('\\', (_, -1)) => (-1, 0),
                _ => panic!(
                    "invalid tile: {} with velocity: {:?}",
                    tile, current_position
                ),
            };
        }
        beams.extend(new_beams.clone());
        for beam in beams.iter_mut() {
            if beam.terminated {
                continue;
            }
            beam.step();
            if beam.position.0 < 0
                || beam.position.1 < 0
                || beam.position.0 >= contraption.len() as i32
                || beam.position.1 >= contraption[0].len() as i32
            {
                beam.terminate();
                continue;
            }
            if visited.contains(&(beam.position, beam.velocity)) {
                beam.terminate();
            } else {
                visited.push((beam.position, beam.velocity));
            }
        }
    }
    visited.iter().map(|(p, _)| p).collect::<HashSet<_>>().len() as i32
}

fn solve_part_one() -> i32 {
    run_simulation(&parse_contraption(), Beam::new((0, 0), (0, 1)))
}

fn solve_part_two() -> i32 {
    let contraption = parse_contraption();
    let mut max_energized = 0;
    (0..contraption.len() as i32).for_each(|y| {
        let energized = run_simulation(&contraption, Beam::new((y, 0), (0, 1)));
        if energized > max_energized {
            max_energized = energized
        }
        let energized = run_simulation(
            &contraption,
            Beam::new((y, contraption[0].len() as i32 - 1), (0, -1)),
        );
        if energized > max_energized {
            max_energized = energized
        }
    });
    (0..contraption[0].len() as i32).for_each(|x| {
        let energized = run_simulation(&contraption, Beam::new((0, x), (1, 0)));
        if energized > max_energized {
            max_energized = energized
        }
        let energized = run_simulation(
            &contraption,
            Beam::new((contraption.len() as i32 - 1, x), (-1, 0)),
        );
        if energized > max_energized {
            max_energized = energized
        }
    });
    return max_energized;
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
