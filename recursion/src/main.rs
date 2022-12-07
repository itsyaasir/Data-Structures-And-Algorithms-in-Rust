fn main() {
    let x = fibonacci(10);
    println!("{:?}", x)
}

#[warn(dead_code)]
fn factorial(number: usize) -> usize {
    match number {
        0 => 0,
        1 => 1,
        _ => number * factorial(number - 1),
    }
}

// Fibonacci

fn fibonacci(limit: i32) -> Vec<i32> {
    //  0, 1, 1, 2, 3, 5, 8, 13, 21, 34
    let mut series = Vec::new();
    for i in 0..limit {
        series.push(fibonacci_recursive(i));
    }
    series
}

fn fibonacci_recursive(number: i32) -> i32 {
    match number {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(number - 1) + fibonacci_recursive(number - 2),
    }
}
