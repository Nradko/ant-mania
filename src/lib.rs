use eyre::{Result, eyre};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type HiveIndex = usize;

const MOVES: u32 = 10000;

#[derive(Debug)]
pub struct Node {
    pub neighbor_count: i8,          // -1 -> destroyed
    pub neighbors: [isize; 4],       // North, West, South, East, -1 -> no neighbor
    pub valid_neighbors: Vec<usize>, // Precomputed list of valid neighbors for faster access
    pub present_ant: isize,          // -1 -> no ant, otherwise ant ID
}

pub struct Map {
    pub index_to_name: Vec<String>,
    pub name_to_index: HashMap<String, HiveIndex>,
    pub graph: Vec<Node>,
}

#[derive(Debug)]
pub struct DestructionEvent {
    pub hive_index: usize,
    pub ant1: isize,
    pub ant2: isize,
}

use std::collections::HashSet;

pub struct Ants {
    pub nodes_with_ants: Vec<usize>,
    pub nodes_with_ants_set: HashSet<usize>, // For O(1) lookups and removals
    pub all_ants: usize,
    pub finished_or_dead_ants: usize,
    pub ant_to_moves: Vec<u32>,
}

pub fn simulate(map: &mut Map, ants: &mut Ants) -> Vec<DestructionEvent> {
    let mut destruction_events = Vec::with_capacity(ants.all_ants / 2 + 1);
    while ants.finished_or_dead_ants < ants.all_ants {
        let choose_node_with_ant = fastrand::usize(..ants.nodes_with_ants.len());

        let node_index = ants.nodes_with_ants[choose_node_with_ant];
        let node = map.graph.get_mut(node_index).unwrap();

        if node.neighbor_count == -1 {
            // destroyed node -> no need to revisit
            ants.nodes_with_ants.swap_remove(choose_node_with_ant);
            continue;
        }

        if node.neighbor_count == 0 {
            // no where to go -> no need to revisit
            ants.nodes_with_ants.swap_remove(choose_node_with_ant);

            if ants.ant_to_moves[node.present_ant as usize] < MOVES {
                ants.finished_or_dead_ants += 1;
            };
            continue;
        }

        // choose random neighbor using precomputed valid neighbors
        if node.valid_neighbors.is_empty() {
            // Node has no neighbors, ant is stuck
            ants.nodes_with_ants.swap_remove(choose_node_with_ant);
            if ants.ant_to_moves[node.present_ant as usize] < MOVES {
                ants.finished_or_dead_ants += 1;
            }
            continue;
        }

        let neighbor_idx = fastrand::usize(0..node.valid_neighbors.len());
        let target_index = node.valid_neighbors[neighbor_idx];
        let ant_index = node.present_ant;
        node.present_ant = -1;

        // drop previous node borrow
        let node = map.graph.get_mut(target_index).unwrap();

        ants.nodes_with_ants[choose_node_with_ant] = target_index;
        if node.present_ant == -1 {
            // move ant to target node
            node.present_ant = ant_index;
            ants.ant_to_moves[ant_index as usize] += 1;

            if ants.ant_to_moves[ant_index as usize] == MOVES {
                ants.finished_or_dead_ants += 1;
            }
        } else {
            // fight -> both ants die, node destroyed
            if ants.ant_to_moves[node.present_ant as usize] < MOVES {
                ants.finished_or_dead_ants += 1;
            }
            if ants.ant_to_moves[ant_index as usize] < MOVES {
                ants.finished_or_dead_ants += 1;
            }

            destruction_events.push(DestructionEvent {
                hive_index: target_index,
                ant1: ant_index,
                ant2: node.present_ant,
            });

            node.present_ant = -1;
            node.neighbor_count = -1;
            let neighbors = node.neighbors;

            ants.nodes_with_ants.swap_remove(choose_node_with_ant);

            for &neighbor in neighbors.iter() {
                if neighbor != -1 {
                    let node = &mut map.graph[neighbor as usize];
                    for n_index in 0..4 {
                        if node.neighbors[n_index] == target_index as isize {
                            node.neighbors[n_index] = -1;
                            node.neighbor_count -= 1;
                            break;
                        }
                    }
                }
            }
        }
    }

    destruction_events
}

/// Returns a index to hive name (Vec) and name to index (HashMap)
pub fn load_and_index_hives(file_path: &str) -> Result<(Vec<String>, HashMap<String, HiveIndex>)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut index_to_name = Vec::new();
    let mut name_to_index = HashMap::new();
    let mut current_index = 0usize;

    for line in reader.lines() {
        let line = line?;
        if let Some(space_pos) = line.find(' ') {
            let hive_name = line[..space_pos].to_string();

            if !name_to_index.contains_key(&hive_name) {
                name_to_index.insert(hive_name.clone(), current_index);
                index_to_name.push(hive_name);
                current_index += 1;
            } else {
                panic!("Duplicate hive name found: {}", hive_name);
            }
        }
    }

    Ok((index_to_name, name_to_index))
}

pub fn load_map(file_path: &str) -> Result<Map> {
    let (index_to_name, name_to_index) = load_and_index_hives(file_path)?;

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut graph = Vec::with_capacity(index_to_name.len());
    for _ in 0..index_to_name.len() {
        graph.push(Node {
            neighbor_count: 0,
            neighbors: [-1; 4],
            valid_neighbors: Vec::new(),
            present_ant: -1,
        });
    }

    for line in reader.lines() {
        let line = line?;
        if let Some(space_pos) = line.find(' ') {
            let hive_name = &line[..space_pos];
            let connections = &line[space_pos + 1..];

            let hive_index = name_to_index.get(hive_name).unwrap();
            let node = &mut graph[*hive_index as usize];

            if node.neighbors != [-1; 4] {
                panic!("Duplicate hive entry found for hive: {}", hive_name);
            }

            for connection in connections.split(' ') {
                if let Some(eq_pos) = connection.find('=') {
                    let direction = &connection[..eq_pos];
                    let target_hive = &connection[eq_pos + 1..];

                    if let Some(&target_index) = name_to_index.get(target_hive) {
                        match direction {
                            "north" => {
                                node.neighbors[0] = target_index as isize;
                                node.neighbor_count += 1;
                                node.valid_neighbors.push(target_index);
                            }
                            "west" => {
                                node.neighbors[1] = target_index as isize;
                                node.neighbor_count += 1;
                                node.valid_neighbors.push(target_index);
                            }
                            "south" => {
                                node.neighbors[2] = target_index as isize;
                                node.neighbor_count += 1;
                                node.valid_neighbors.push(target_index);
                            }
                            "east" => {
                                node.neighbors[3] = target_index as isize;
                                node.neighbor_count += 1;
                                node.valid_neighbors.push(target_index);
                            }
                            _ => {} // Ignore unknown directions
                        }
                    }
                }
            }
        }
    }

    Ok(Map {
        index_to_name,
        name_to_index,
        graph,
    })
}

pub fn place_ants(map: &mut Map, num_ants: usize) -> Result<Ants> {
    let mut valid_indices: Vec<usize> = (0..map.graph.len()).collect();

    if num_ants > valid_indices.len() {
        return Err(eyre!(
            "Number of ants ({}) exceeds number of available hives ({})",
            num_ants,
            valid_indices.len()
        ));
    }

    // Shuffle and select N random indices
    fastrand::shuffle(&mut valid_indices);

    let placed_indices = valid_indices
        .iter()
        .take(num_ants)
        .cloned()
        .collect::<Vec<usize>>();

    for i in 0..placed_indices.len() {
        let index = placed_indices[i];
        map.graph[index].present_ant = i as isize;
    }

    let nodes_set: HashSet<usize> = placed_indices.iter().copied().collect();

    Ok(Ants {
        nodes_with_ants: placed_indices,
        nodes_with_ants_set: nodes_set,
        all_ants: num_ants,
        finished_or_dead_ants: 0,
        ant_to_moves: vec![0; num_ants],
    })
}
