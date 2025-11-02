use eyre::{Result, eyre};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn north(&self) -> Coord {
        Coord::new(self.x, self.y + 1)
    }
    fn south(&self) -> Coord {
        Coord::new(self.x, self.y - 1)
    }
    fn east(&self) -> Coord {
        Coord::new(self.x + 1, self.y)
    }
    fn west(&self) -> Coord {
        Coord::new(self.x - 1, self.y)
    }
}

fn generate_hive_name(coord: Coord) -> String {
    format!("Hive_{:+04}_{:+04}", coord.x, coord.y)
}

fn generate_spiral_coords(num_nodes: usize) -> Vec<Coord> {
    let mut coords = Vec::with_capacity(num_nodes);
    let origin = Coord::new(0, 0);
    coords.push(origin);

    if num_nodes == 1 {
        return coords;
    }

    let mut ring = 1;

    while coords.len() < num_nodes {
        // Generate all coordinates at Manhattan distance = ring from origin
        let mut ring_coords = Vec::new();

        if ring == 0 {
            ring_coords.push(origin);
        } else {
            // Start from (ring, 0) and go clockwise around the diamond
            // Right edge: (ring, 0) down to (0, ring)
            for i in 0..ring as i32 {
                let coord = Coord::new(ring as i32 - i, i);
                ring_coords.push(coord);
            }

            // Top edge: (0, ring) left to (-ring, 0)
            for i in 0..ring as i32 {
                let coord = Coord::new(-i, ring as i32 - i);
                ring_coords.push(coord);
            }

            // Left edge: (-ring, 0) down to (0, -ring)
            for i in 0..ring as i32 {
                let coord = Coord::new(-(ring as i32 - i), -i);
                ring_coords.push(coord);
            }

            // Bottom edge: (0, -ring) right to (ring, 0)
            for i in 0..ring as i32 {
                let coord = Coord::new(i, -(ring as i32 - i));
                ring_coords.push(coord);
            }
        }

        // Add coordinates from this ring
        for coord in ring_coords {
            if coords.len() < num_nodes {
                coords.push(coord);
            } else {
                break;
            }
        }

        ring += 1;
    }

    coords.truncate(num_nodes);
    coords
}

fn generate_map(num_nodes: usize, output_file: &str) -> Result<()> {
    println!(
        "Generating spiral grid map with {} nodes to {}",
        num_nodes, output_file
    );

    // Generate spiral coordinates
    let coords = generate_spiral_coords(num_nodes);
    let coord_to_index: HashMap<Coord, usize> = coords
        .iter()
        .enumerate()
        .map(|(i, &coord)| (coord, i))
        .collect();

    let mut file = File::create(output_file)?;

    for (i, &coord) in coords.iter().enumerate() {
        let hive_name = generate_hive_name(coord);
        let mut connections = Vec::new();

        // Check all four directions for valid neighbors
        let neighbors = [
            ("north", coord.north()),
            ("west", coord.west()),
            ("south", coord.south()),
            ("east", coord.east()),
        ];

        for (direction, neighbor_coord) in neighbors {
            if let Some(&_neighbor_index) = coord_to_index.get(&neighbor_coord) {
                let neighbor_name = generate_hive_name(neighbor_coord);
                connections.push(format!("{}={}", direction, neighbor_name));
            }
        }

        // Write the hive line
        writeln!(file, "{} {}", hive_name, connections.join(" "))?;

        // Print progress for large maps
        if num_nodes > 10000 && i % (num_nodes / 100) == 0 {
            println!("Progress: {}%", (i * 100) / num_nodes);
        }
    }

    println!("Map generation complete!");
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err(eyre!("Usage: {} <num_nodes> <output_file>", args[0]));
    }

    let num_nodes: usize = args[1]
        .parse()
        .map_err(|_| eyre!("Invalid number of nodes: '{}'", args[1]))?;

    let output_file = &args[2];

    if num_nodes == 0 {
        return Err(eyre!("Number of nodes must be greater than 0"));
    }

    println!("Generating {} nodes...", num_nodes);
    let start_time = std::time::Instant::now();

    generate_map(num_nodes, output_file)?;

    let duration = start_time.elapsed();
    println!("Generation took: {:?}", duration);
    println!("Generated {} nodes in {}", num_nodes, output_file);

    Ok(())
}
