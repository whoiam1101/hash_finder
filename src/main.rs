use clap::{arg, command, Parser};
use rayon::prelude::*;
use sha2::{Sha256, Digest};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(name = "hash_finder")]
#[command(version = "1.0")]
#[command(about = "Finds numbers whose SHA-256 hash ends with N zeros", long_about = None)]
struct Cli {
    #[arg(short = 'N', long = "number-of-zeros")]
    zeros: usize,
    
    #[arg(short = 'F', long = "number-of-results")]
    results: usize,
}

fn calculate_sha256(number: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(number.to_string().as_bytes());
    let result = hasher.finalize();
    format!("{:064x}", result)
}

fn main() {
    let args = Cli::parse();

    let n_zeros = args.zeros;
    let n_results = args.results;

    let target_suffix = &"0".repeat(n_zeros);
    let counter = Arc::new(AtomicUsize::new(1));
    let found_counter = Arc::new(AtomicUsize::new(0));

    rayon::scope(|s| {
        for _ in 0..rayon::current_num_threads() {
            let counter = Arc::clone(&counter);
            let found_counter = Arc::clone(&found_counter);
            s.spawn(move |_| {
                while found_counter.load(Ordering::Relaxed) < n_results {
                    let num = counter.fetch_add(1, Ordering::SeqCst);
                    let hash = calculate_sha256(num);
                    if hash.ends_with(target_suffix) {
                        println!("{}, \"{}\"", num, hash);
                        found_counter.fetch_add(1, Ordering::SeqCst);
                    }
                }
            });
        }
    });
}
