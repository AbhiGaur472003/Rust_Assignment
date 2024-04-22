use std::io;

fn main() {
    println!("Enter the sorted array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let numbers: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();
    
    println!("Enter the target number:");
    let mut target = String::new();
    io::stdin().read_line(&mut target)
        .expect("Failed to read line");
    
    let target: i32 = target.trim().parse().expect("Invalid number");
    
    // Answer will be 0 based indexing
    match find_first_occurrence(&numbers, target) {
        Some(index) => println!("The first occurrence of {} is at index {}", target, index),
        None => println!("The number {} is not found in the array.", target),
    }
}

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &num) in arr.iter().enumerate() {
        if num == target {
            return Some(index);
        }
    }
    None
}
