struct RangeMapping {
    from: i64,
    to: i64,
    range: i64,
}

type RangeMappings = Vec<RangeMapping>;

struct ParsedInput {
    seeds: Vec<i64>,
    all_range_mappings: Vec<RangeMappings>,
}

fn parse_range_mappings(input: &str) -> RangeMappings {
    let mut split_input = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    split_input.remove(0);
    split_input
        .into_iter()
        .map(|line| {
            let numbers = line
                .split(" ")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            return RangeMapping {
                from: numbers[1],
                to: numbers[0],
                range: numbers[2],
            };
        })
        .collect::<RangeMappings>()
}

fn parse_input() -> ParsedInput {
    let input = std::fs::read_to_string("input/day05").unwrap();
    let mut input_parts = input
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    let seeds = input_parts[0].split("seeds: ").collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    input_parts.remove(0);
    let all_range_mappings = input_parts
        .into_iter()
        .map(|s| parse_range_mappings(s))
        .collect::<Vec<RangeMappings>>();
    ParsedInput {
        seeds,
        all_range_mappings,
    }
}

fn solve_part_one() -> i64 {
    let parsed_input = parse_input();
    return parsed_input
        .seeds
        .into_iter()
        .map(|seed| {
            let mut mapped_value: i64 = seed;
            parsed_input
                .all_range_mappings
                .iter()
                .for_each(|range_mappings| {
                    for range_mapping in range_mappings {
                        if mapped_value >= range_mapping.from
                            && mapped_value < range_mapping.from + range_mapping.range
                        {
                            mapped_value = range_mapping.to + (mapped_value - range_mapping.from);
                            break;
                        }
                    }
                });
            return mapped_value;
        })
        .collect::<Vec<i64>>()
        .into_iter()
        .min()
        .unwrap();
}

fn split_seed_range(seed_range: (i64, i64), range_mappings: &RangeMappings) -> Vec<(i64, i64)> {
    let mut split_ranges: Vec<(i64, i64)> = Vec::new();
    let mut start = seed_range.0;
    let stop = seed_range.1;
    let mut incision_points = range_mappings
        .iter()
        .map(|range_mapping| range_mapping.from)
        .filter(|range_from| *range_from > seed_range.0 && *range_from < seed_range.1)
        .collect::<Vec<i64>>();
    incision_points.sort();
    incision_points.reverse();
    while start < stop {
        let next_incision = match incision_points.pop() {
            Some(next_incision) => next_incision,
            None => stop + 1,
        };
        let transform = range_mappings
            .iter()
            .filter(|range_mapping| {
                range_mapping.from <= start && range_mapping.from + range_mapping.range >= start
            })
            .map(|range_mapping| range_mapping.to - range_mapping.from)
            .next();
        match transform {
            Some(transform) => {
                split_ranges.push((start + transform, next_incision + transform - 1));
            }
            None => {
                split_ranges.push((start, next_incision - 1));
            }
        }
        start = next_incision;
    }
    return split_ranges;
}

fn solve_part_two() -> i64 {
    let parsed_input = parse_input();
    let mut seed_ranges = Vec::new();
    for i in (0..parsed_input.seeds.len() - 1).step_by(2) {
        seed_ranges.push((
            parsed_input.seeds[i],
            parsed_input.seeds[i] + parsed_input.seeds[i + 1],
        ));
    }

    return seed_ranges
        .into_iter()
        .map(|seed_range| {
            let mut transformed_ranges = Vec::from([seed_range]);
            let mut new_ranges = Vec::new();
            parsed_input
                .all_range_mappings
                .iter()
                .for_each(|range_mappings| {
                    transformed_ranges
                        .clone()
                        .into_iter()
                        .for_each(|transformed_range| {
                            let split_ranges = split_seed_range(transformed_range, range_mappings);
                            new_ranges.extend(split_ranges);
                        });
                    transformed_ranges = new_ranges.clone();
                    new_ranges = Vec::new();
                });
            return transformed_ranges
                .into_iter()
                .map(|range| return if range.1 < range.0 { range.1 } else { range.0 })
                .min();
        })
        .min()
        .unwrap()
        .unwrap();
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
