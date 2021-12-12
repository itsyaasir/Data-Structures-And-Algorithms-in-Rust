// O(1) - Constant time
// The time to execute a function is constant, regardless of the input.

use std::time::Instant;

fn main() {
    let small = vec![String::from("nemo"); 10];
    let medium = vec![String::from("nemo"); 100];
    let large = vec![String::from("nemo"); 100_000];

    finding_nemo(&large);
    finding_nemo(&small);
    finding_nemo(&medium);
}

fn finding_nemo(list: &Vec<String>) {
    let t1 = Instant::now();
    println!("list[0] = {}", list[0]);
    println!("list[1] = {}", list[1]);
    let t2 = Instant::now();
    println!(
        "Time taken: {} microseconds ",
        t2.duration_since(t1).as_millis()
    );
}

// The time taken in all 3 instances was 0 seconds/microseconds.
