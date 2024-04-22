use std::io;

fn main() {
    println!("Enter the first sorted array of integers separated by spaces:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1)
        .expect("Failed to read line");

    let array1: Vec<i32> = input1.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    println!("Enter the second sorted array of integers separated by spaces:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2)
        .expect("Failed to read line");

    let array2: Vec<i32> = input2.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let merged_array = merge_sorted_arrays(&array1, &array2);

    println!("Merged sorted array: {:?}", merged_array);
}

fn merge_sorted_arrays(array1: &[i32], array2: &[i32]) -> Vec<i32> {
    let mut merged_array = Vec::with_capacity(array1.len() + array2.len());

    let mut i = 0;
    let mut j = 0;

    while i < array1.len() && j < array2.len() {
        if array1[i] <= array2[j] {
            merged_array.push(array1[i]);
            i += 1;
        } else {
            merged_array.push(array2[j]);
            j += 1;
        }
    }

    // Append remaining elements of array1, if any
    while i < array1.len() {
        merged_array.push(array1[i]);
        i += 1;
    }

    // Append remaining elements of array2, if any
    while j < array2.len() {
        merged_array.push(array2[j]);
        j += 1;
    }

    merged_array
}
