use std::io;

fn main() {
    println!("Enter a string:");
    
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let input = input.trim(); // Trim whitespace
    
    if is_palindrome(input) {
        println!("{} is a palindrome!",input);
    } else {
        println!("{} is not a palindrome.",input);
    }
}

fn is_palindrome(s: &str) -> bool {
    let reversed = s.chars().rev().collect::<String>(); // Reverse the string
    s == reversed
}
