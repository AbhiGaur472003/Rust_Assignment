use std::io;

fn main() {
    println!("Enter the array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let numbers: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    println!("Enter the value of k:");
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input)
        .expect("Failed to read line");

    let k: usize = k_input.trim().parse().expect("Invalid number");

    let kth_smallest = find_kth_smallest(&numbers, k);
    
    match kth_smallest {
        Some(value) => println!("The {}th smallest element is: {}", k, value),
        None => println!("Array is empty or k is out of range."),
    }
}

fn find_kth_smallest(numbers: &[i32], k: usize) -> Option<i32> {
    if numbers.is_empty() || k == 0 || k > numbers.len() {
        return None;
    }
    
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort();

    Some(sorted_numbers[k - 1])
}
