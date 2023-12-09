use std::io;
// use std::fs;
use std::fs::read_to_string;

use crate::rotors::Rotor;
mod rotors;


fn read_lines_to_vector(file_name: &str) -> Vec<String> {
    
    read_to_string(file_name)
    .unwrap() 
    .lines() 
    .map(String::from) 
    .collect() 
}



fn main() {

    let mut rotor_I: Rotor = Rotor::new('1', vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'], vec!['E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O', 'W', 'Y', 'H', 'X', 'U', 'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J']);


        let mut input = String::new(); // message to encrypt

        println!("Please enter a message to encrypt: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.to_uppercase();

        let mut input_to_chars: Vec<char> = input.chars().collect();
        //  Each Letter needs to run through each rotor, the reflector, then the rotors again

        // Offset is not increased until the cycle is complete

        
        for (ind, chr) in input.chars().enumerate() {
            input_to_chars[ind] = rotor_I.encode(ind as isize, chr);
            // println!("Letter encoded as: {}", encc)
            
        }

        let encoded_output: String = input_to_chars.into_iter().collect();

        println!("{}", encoded_output)
    // println!("rotor I input: {:?}", rotor_I.wiring.input[0]);
    // println!("rotor I output: {:?}", rotor_I.wiring.output[0]);
     
}

// rotate just offset match 


