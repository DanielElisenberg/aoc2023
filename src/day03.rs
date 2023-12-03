use std::collections::HashMap;

struct NumberLocation {
    line_number: usize,
    from: usize,
    to: usize,
}

struct EngineSchematic {
    number_locations: Vec<(i32, NumberLocation)>,
    symbols: HashMap<(usize, usize), char>,
}

fn parse_engine_schematic() -> EngineSchematic {
    let mut symbols: HashMap<(usize, usize), char> = HashMap::new();
    let mut number_locations: Vec<(i32, NumberLocation)> = Vec::new();
    std::fs::read_to_string("input/day03")
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
        .into_iter()
        .enumerate()
        .for_each(|(line_number, line)| {
            let mut parsed_number_string = String::from("");
            let mut number_from_index = 0;
            line.clone()
                .into_iter()
                .enumerate()
                .for_each(|(char_index, char)| {
                    if char.is_numeric() {
                        if parsed_number_string == "" {
                            number_from_index = char_index;
                        }
                        parsed_number_string.push(char);
                    }
                    if parsed_number_string != "" && !char.is_numeric()
                        || char_index == line.len() - 1 && parsed_number_string != ""
                    {
                        let number_to_index = if char_index == line.len() - 1 {
                            char_index
                        } else {
                            char_index - 1
                        };
                        let _ = number_locations.push((
                            parsed_number_string.parse::<i32>().unwrap(),
                            NumberLocation {
                                line_number,
                                from: number_from_index,
                                to: number_to_index,
                            },
                        ));
                        parsed_number_string = String::from("");
                    }
                    if !char.is_numeric() && char != '.' {
                        symbols.insert((line_number, char_index), char);
                    }
                })
        });
    return EngineSchematic {
        number_locations,
        symbols,
    };
}

fn is_adjecent_to_symbol(
    number_location: &NumberLocation,
    symbols: HashMap<(usize, usize), char>,
) -> bool {
    let mut surrounding_positions: Vec<(usize, usize)> = Vec::new();
    let from_line_number = if number_location.line_number == 0 {
        0
    } else {
        number_location.line_number - 1
    };
    let from_char_index = if number_location.from == 0 {
        0
    } else {
        number_location.from - 1
    };
    for line_number in from_line_number..number_location.line_number + 2 {
        for char_index in from_char_index..number_location.to + 2 {
            surrounding_positions.push((line_number, char_index));
        }
    }
    return surrounding_positions
        .into_iter()
        .any(|(y, x)| symbols.contains_key(&(y, x)));
}

fn gear_sum(x: usize, y: usize, number_locations: &Vec<(i32, NumberLocation)>) -> i32 {
    let from_line_number = if y == 0 { 0 } else { y - 1 };
    let from_char_index = if x == 0 { 0 } else { x - 1 };
    let adjecent_numbers = number_locations
        .into_iter()
        .filter(|(_, number_location)| {
            return (from_line_number..y + 2).contains(&number_location.line_number)
                && ((from_char_index..x + 2).contains(&number_location.from)
                    || (from_char_index..x + 2).contains(&number_location.to));
        })
        .map(|(number, _)| number)
        .collect::<Vec<&i32>>();
    return if adjecent_numbers.len() == 2 {
        adjecent_numbers[0] * adjecent_numbers[1]
    } else {
        0
    };
}

fn solve_part_one() -> i32 {
    let engine_schematic = parse_engine_schematic();
    return engine_schematic
        .number_locations
        .iter()
        .map(|(number, number_location)| {
            return if is_adjecent_to_symbol(number_location, engine_schematic.symbols.clone()) {
                *number
            } else {
                0
            };
        })
        .sum::<i32>();
}

fn solve_part_two() -> i32 {
    let engine_schematic = parse_engine_schematic();
    return engine_schematic
        .symbols
        .iter()
        .filter(|(_, symbol)| symbol.to_owned().to_owned() == '*')
        .map(|((y, x), _)| gear_sum(*x, *y, &engine_schematic.number_locations))
        .sum::<i32>();
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
