type Pattern = Vec<Vec<char>>;

trait Transposable {
    fn transpose(&self) -> Self;
}

impl Transposable for Pattern {
    fn transpose(&self) -> Self {
        (0..self[0].len())
            .map(|x| (0..self.len()).map(|y| self[y][x]).collect::<Vec<char>>())
            .collect()
    }
}

trait PatternLineComparison {
    fn one_off(&self, other: &Self) -> bool;
}

impl PatternLineComparison for Vec<char> {
    fn one_off(&self, other: &Vec<char>) -> bool {
        let mut mismatch = 0;
        for i in 0..self.len() {
            if self[i] != other[i] {
                mismatch += 1;
            }
            if mismatch > 1 {
                return false;
            }
        }
        true
    }
}

fn lines_above_reflection_point(pattern: Pattern, repair_smudges: bool) -> i64 {
    let reflection_points = pattern
        .windows(2)
        .enumerate()
        .filter_map(|(index, window)| {
            if window[0].one_off(&window[1]) {
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
                    if pattern[low_scan as usize].one_off(&pattern[high_scan as usize])
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
        .filter(|(corrections, _)| {
            if repair_smudges {
                *corrections == 1
            } else {
                *corrections == 0
            }
        })
        .collect::<Vec<(usize, usize)>>();
    if reflection_points.len() == 0 {
        0
    } else if reflection_points.len() > 1 {
        panic!("Multiple reflection points found: {:?}", reflection_points);
    } else {
        reflection_points[0].1 as i64 + 1
    }
}

fn lines_to_the_left_of_reflection_point(pattern: Pattern, repair_smudges: bool) -> i64 {
    lines_above_reflection_point(pattern.transpose(), repair_smudges)
}

fn solve_part(repair_smudges: bool) -> i64 {
    let file = std::fs::read_to_string("input/day13").unwrap();
    let patterns = file
        .split("\n\n")
        .map(|line| {
            line.split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect()
        })
        .collect::<Vec<Pattern>>();
    let lines_to_the_left: i64 = patterns
        .iter()
        .map(|pattern| lines_to_the_left_of_reflection_point(pattern.to_vec(), repair_smudges))
        .sum();

    let lines_above: i64 = patterns
        .iter()
        .map(|pattern| lines_above_reflection_point(pattern.to_vec(), repair_smudges))
        .sum();
    return lines_to_the_left + (100 * lines_above);
}

pub fn solve() {
    println!("Part 1: {}", solve_part(false));
    println!("Part 2: {}", solve_part(true));
}
