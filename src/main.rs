use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    const RUNS: usize = 100_000;
    let mut runtimes: [usize; RUNS] = [0; RUNS];
    let iterations: u64 = 100_000;
    let top: u64 = 100_000_000_000;

    for i in 0..RUNS {
        let runtime = run_benchmark(iterations, top);

        runtimes[i] = runtime.as_micros() as usize;
    }

    let duration = start.elapsed();
    let average = average(&runtimes);
    let min = runtimes.iter().min().unwrap();
    let max = runtimes.iter().max().unwrap();

    println!("benchmark runs: {}", RUNS);
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

fn average(numbers: &[usize]) -> usize {
    let sum: usize = numbers.iter().sum();
    let count = numbers.len() as usize;

    return sum / count;
}
