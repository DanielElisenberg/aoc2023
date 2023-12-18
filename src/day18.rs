enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct DigInstruction {
    direction: Direction,
    meters: i64,
}

fn simple_dig_instructions() -> Vec<DigInstruction> {
    let file = std::fs::read_to_string("input/day18").unwrap();
    file.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let split_line = l.split(" ").collect::<Vec<&str>>();
            DigInstruction {
                direction: match split_line[0] {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("Invalid direction"),
                },
                meters: split_line[1].parse().unwrap(),
            }
        })
        .collect()
}

fn hex_dig_instructions() -> Vec<DigInstruction> {
    let file = std::fs::read_to_string("input/day18").unwrap();
    file.lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let hex = l.split(" ").collect::<Vec<&str>>()[2];
            let meter_hex = i64::from_str_radix(
                &(hex.chars().skip(2).take(hex.len() - 4).collect::<String>()),
                16,
            )
            .unwrap();
            let direction_char = hex.chars().nth(hex.len() - 2).unwrap();
            DigInstruction {
                direction: match direction_char {
                    '3' => Direction::Up,
                    '1' => Direction::Down,
                    '2' => Direction::Left,
                    '0' => Direction::Right,
                    _ => panic!("Invalid direction"),
                },
                meters: meter_hex,
            }
        })
        .collect()
}

fn shoelace(trench_map: &Vec<(i64, i64)>) -> i64 {
    let mut sum1 = 0;
    let mut sum2 = 0;
    (0..trench_map.len() - 1).for_each(|i| {
        sum1 = sum1 + trench_map[i].0 * trench_map[i + 1].1;
        sum2 = sum2 + trench_map[i].1 * trench_map[i + 1].0;
    });
    sum1 = sum1 + trench_map[trench_map.len() - 1].0 * trench_map[0].1;
    sum2 = sum2 + trench_map[0].0 * trench_map[trench_map.len() - 1].1;

    (sum1 - sum2).abs() / 2
}

fn total_trench_volume(dig_instructions: &Vec<DigInstruction>) -> i64 {
    let mut trench_map = Vec::new();
    let mut outline_count = 0;
    let mut position = (0 as i64, 0 as i64);
    dig_instructions.iter().for_each(|dig_instruction| {
        let velocity = match dig_instruction.direction {
            Direction::Up => (-1 * dig_instruction.meters, 0),
            Direction::Down => (1 * dig_instruction.meters, 0),
            Direction::Left => (0, -1 * dig_instruction.meters),
            Direction::Right => (0, 1 * dig_instruction.meters),
        };
        position = (position.0 + velocity.0, position.1 + velocity.1);
        outline_count += dig_instruction.meters;
        trench_map.push(position);
    });
    shoelace(&trench_map) + outline_count / 2 + 1
}

fn solve_part_one() -> i64 {
    total_trench_volume(&simple_dig_instructions())
}

fn solve_part_two() -> i64 {
    total_trench_volume(&hex_dig_instructions())
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
