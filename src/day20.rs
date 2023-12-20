use super::day08::lowest_common_multiple;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Broadcaster {
    outputs: Vec<String>,
}

#[derive(Debug, Clone)]
struct FlipFlop {
    state: bool,
    outputs: Vec<String>,
}

#[derive(Debug, Clone)]
struct Conjuction {
    inputs: Vec<(String, bool)>,
    outputs: Vec<String>,
}

#[derive(Debug, Clone)]
enum Module {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjuction(Conjuction),
}

fn parse_modules() -> HashMap<String, Module> {
    let file = std::fs::read_to_string("input/day20").unwrap();
    let modules: HashMap<String, Module> = file
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let name_string = l.split(" -> ").nth(0).unwrap();
            let outputs = l
                .split(" -> ")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|s| s.trim().to_string());
            match name_string {
                name if name.starts_with("%") => (
                    name.chars().skip(1).collect::<String>(),
                    Module::FlipFlop(FlipFlop {
                        state: false,
                        outputs: outputs.collect(),
                    }),
                ),
                name if name.starts_with("&") => (
                    name.chars().skip(1).collect::<String>(),
                    Module::Conjuction(Conjuction {
                        inputs: Vec::new(),
                        outputs: outputs.collect(),
                    }),
                ),
                "broadcaster" => (
                    "broadcaster".to_string(),
                    Module::Broadcaster(Broadcaster {
                        outputs: outputs.collect(),
                    }),
                ),
                _ => panic!("Unknown module: {}", name_string),
            }
        })
        .collect();

    let mut inputs = HashMap::new();
    modules.iter().for_each(|(name, module)| {
        let outputs = match module {
            Module::Broadcaster(broadcaster) => broadcaster.outputs.clone(),
            Module::FlipFlop(flipflop) => flipflop.outputs.clone(),
            Module::Conjuction(conjuction) => conjuction.outputs.clone(),
        };
        outputs.clone().into_iter().for_each(|s| {
            if !inputs.contains_key(&s) {
                inputs.insert(s, vec![name]);
            } else {
                inputs.get_mut(&s).unwrap().extend([name]);
            }
        });
    });
    modules
        .iter()
        .map(|(name, module)| match module {
            Module::Conjuction(conjuction) => (
                name.clone(),
                Module::Conjuction(Conjuction {
                    outputs: conjuction.outputs.clone(),
                    inputs: inputs[name]
                        .clone()
                        .into_iter()
                        .map(|s| (s.clone(), false))
                        .collect(),
                }),
            ),
            _ => (name.clone(), module.clone()),
        })
        .collect()
}

fn handle_signal(
    module: &mut Module,
    from: &String,
    signal: bool,
    current: &String,
) -> Vec<(String, bool, String)> {
    match module {
        Module::Broadcaster(broadcaster) => broadcaster
            .outputs
            .clone()
            .into_iter()
            .map(|output_module| (current.clone(), signal.clone(), output_module))
            .collect(),
        Module::FlipFlop(flipflop) => {
            if !signal {
                flipflop.state = !flipflop.state;
                flipflop
                    .outputs
                    .clone()
                    .into_iter()
                    .map(|output_module| (current.clone(), flipflop.state, output_module))
                    .collect()
            } else {
                vec![]
            }
        }
        Module::Conjuction(conjuction) => {
            conjuction.inputs = conjuction
                .inputs
                .clone()
                .into_iter()
                .map(|(input_module, memory)| {
                    if input_module == *from {
                        (input_module, signal)
                    } else {
                        (input_module, memory)
                    }
                })
                .collect();
            let output_signal = !conjuction.inputs.iter().all(|(_, memory)| *memory);
            conjuction
                .outputs
                .clone()
                .into_iter()
                .map(|output_module| (current.clone(), output_signal, output_module))
                .collect()
        }
    }
}

fn solve_part_one() -> i64 {
    let mut modules = parse_modules();
    let mut low_signal_count = 0;
    let mut high_signal_count = 0;
    (0..1000).for_each(|_| {
        let mut signals = vec![("button".to_string(), false, "broadcaster".to_string())];
        low_signal_count += 1;
        while signals.len() > 0 {
            let mut new_signals = Vec::new();
            for (from, signal, to) in signals {
                match modules.get_mut(&to) {
                    Some(m) => new_signals.extend(handle_signal(m, &from, signal, &to)),
                    None => continue,
                };
            }
            high_signal_count += new_signals
                .clone()
                .into_iter()
                .filter(|(_, b, _)| *b)
                .count();
            low_signal_count += new_signals
                .clone()
                .into_iter()
                .filter(|(_, b, _)| !*b)
                .count();
            signals = new_signals;
        }
    });
    low_signal_count as i64 * high_signal_count as i64
}

fn solve_part_two() -> i64 {
    let mut modules = parse_modules();
    let mut button_presses = 0;
    let mut component_loops = vec![];
    while component_loops.len() < 4 {
        let mut signals = vec![("button".to_string(), false, "broadcaster".to_string())];
        button_presses += 1;
        while signals.len() > 0 {
            let mut new_signals = Vec::new();
            for (from, signal, to) in signals {
                if ["vt", "sk", "xc", "kk"].contains(&from.as_str()) && signal {
                    component_loops.push(button_presses);
                }
                if component_loops.len() == 4 {
                    new_signals = vec![];
                    break;
                }
                match modules.get_mut(&to) {
                    Some(m) => new_signals.extend(handle_signal(m, &from, signal, &to)),
                    None => continue,
                };
            }
            signals = new_signals;
        }
    }
    lowest_common_multiple(component_loops)
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: {}", solve_part_two());
}
