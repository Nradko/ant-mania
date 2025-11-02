use std::env;

use ant_mania::{load_map, place_ants, simulate};
use eyre::{Result, eyre};

fn main() -> Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(eyre!("Usage: {} <number_of_ants>", args[0]));
    }

    let num_ants: usize = args[1].parse().map_err(|_| {
        eyre!(
            "Invalid number of ants: '{}'. Please provide a valid positive integer.",
            args[1]
        )
    })?;

    if num_ants == 0 {
        return Err(eyre!("Number of ants must be greater than 0"));
    }

    // Load environment variables from .env file
    dotenv::dotenv()?;

    let map_file = env::var("MAP_FILE")
        .unwrap_or_else(|_| "description/Ant_Mania/hiveum_map_small.txt".to_string());

    let mut map = load_map(&map_file)?;
    let mut ants = place_ants(&mut map, num_ants)?;

    let destruction_events = simulate(&mut map, &mut ants);

    for event in &destruction_events {
        println!(
            "{} has been destroyed by ant {} and ant {}!",
            map.index_to_name[event.hive_index], event.ant1, event.ant2
        );
    }

    // Print remaining world in the same format as input
    for (i, node) in map.graph.iter().enumerate() {
        if node.neighbor_count >= 0 {
            // Only print non-destroyed nodes
            let hive_name = &map.index_to_name[i];
            print!("{}", hive_name);

            let directions = ["north", "west", "south", "east"];
            for (dir_idx, &neighbor) in node.neighbors.iter().enumerate() {
                if neighbor >= 0 {
                    let neighbor_name = &map.index_to_name[neighbor as usize];
                    print!(" {}={}", directions[dir_idx], neighbor_name);
                }
            }
            println!();
        }
    }

    Ok(())
}
