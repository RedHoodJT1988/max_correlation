use std::io;
use max_correlation::algorithms::correlation::max_array_correlation;

fn main() {
    println!("Enter numbers for array A (comma-separated):");
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).expect("Failed to read input");

    println!("Enter numbers for array B (comma-separated):");
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("Failed to read input");

    let a: Vec<i32> = input_a.trim().split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    
    let b: Vec<i32> = input_b.trim().split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if a.len() != b.len() {
        eprintln!("Error: Both arrays must have the same length!");
        return;
    }

    let result = max_array_correlation(a, b);
    println!("Maximum Array Correlation: {}", result);
}
