use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    let runs: u64 = 100_000;
    let mut runtimes = Vec::new();
    let iterations: u64 = 100_000;
    let top: u64 = 100_000_000_000;

    for _ in 1..=runs {
        let runtime = run_benchmark(iterations, top);

        runtimes.push(runtime.as_micros());
    }

    let duration = start.elapsed();
    let average = average(&runtimes);
    let min = runtimes.iter().min().unwrap();
    let max = runtimes.iter().max().unwrap();

    println!("benchmark runs: {}", runs);
    println!("per run: {} iterations; range(0..={})", iterations, top);
    println!("min {min} µs; max {max} µs; average {average} µs");
    println!("duration: {} s", duration.as_secs());
}

fn calculate(bottom: u64, top: u64) -> u64 {
    return (bottom..=top).filter(|x| x % 2 == 0).sum();
}

fn run_benchmark(iterations: u64, top: u64) -> Duration {
    let start = Instant::now();

    for _ in 1..=iterations {
        calculate(0, top);
    }

    return start.elapsed();
}

fn average(numbers: &Vec<u128>) -> u128 {
    let sum: u128 = numbers.iter().sum();
    let count = numbers.len() as u128;

    return sum / count;
}
