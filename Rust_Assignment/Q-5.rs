use std::io;

fn main() {
    println!("Enter the sorted array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let numbers: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let median = find_median(&numbers);
    
    match median {
        Some(median_value) => println!("The median is: {}", median_value),
        None => println!("No median found for an empty array."),
    }
}

fn find_median(numbers: &[i32]) -> Option<f64> {
    let len = numbers.len();
    if len == 0 {
        return None;
    }
    
    if len % 2 == 0 {
        // If the length of the array is even, average the middle two numbers
        let middle_index = len / 2;
        let median = (numbers[middle_index - 1] + numbers[middle_index]) as f64 / 2.0;
        Some(median)
    } else {
        // If the length of the array is odd, return the middle number
        let middle_index = len / 2;
        Some(numbers[middle_index] as f64)
    }
}
