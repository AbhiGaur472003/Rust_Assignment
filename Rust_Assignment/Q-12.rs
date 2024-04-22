use std::io;

fn main() {
    println!("Enter the array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let array: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let max_sum = max_subarray_sum(&array);

    println!("Maximum subarray sum: {}", max_sum);
}

fn max_subarray_sum(array: &[i32]) -> i32 {
    if array.is_empty() {
        return 0;
    }

    let mut max_ending_here = array[0];
    let mut max_so_far = array[0];

    for &num in array.iter().skip(1) {
        max_ending_here = num.max(max_ending_here + num);
        max_so_far = max_so_far.max(max_ending_here);
    }

    max_so_far
}
