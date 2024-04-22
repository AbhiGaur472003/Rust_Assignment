use std::io;

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num: u64 = input.trim().parse().expect("Invalid number");
    
    if is_prime(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}

fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    if num == 2 || num == 3 {
        return true;
    }
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }
    
    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
