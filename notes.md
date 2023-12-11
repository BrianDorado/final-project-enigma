Old code:
fn main() {
        let mut message = collect_input();
        println!("Found: {:?}", message);
        // let mut input = String::new(); // message to encrypt

        // println!("Please enter a message to encrypt: ");
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("Failed to read line");

        // input = input.to_uppercase();

        // let mut input_to_chars: Vec<char> = input.chars().collect();

        //  1. Each Letter needs to run through each rotor, the reflector, then the rotors again
        // 2. Offset is not increased until the cycle is complete

        
        // for (ind, chr) in input.chars().enumerate() {
        //     input_to_chars[ind] = ROTOR_I.encode(ind as isize, chr);
        //     // input_to_chars[ind] = REFLECTOR_A.encode(chr)
            
        // }

        // let encoded_output: String = input_to_chars.into_iter().collect();

        // println!("{}", encoded_output)
     
}

// pub struct Wiring {
//     input: Vec<char>,
//     output: Vec<char>
// }













encode handler.rs

// use crate::rotors::Rotor;

// #[path = "../src/initialization.rs"] mod initialization;

// use crate::initialization::{ROTOR_I, ROTOR_II, ROTOR_III, ROTOR_IV, ROTOR_V};
// use crate::initialization::{REFLECTOR_A, REFLECTOR_B, REFLECTOR_C};

// use crate::initialization::ROTOR_I;


// pub fn encode_handler(mut message: Vec<char>) -> String {
//         // println!("Found: {:?}", message);
//         // //  1. Each Letter needs to run through each rotor, the reflector, then the rotors again
//         // // 2. Offset is not increased until the cycle is complete
//         // for (ind, chr) in message.iter_mut().enumerate() {
//         //     *chr = ROTOR_I::encode(ind as isize, *chr); 
//         // }

//     // Convert the modified message vector directly into a String
//         let encoded_output: String = message.into_iter().collect();

//         println!("{}", encoded_output);

//         encoded_output
// }

// pub fn encode_handler(message: &mut Vec<char>, rotor: &mut Rotor) {
        
//         println!("Message recieved: {:?}", message);
//         // let mut input_to_chars: Vec<char> = message.chars().collect();
        
//        for (ind, chr) in message.iter_mut().enumerate() {
//         *chr = rotor.encode(ind as isize, *chr);
//         // print!("Waiting for input!")
//     }

//         // let encoded_output: String = input_to_chars.into_iter().collect();

//         // println!("{}", encoded_output)
// }






pub fn encode_handler(message: &mut Vec<char>, rotor: &mut Rotor) {
        
        println!("Message recieved: {:?}", message);
        // let mut input_to_chars: Vec<char> = message.chars().collect();
        
       for (ind, chr) in message.iter_mut().enumerate() {
        *chr = rotor.encode(ind as isize, *chr);
        // print!("Waiting for input!")
    }

        // let encoded_output: String = input_to_chars.into_iter().collect();

        // println!("{}", encoded_output)
}
