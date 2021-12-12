// O(n) - Linear - For Loops , while loops through n items

// n is the number of items in the collection or array or list or vector.

use std::time::Instant;
fn main() {
    // Create a vector if 10 strings
    let small = vec![String::from("nemo")];

    let _everyone = vec![
        String::from("dory"),
        String::from("bruce"),
        String::from("marlin"),
        String::from("nemo"),
        String::from("flipper"),
        String::from("bloat"),
        String::from("nigel"),
        String::from("squirt"),
        String::from("darla"),
        String::from("hank"),
    ];

    // large vector
    let _large = vec![String::from("nemo"); 1000000];
    // start the timer
    let time = Instant::now();
    linear_search(&small, "nemo"); // O(n)
    println!(
        "The time for linear search took {:?} microseconds",
        time.elapsed().as_micros()
    );
}

fn linear_search(list: &Vec<String>, item: &str) {
    for i in list {
        if i == item {
            print!("NEMO Found!! \n");
        }
    }
}



