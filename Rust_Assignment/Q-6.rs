use std::io;

fn main() {
    println!("Enter the strings separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let strings: Vec<&str> = input.trim().split_whitespace().collect();

    let longest_common_prefix = find_longest_common_prefix(&strings);
    
    match longest_common_prefix {
        Some(prefix) => println!("The longest common prefix is: {}", prefix),
        None => println!("No common prefix found."),
    }
}

fn find_longest_common_prefix(strings: &[&str]) -> Option<String> {
    if strings.is_empty() {
        return None;
    }
    
    let mut common_prefix = String::new();
    let first_string = strings[0];
    
    'outer: for (i, ch) in first_string.chars().enumerate() {
        for string in strings.iter().skip(1) {
            if let Some(curr_ch) = string.chars().nth(i) {
                if curr_ch != ch {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        common_prefix.push(ch);
    }
    
    if common_prefix.is_empty() {
        None
    } else {
        Some(common_prefix)
    }
}
