type Point = (usize, usize);

fn parse_map() -> Vec<Vec<char>> {
    let file = std::fs::read_to_string("input/day10").unwrap();
    file.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect()
}

fn find_starting_point(map: &Vec<Vec<char>>) -> Point {
    map.into_iter()
        .enumerate()
        .find_map(|(line_num, s)| match s.iter().position(|c| *c == 'S') {
            Some(pos) => {
                return Some((line_num, pos));
            }
            None => return None,
        })
        .unwrap()
}

fn find_available_points(map: &Vec<Vec<char>>, at: Point) -> Vec<Point> {
    let x = at.1 as i32;
    let y = at.0 as i32;
    let available_points: Vec<(i32, i32)> = match map[at.0][at.1] {
        'S' => [].to_vec(),
        '|' => [(y + 1, x), (y - 1, x)].to_vec(),
        '-' => [(y, x + 1), (y, x - 1)].to_vec(),
        'J' => [(y - 1, x), (y, x - 1)].to_vec(),
        '7' => [(y + 1, x), (y, x - 1)].to_vec(),
        'F' => [(y + 1, x), (y, x + 1)].to_vec(),
        'L' => [(y - 1, x), (y, x + 1)].to_vec(),
        '.' => [].to_vec(),
        _ => panic!("Invalid character {}", map[at.0][at.1]),
    };
    available_points
        .into_iter()
        .filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < map.len() as i32 && p.1 < map[0].len() as i32)
        .map(|p| (p.0 as usize, p.1 as usize))
        .collect()
}

fn find_start_pipe_type(pipe_map: &Vec<Vec<char>>, start: Point) -> char {
    let xs = start.0 as i32;
    let ys = start.1 as i32;
    let available_points = [
        (xs as i32 + 1, ys as i32),
        (xs as i32 - 1, ys as i32),
        (xs as i32, ys as i32 + 1),
        (xs as i32, ys as i32 - 1),
    ]
    .into_iter()
    .filter(|p| {
        p.0 >= 0
            && p.1 >= 0
            && p.0 < pipe_map.len() as i32
            && p.1 < pipe_map[0].len() as i32
            && find_available_points(pipe_map, (p.0 as usize, p.1 as usize)).contains(&start)
    })
    .collect::<Vec<(i32, i32)>>();

    match available_points.as_slice() {
        [first, second] if first == &(xs + 1, ys) && second == &(xs - 1, ys) => '|',
        [first, second] if first == &(xs, ys + 1) && second == &(xs, ys - 1) => '-',
        [first, second] if first == &(xs - 1, ys) && second == &(xs, ys - 1) => 'J',
        [first, second] if first == &(xs + 1, ys) && second == &(xs, ys + 1) => 'F',
        [first, second] if first == &(xs - 1, ys) && second == &(xs, ys + 1) => 'L',
        [first, second] if first == &(xs + 1, ys) && second == &(xs, ys - 1) => '7',
        _ => panic!("Invalid character {}", pipe_map[start.0][start.1]),
    }
}

fn find_loop(pipe_map: &Vec<Vec<char>>, start: Point) -> Vec<Point> {
    let mut paths: Vec<Vec<Point>> = Vec::from([[start].to_vec()]);
    loop {
        let this_path = paths[0].clone();
        let at = this_path[this_path.len() - 1];
        let last = if paths.len() == 1 {
            at
        } else {
            this_path[this_path.len() - 2]
        };
        let available_points = find_available_points(pipe_map, at)
            .into_iter()
            .filter(|p| *p != last)
            .collect::<Vec<Point>>();
        paths.remove(0);
        for available_point in available_points {
            if available_point == start {
                return this_path;
            }
            if this_path.contains(&available_point) {
                continue;
            }
            let mut new_path = this_path.clone();
            new_path.push(available_point);
            paths.push(new_path);
        }
    }
}

fn solve_part_one() -> i32 {
    let mut pipe_map = parse_map();
    let start = find_starting_point(&pipe_map);
    pipe_map[start.0][start.1] = find_start_pipe_type(&pipe_map, start);
    let pipe_loop = find_loop(&pipe_map, start);
    return (pipe_loop.len() / 2) as i32;
}

fn solve_part_two() -> i32 {
    let mut pipe_map = parse_map();
    let start = find_starting_point(&pipe_map);
    pipe_map[start.0][start.1] = find_start_pipe_type(&pipe_map, start);
    let pipe_loop = find_loop(&pipe_map, start);
    let mut enclosed = Vec::new();

    pipe_map.into_iter().enumerate().for_each(|(y, row)| {
        let mut in_the_loop = false;
        let mut switch_pipes = ['|', 'L', 'F'].to_vec();
        row.into_iter().enumerate().for_each(|(x, char)| {
            if pipe_loop.contains(&(y, x)) && switch_pipes.contains(&char) {
                in_the_loop = !in_the_loop;
                switch_pipes = match char {
                    c if c == 'F' => ['F', '7', '|'].to_vec(),
                    c if c == 'L' => ['L', 'J', '|'].to_vec(),
                    c if ['|', 'J', '7'].contains(&c) => ['|', 'L', 'F'].to_vec(),
                    _ => panic!("Invalid character {}", char),
                }
            } else if !pipe_loop.contains(&(y, x)) && in_the_loop {
                enclosed.push((y, x));
            }
        })
    });
    return enclosed.len() as i32;
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
