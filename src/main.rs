use std::fmt::Debug;

mod aoc_2023;

fn test_solution<T: Debug>(name: &str, mut func: impl FnMut() -> T) -> T {
    let val = func();

    let instant = std::time::Instant::now();

    let iterations = 5000;

    for _ in 0..iterations {
        let _ = func();
    }

    let elapsed = instant.elapsed();
    let average_ms = elapsed.as_millis() as f64 / iterations as f64;

    println!("Solution for {}: {:?} ({} ms)", name, val, average_ms);

    val
}

fn main() {
    test_solution("Day 1 Part 1", aoc_2023::day_01::part_one);
    test_solution("Day 1 Part 2", aoc_2023::day_01::part_two);
}
