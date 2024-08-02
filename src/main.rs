use simple_logger;
use miniception_rust::{minimizers_locations, minimizers_locations_mono_queue};
use rand::thread_rng;
use rand::distributions::{Uniform, Distribution};  // Add Distribution to the imports
use std::time::Instant;

fn random_dna_sequence(length: usize) -> String {
    let mut rng = thread_rng();
    let nucleotides = ['A', 'C', 'G', 'T'];
    let dist = Uniform::from(0..nucleotides.len());
    let seq: String = (0..length)
        .map(|_| nucleotides[dist.sample(&mut rng)])
        .collect();
    seq
}

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let ns = vec![100, 1000, 5000, 10_000, 50_000, 100_000];  // Different values of n to test
    let w = 40;
    let k = 15;

    for &n in &ns {
        let seq = random_dna_sequence(n);

        let start_time = Instant::now();
        let minimizers = minimizers_locations(&seq, w, k);
        let duration = start_time.elapsed();
        println!(
            "minimizers_locations_####_##### with n={}: {:.6} seconds, length of output: {}",
            n,
            duration.as_secs_f64(),
            minimizers.len()
        );

        let start_time = Instant::now();
        let minimizers_mono_queue = minimizers_locations_mono_queue(&seq, w, k);
        let duration = start_time.elapsed();
        println!(
            "minimizers_locations_mono_queue with n={}: {:.6} seconds, length of output: {}",
            n,
            duration.as_secs_f64(),
            minimizers_mono_queue.len()
        );

        println!();
    }
}
