use std::collections::HashMap;

#[derive(PartialEq)]
enum Operator {
    Set(i32),
    Delete,
}

struct Instruction {
    label: String,
    op: Operator,
    instruction_hash: i32,
    label_hash: i32,
}

#[derive(Debug)]
struct Box {
    contents: Vec<(String, i32)>,
}

impl Box {
    fn new() -> Box {
        Box {
            contents: Vec::new(),
        }
    }

    fn set(&mut self, label: String, amount: i32) {
        if self.contents.iter().any(|(l, _)| l == &label.to_string()) {
            self.contents = self
                .contents
                .iter()
                .map(|(l, a)| {
                    if l == &label {
                        (l.to_string(), amount)
                    } else {
                        (l.to_string(), *a)
                    }
                })
                .collect();
        } else {
            self.contents.push((label, amount));
        }
    }

    fn delete(&mut self, label: &str) {
        self.contents = self
            .contents
            .clone()
            .iter()
            .filter(|(l, _)| l != label)
            .map(|(l, a)| (l.to_string(), *a))
            .collect();
    }
}

fn hash_algorithm(input_string: &str) -> i32 {
    let mut hash = 0;
    input_string.chars().for_each(|c| {
        hash += c as i32;
        hash = (hash * 17) % 256;
    });
    return hash;
}

fn parse_instructions() -> Vec<Instruction> {
    let file = std::fs::read_to_string("input/day15").unwrap();
    file.strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|step| {
            let separator = if step.contains("=") { '=' } else { '-' };
            let label = step.split(separator).collect::<Vec<&str>>()[0].to_string();
            Instruction {
                label: label.clone(),
                op: if separator == '=' {
                    Operator::Set(
                        step.split(separator).collect::<Vec<&str>>()[1]
                            .parse()
                            .unwrap(),
                    )
                } else {
                    Operator::Delete
                },
                label_hash: hash_algorithm(&label),
                instruction_hash: hash_algorithm(step),
            }
        })
        .collect()
}

fn solve_part_one() -> i32 {
    parse_instructions()
        .iter()
        .map(|instruction| instruction.instruction_hash)
        .sum()
}

fn solve_part_two() -> i32 {
    let mut boxes: HashMap<i32, Box> = HashMap::new();
    parse_instructions()
        .iter()
        .for_each(|instruction| match instruction.op {
            Operator::Set(value) => {
                if !boxes.contains_key(&instruction.label_hash) {
                    boxes.insert(instruction.label_hash, Box::new());
                }
                boxes
                    .get_mut(&instruction.label_hash)
                    .unwrap()
                    .set(instruction.label.clone(), value);
            }
            Operator::Delete => {
                if boxes.contains_key(&instruction.label_hash) {
                    boxes
                        .get_mut(&instruction.label_hash)
                        .unwrap()
                        .delete(&instruction.label);
                }
            }
        });
    boxes
        .iter()
        .map(|(hash, this_box)| {
            this_box
                .contents
                .iter()
                .enumerate()
                .map(|(index, (_, amount))| (hash + 1) * (index as i32 + 1) * amount)
                .sum::<i32>()
        })
        .sum()
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
