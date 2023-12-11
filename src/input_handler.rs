use std::io;
#[allow(dead_code)]
pub fn collect_input() -> Vec<char> {
        let mut input = String::new(); // message to encrypt
        
        println!("Please enter a message to encrypt: ");
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        input = input.to_uppercase();

        return input.chars().collect();

        // return input_to_chars
}