use std::collections::HashMap;

#[derive(Debug)]
enum Comparison {
    GreaterThan,
    LessThan,
}

#[derive(Debug)]
enum WorkflowStep {
    Conditional(String, Comparison, i32, String),
    Forward(String),
    Accept,
    Reject,
}

enum WorkflowStepResult {
    Accept,
    Reject,
}

#[derive(Clone, Copy, Debug)]
struct PartRange {
    x: (i32, i32),
    m: (i32, i32),
    a: (i32, i32),
    s: (i32, i32),
}

impl PartRange {
    fn distinct_values(&self) -> i64 {
        (self.x.1 - self.x.0 + 1) as i64
            * (self.m.1 - self.m.0 + 1) as i64
            * (self.a.1 - self.a.0 + 1) as i64
            * (self.s.1 - self.s.0 + 1) as i64
    }

    fn get(&self, field: &str) -> (i32, i32) {
        match field {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("Invalid field: {}", field),
        }
    }
    fn set(&mut self, field: &str, value: (i32, i32)) {
        match field {
            "x" => self.x = value,
            "m" => self.m = value,
            "a" => self.a = value,
            "s" => self.s = value,
            _ => panic!("Invalid field: {}", field),
        }
    }
}

fn parse_workflows() -> HashMap<String, Vec<WorkflowStep>> {
    let file = std::fs::read_to_string("input/day19").unwrap();
    file.split("\n\n")
        .nth(0)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let workflow_name = l.split("{").nth(0).unwrap();
            let workflow_steps = l
                .split("{")
                .nth(1)
                .unwrap()
                .split("}")
                .nth(0)
                .unwrap()
                .split(",")
                .map(|step| {
                    if step.contains(":") {
                        let conditional = step.split(":").nth(0).unwrap();
                        let forward = step.split(":").nth(1).unwrap();
                        WorkflowStep::Conditional(
                            conditional.chars().nth(0).iter().collect(),
                            if conditional.contains(">") {
                                Comparison::GreaterThan
                            } else {
                                Comparison::LessThan
                            },
                            conditional
                                .chars()
                                .skip(2)
                                .collect::<String>()
                                .parse()
                                .unwrap(),
                            forward.to_string(),
                        )
                    } else {
                        match step {
                            "A" => WorkflowStep::Accept,
                            "R" => WorkflowStep::Reject,
                            _ => WorkflowStep::Forward(step.to_string()),
                        }
                    }
                })
                .collect::<Vec<WorkflowStep>>();
            (workflow_name.to_string(), workflow_steps)
        })
        .collect()
}

fn parse_parts() -> Vec<HashMap<String, i32>> {
    let file = std::fs::read_to_string("input/day19").unwrap();
    file.split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.chars()
                .skip(1)
                .take(l.len() - 2)
                .collect::<String>()
                .split(",")
                .map(|s| {
                    (
                        s.split("=").nth(0).unwrap().to_string(),
                        s.split("=").nth(1).unwrap().parse().unwrap(),
                    )
                })
                .collect::<HashMap<String, i32>>()
        })
        .collect()
}

fn handle_part(
    workflows: &HashMap<String, Vec<WorkflowStep>>,
    part: &HashMap<String, i32>,
) -> WorkflowStepResult {
    let mut next = "in";
    loop {
        match next {
            "A" => return WorkflowStepResult::Accept,
            "R" => return WorkflowStepResult::Reject,
            _ => {}
        };
        for step in &workflows[next] {
            match step {
                WorkflowStep::Accept => {
                    return WorkflowStepResult::Accept;
                }
                WorkflowStep::Reject => {
                    return WorkflowStepResult::Reject;
                }
                WorkflowStep::Forward(to) => {
                    next = &to;
                    break;
                }
                WorkflowStep::Conditional(field, Comparison::GreaterThan, number, to) => {
                    if part[field] > *number {
                        next = &to;
                        break;
                    }
                }
                WorkflowStep::Conditional(field, Comparison::LessThan, number, to) => {
                    if part[field] < *number {
                        next = &to;
                        break;
                    }
                }
            }
        }
    }
}
fn find_distinct_part_combinations(
    workflows: &HashMap<String, Vec<WorkflowStep>>,
    part_range: &mut PartRange,
    workflow_name: &str,
) -> i64 {
    match workflow_name {
        "A" => {
            return part_range.distinct_values();
        }
        "R" => return 0,
        _ => {}
    };
    let mut total = 0;
    for step in &workflows[workflow_name] {
        match step {
            WorkflowStep::Accept => {
                return part_range.distinct_values() + total;
            }
            WorkflowStep::Reject => {
                return 0 + total;
            }
            WorkflowStep::Forward(to) => {
                total += find_distinct_part_combinations(workflows, &mut part_range.clone(), to);
            }
            WorkflowStep::Conditional(field, Comparison::GreaterThan, number, to) => {
                if part_range.get(field).1 <= *number {
                    continue;
                }
                let mut new_part_range = part_range.clone();
                new_part_range.set(field, (*number + 1, part_range.get(field).1));
                total += find_distinct_part_combinations(workflows, &mut new_part_range, to);
                part_range.set(field, (part_range.get(field).0, *number));
            }
            WorkflowStep::Conditional(field, Comparison::LessThan, number, to) => {
                if part_range.get(field).0 >= *number {
                    continue;
                }
                let mut new_part_range = part_range.clone();
                new_part_range.set(field, (part_range.get(field).0, *number - 1));
                total += find_distinct_part_combinations(workflows, &mut new_part_range, to);
                part_range.set(field, (*number, part_range.get(field).1));
            }
        }
    }
    total
}

fn solve_part_one() -> i64 {
    let parts = parse_parts();
    let workflows = parse_workflows();
    parts
        .iter()
        .filter_map(|part| match handle_part(&workflows, part) {
            WorkflowStepResult::Accept => Some(part),
            WorkflowStepResult::Reject => None,
        })
        .map(|part| part.values().map(|v| *v).sum::<i32>())
        .sum::<i32>() as i64
}

fn solve_part_two() -> i64 {
    let workflows = parse_workflows();
    let mut part_range = PartRange {
        x: (1, 4000),
        m: (1, 4000),
        a: (1, 4000),
        s: (1, 4000),
    };
    find_distinct_part_combinations(&workflows, &mut part_range, "in")
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
