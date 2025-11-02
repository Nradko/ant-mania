use ant_mania::{load_map, place_ants, simulate};
use eyre::Result;
use rayon::prelude::*;
use std::env;
use std::time::Instant;

#[derive(Debug, Clone)]
struct BenchmarkResult {
    map_name: String,
    num_ants: usize,
    avg_time_ms: f64,
    min_time_ms: f64,
    max_time_ms: f64,
}

impl BenchmarkResult {
    fn new(map_name: String, num_ants: usize, run_times_ms: Vec<f64>) -> Self {
        let avg_time_ms = run_times_ms.iter().sum::<f64>() / run_times_ms.len() as f64;
        let min_time_ms = run_times_ms.iter().copied().fold(f64::INFINITY, f64::min);
        let max_time_ms = run_times_ms
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);

        Self {
            map_name,
            num_ants,
            avg_time_ms,
            min_time_ms,
            max_time_ms,
        }
    }
}

fn run_benchmark_set(map_file: &str, num_ants: usize, num_runs: usize) -> Result<BenchmarkResult> {
    let map_name = map_file.split('/').last().unwrap_or(map_file).to_string();
    let mut run_times = Vec::new();

    println!(
        "Running {} simulations with {} ants on {}...",
        num_runs, num_ants, map_name
    );

    for run in 0..num_runs {
        // Load fresh map for each run
        let mut map = load_map(&map_file)?;
        let mut ants = place_ants(&mut map, num_ants)?;

        let start_time = Instant::now();
        simulate(&mut map, &mut ants);
        let elapsed = start_time.elapsed();

        let time_ms = elapsed.as_secs_f64() * 1000.0;
        run_times.push(time_ms);

        if run % 2 == 0 {
            print!(".");
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }
    }
    println!(" Done!");

    Ok(BenchmarkResult::new(map_name, num_ants, run_times))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        eprintln!("Usage: {} (no arguments needed)", args[0]);
        std::process::exit(1);
    }

    println!("🐜 Ant Mania Benchmark Suite 🐜");
    println!("================================");

    let maps = vec![
        ("hiveum_map_large.txt", "Large Map (100K nodes)"),
        ("hiveum_map_very_large.txt", "Very Large Map (1M nodes)"),
    ];

    // Ant counts to test
    let ant_counts_large = vec![1, 10, 100, 1000, 10000, 100000];
    let ant_counts_very_large = vec![1, 10, 100, 1000, 10000, 100000, 1000000];

    let num_runs = 10;

    // Create a vector of all test configurations
    let mut test_configs = Vec::new();

    for (map_file, map_desc) in &maps {
        let ant_counts = if map_file.contains("very_large") {
            &ant_counts_very_large
        } else {
            &ant_counts_large
        };

        println!("\n🗺️  Testing {} 🗺️", map_desc);
        println!("{}", "─".repeat(50));

        for &num_ants in ant_counts {
            test_configs.push((map_file.to_string(), num_ants));
        }
    }

    let mut all_results: Vec<BenchmarkResult> = test_configs
        .par_iter()
        .filter_map(|(map_file, num_ants)| run_benchmark_set(map_file, *num_ants, num_runs).ok())
        .collect();

    // Sort results by map name and ant count for organized output
    all_results.sort_by(|a, b| {
        a.map_name
            .cmp(&b.map_name)
            .then(a.num_ants.cmp(&b.num_ants))
    });

    // Print results table
    println!("\n📊 BENCHMARK RESULTS 📊");
    println!("=======================");
    println!(
        "{:<25} {:<12} {:<12} {:<12} {:<12}",
        "Map", "Ants", "Avg (ms)", "Min (ms)", "Max (ms)"
    );
    println!("{}", "─".repeat(75));

    for result in &all_results {
        println!(
            "{:<25} {:<12} {:<12.2} {:<12.2} {:<12.2}",
            result.map_name,
            result.num_ants,
            result.avg_time_ms,
            result.min_time_ms,
            result.max_time_ms
        );
    }

    Ok(())
}
