type Pattern = Vec<Vec<char>>;

fn transpose_pattern(pattern: Pattern) -> Pattern {
    (0..pattern[0].len())
        .map(|x| {
            (0..pattern.len())
                .map(|y| pattern[y][x])
                .collect::<Vec<char>>()
        })
        .collect()
}

fn one_off(a: &Vec<char>, b: &Vec<char>) -> bool {
    let mut mismatch = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            mismatch += 1;
        }
        if mismatch > 1 {
            return false;
        }
    }
    if mismatch == 0 {
        false
    } else {
        true
    }
}

fn lines_above_reflection_point(pattern: Pattern) -> i64 {
    let reflection_points = pattern
        .windows(2)
        .enumerate()
        .filter_map(|(index, window)| {
            if window[0] == window[1] {
                Some(index)
            } else {
                None
            }
        })
        .filter(|index| {
            let mut low_scan = *index as i32;
            let mut high_scan = *index as i32 + 1;
            while low_scan >= 0 && high_scan < pattern.len() as i32 {
                if pattern[low_scan as usize] != pattern[high_scan as usize] {
                    return false;
                }
                low_scan -= 1;
                high_scan += 1;
            }
            return true;
        })
        .collect::<Vec<usize>>();
    if reflection_points.len() != 1 {
        0
    } else {
        reflection_points[0] as i64 + 1
    }
}

fn lines_above_reflection_point_one_off(pattern: Pattern) -> i64 {
    let reflection_points = pattern
        .windows(2)
        .enumerate()
        .filter_map(|(index, window)| {
            if window[0] == window[1] {
                Some(index)
            } else if one_off(&window[0], &window[1]) {
                Some(index)
            } else {
                None
            }
        })
        .filter_map(|index| {
            let mut low_scan = index as i32;
            let mut high_scan = index as i32 + 1;
            let mut corrections = 0;
            while low_scan >= 0 && high_scan < pattern.len() as i32 {
                if pattern[low_scan as usize] != pattern[high_scan as usize] {
                    if one_off(&pattern[low_scan as usize], &pattern[high_scan as usize])
                        && corrections == 0
                    {
                        corrections += 1
                    } else {
                        return None;
                    }
                }
                low_scan -= 1;
                high_scan += 1;
            }
            return Some((corrections, index));
        })
        .filter(|(corrections, _)| *corrections == 1)
        .collect::<Vec<(usize, usize)>>();
    if reflection_points.len() == 0 {
        0
    } else if reflection_points.len() > 1 {
        panic!("Multiple reflection points found: {:?}", reflection_points);
    } else {
        reflection_points[0].1 as i64 + 1
    }
}

fn solve_part_one() -> i64 {
    let file = std::fs::read_to_string("input/day13").unwrap();
    let patterns = file
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect()
        })
        .collect::<Vec<Pattern>>();
    let lines_to_the_left: i64 = patterns
        .clone()
        .iter()
        .map(|pattern| lines_above_reflection_point(transpose_pattern(pattern.to_vec())))
        .sum();

    let lines_above: i64 = patterns
        .iter()
        .map(|pattern| lines_above_reflection_point(pattern.to_vec()))
        .sum();
    return lines_to_the_left + (100 * lines_above);
}

fn solve_part_two() -> i64 {
    let file = std::fs::read_to_string("input/day13").unwrap();
    let patterns = file
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect()
        })
        .collect::<Vec<Pattern>>();
    let lines_to_the_left: i64 = patterns
        .clone()
        .iter()
        .map(|pattern| lines_above_reflection_point_one_off(transpose_pattern(pattern.to_vec())))
        .sum();
    let lines_above: i64 = patterns
        .iter()
        .map(|pattern| lines_above_reflection_point_one_off(pattern.to_vec()))
        .sum();
    return lines_to_the_left + (100 * lines_above);
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
