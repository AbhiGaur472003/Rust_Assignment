use std::io;

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let reversed_string = reverse_string(input.trim());

    println!("Reversed string: {}", reversed_string);
}

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}
