use std::io;

fn main() {
    println!("Enter a string of words:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let shortest_word = find_shortest_word(&input);
    
    match shortest_word {
        Some(word) => println!("The shortest word is: {}", word),
        None => println!("No words found in the input."),
    }
}

fn find_shortest_word(input: &str) -> Option<&str> {
    let words: Vec<&str> = input.trim().split_whitespace().collect();
    
    if words.is_empty() {
        return None;
    }
    
    let shortest_word = words.iter().min_by_key(|&&word| word.len());
    
    shortest_word.copied()
}
