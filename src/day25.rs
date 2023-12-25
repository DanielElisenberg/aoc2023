use std::collections::{HashMap, HashSet};

fn parse_graph() -> HashMap<String, HashSet<String>> {
    let file = std::fs::read_to_string("input/day25").unwrap();
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    file.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let (from, to_list) = line.split_once(": ").unwrap();
            to_list.split(" ").for_each(|to| {
                if graph.contains_key(from) {
                    graph.get_mut(from).unwrap().insert(to.to_string());
                } else {
                    graph.insert(from.to_string(), HashSet::from([to.to_string()]));
                }
                if graph.contains_key(to) {
                    graph.get_mut(to).unwrap().insert(from.to_string());
                } else {
                    graph.insert(to.to_string(), HashSet::from([from.to_string()]));
                }
            });
        });
    graph
}

fn walk_back(
    from: String,
    to: String,
    path: Vec<String>,
    distances: &HashMap<String, Vec<(String, i32)>>,
) -> Vec<String> {
    if from == to {
        return path;
    }
    let mut choices = distances
        .get(&from)
        .unwrap()
        .iter()
        .filter(|(to, _)| !path.contains(to))
        .map(|(to, dist)| (to.clone(), *dist))
        .collect::<Vec<(String, i32)>>();
    if choices.len() == 0 {
        panic!("No choices from {} to {}", from, to);
    }
    choices.sort_by(|a, b| a.1.cmp(&b.1));
    let mut new_path = path.clone();
    new_path.push(choices[0].0.clone());
    walk_back(choices[0].0.clone(), to, new_path, distances)
}

fn map_distances(
    from: String,
    graph: &HashMap<String, HashSet<String>>,
) -> HashMap<String, Vec<(String, i32)>> {
    let mut distances: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    let mut search_from = Vec::from([(from.clone(), 0)]);
    let mut visited = HashMap::from([(from.clone(), 0)]);
    while search_from.len() > 0 {
        search_from.sort_by(|a, b| a.1.cmp(&b.1));
        let (from, dist) = search_from.remove(0);
        if distances.contains_key(&from) {
            continue;
        }
        let next = graph
            .get(&from)
            .unwrap()
            .into_iter()
            .map(|to| {
                if !visited.contains_key(to) {
                    visited.insert(to.clone(), dist + 1);
                    (to.clone(), dist + 1)
                } else {
                    (to.clone(), *visited.get(to).unwrap())
                }
            })
            .collect::<Vec<(String, i32)>>();
        search_from.extend(next.iter().map(|(to, dist)| (to.clone(), *dist)));
        distances.insert(from.clone(), next);
    }
    distances
}

fn find_cluster_size(graph: HashMap<String, HashSet<String>>, from: String) -> i32 {
    let mut visited = HashSet::from([from.clone()]);
    let mut search_from = Vec::from([from.clone()]);
    while search_from.len() > 0 {
        let current = search_from.remove(0);
        let next = graph
            .get(&current)
            .unwrap()
            .into_iter()
            .filter(|to| !visited.contains(*to))
            .map(|to| to.clone())
            .collect::<Vec<String>>();
        search_from.extend(next.clone());
        next.into_iter().for_each(|to| {
            visited.insert(to);
        })
    }
    return visited.len() as i32;
}
fn solve_part_one() -> i32 {
    let graph = parse_graph();
    let mut most_traveled: HashMap<Vec<String>, i32> = HashMap::new();
    graph.keys().for_each(|k| {
        let distances = map_distances(k.clone(), &graph);
        graph.keys().for_each(|k2| {
            if k == k2 {
                return;
            }
            walk_back(k2.clone(), k.clone(), vec![], &distances)
                .windows(2)
                .for_each(|w| {
                    let mut sorted_w = w.to_vec();
                    sorted_w.sort();
                    if most_traveled.contains_key(&sorted_w.clone()) {
                        most_traveled.insert(
                            sorted_w.clone(),
                            most_traveled.get(&sorted_w.clone()).unwrap() + 1,
                        );
                    } else {
                        most_traveled.insert(sorted_w.clone(), 1);
                    }
                });
        });
    });
    let mut most_traveled_ranked = most_traveled
        .iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect::<Vec<(Vec<String>, i32)>>();
    most_traveled_ranked.sort_by(|a, b| b.1.cmp(&a.1));
    let mut disconnect = HashMap::new();
    most_traveled_ranked
        .into_iter()
        .take(3)
        .for_each(|(connection, _)| {
            disconnect.insert(connection[0].clone(), connection[1].clone());
            disconnect.insert(connection[1].clone(), connection[0].clone());
        });
    let disconnected_graph = graph
        .keys()
        .map(|k| {
            if disconnect.contains_key(k) {
                (
                    k.clone(),
                    graph
                        .get(k)
                        .unwrap()
                        .clone()
                        .into_iter()
                        .filter(|to| to != disconnect.get(k).unwrap())
                        .collect::<HashSet<String>>(),
                )
            } else {
                return (k.clone(), graph.get(k).unwrap().clone());
            }
        })
        .collect::<HashMap<String, HashSet<String>>>();
    find_cluster_size(
        disconnected_graph.clone(),
        disconnect.keys().next().unwrap().clone(),
    ) * find_cluster_size(
        disconnected_graph.clone(),
        disconnect
            .get(disconnect.keys().next().unwrap())
            .unwrap()
            .clone(),
    )
}

pub fn solve() {
    println!("Part 1: {}", solve_part_one());
    println!("Part 2: Merry Christmas!");
}
