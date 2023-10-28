use std::collections::HashMap;

use advent_of_code::{input_to_list, read_input, should_submit, submit};

const DAY: u8 = 12;
const YEAR: u16 = 2021;
#[derive(PartialEq)]
enum NodeType {
    Start,
    End,
    Big,
    Small
}

struct Node {
    name: String,
    neighbors: Vec<String>,
    node_type: NodeType
}

fn is_uppercase(input: &str) -> bool {
    input.chars().all(|c| c.is_ascii_uppercase())
}

fn parse_graph(input: &str) -> HashMap<String, Node> {
    let list: Vec<String> = input_to_list(input).unwrap();
    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in list {
        let names: Vec<String> = line.split('-').map(|n| n.to_string()).collect();

        let name1 = &names[0];
        let name2 = &names[1];

        if let Some(node) = nodes.get_mut(name1) {
            node.neighbors.push(name2.clone())
        } else {
            nodes.insert(name1.clone(), Node{
                name: name1.clone(),
                neighbors: vec![name2.clone()],
                node_type: match name1.as_str() {
                    "start" => NodeType::Start,
                    "end" => NodeType::End,
                    x if is_uppercase(x) => NodeType::Big,
                    _ => NodeType::Small
                },
            });
        }


        if let Some(node) = nodes.get_mut(name2) {
            node.neighbors.push(name1.clone())
        } else {
            nodes.insert(name2.clone(), Node{
                name: name2.clone(),
                neighbors: vec![name1.clone()],
                node_type: match name2.as_str() {
                    "start" => NodeType::Start,
                    "end" => NodeType::End,
                    x if is_uppercase(x) => NodeType::Big,
                    _ => NodeType::Small
                },
            });
        }
    }

    nodes
}

fn visit(nodes: &HashMap<String, Node>, visited_paths: &mut Vec<Vec<String>>, current_node_name: &str, current_path: &mut Vec<String>) {
    let current_node = nodes.get(current_node_name).unwrap();

    if current_node.node_type != NodeType::Big && current_path.contains(&current_node_name.to_owned()) {
        return
    }

    current_path.push(current_node.name.clone());


    if current_node.node_type == NodeType::End {
        visited_paths.push(current_path.clone());
        return
    }

    for neighbor_name in current_node.neighbors.iter() {
        visit(nodes, visited_paths, neighbor_name, &mut current_path.clone());
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let nodes = parse_graph(input);

    let mut paths: Vec<Vec<String>> = vec![];

    visit(&nodes, &mut paths, "start", &mut vec![]);

    Some(paths.len() as u32)
}

fn visit_part_two(nodes: &HashMap<String, Node>, visited_paths: &mut Vec<Vec<String>>, current_node_name: &str, current_path: &mut Vec<String>, visited_small_cave_twice: bool) {
    let current_node = nodes.get(current_node_name).unwrap();
    let mut visited_small_cave_twice= visited_small_cave_twice;

    if current_path.contains(&current_node_name.to_owned()) {
        match current_node.node_type {
            NodeType::Start => {
                return
            }
            NodeType::Small => {
                if visited_small_cave_twice {
                    return
                }
                visited_small_cave_twice = true

            }
            _ => {}

        }
    };

    current_path.push(current_node.name.clone());

    if current_node.node_type == NodeType::End {
        visited_paths.push(current_path.clone());
        return
    }

    for neighbor_name in current_node.neighbors.iter() {
        visit_part_two(nodes, visited_paths, neighbor_name, &mut current_path.clone(), visited_small_cave_twice);
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let nodes = parse_graph(input);

    let mut paths: Vec<Vec<String>> = vec![];

    visit_part_two(&nodes, &mut paths, "start", &mut vec![], false);

    Some(paths.len() as u32)
}

fn main() {
    let input = read_input(DAY, YEAR);
    let part_one_response = part_one(&input);

    if let Some(part_one_response) = part_one_response {
        if should_submit() {
            println!("Submitting part 1 response: {part_one_response}");
            match submit(DAY, YEAR, 1, &part_one_response.to_string()) {
                Ok(correct) => {
                    if correct {
                        println!("Part 1 valid");
                    } else {
                        eprintln!("Part 1 invalid");
                    }
                }
                Err(e) => {
                    eprintln!("Error while submitting part1: {e}");
                }
            }
        } else {
            println!("Part 1 response: {part_one_response}")
        }
    } else {
        println!("No part 1 response");
    }

    let part_two_response = part_two(&input);
    if let Some(part_two_response) = part_two_response {
        if should_submit() {
            println!("Submitting part 2 response: {part_two_response}");
            match submit(DAY, YEAR, 2, &part_two_response.to_string()) {
                Ok(correct) => {
                    if correct {
                        println!("Part 2 valid");
                    } else {
                        eprintln!("Part 2 invalid");
                    }
                }
                Err(e) => {
                    eprintln!("Error while submitting part 2: {e}");
                }
            }
        } else {
            println!("Part 2 response: {part_two_response}")
        }
    } else {
        println!("No part 2 response");
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::read_example;

    use super::{DAY, part_one, part_two, YEAR};

    #[test]
    fn test_part_one() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_one(&example), Some(19));
    }

    #[test]
    fn test_part_two() {
        let example = read_example(DAY, YEAR);
        assert_eq!(part_two(&example), Some(103));
    }
}
