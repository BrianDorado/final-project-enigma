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


Naive encode funciton

//     pub fn encode(&mut self, offset:isize, char_to_encode:char) -> char {
//             // Offset argument -> Postive encodes forward; Negative encodes backward
            
//         if char_to_encode.is_whitespace() {
//                 return ' '
//         } else {
//             let index: Option<usize> = self.wiring.input.iter().position(|&c| c == char_to_encode);
                
//             match index {
//                 Some(i) => self.position = i,
//                 None => {
//                 println!("Index not found, check rotor configuration")
//             }
//         }
//     }
//             // println!("Current position: {}", self.position);
//         let ouija_board:usize = (self.position + offset as usize) % 26; 
//         self.wiring.output[ouija_board]
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






// encode handler debugger

*chr = rotor_i.encode(ind as isize, *chr);
                if ind == 0 {println!("{}", chr)}
            *chr = rotor_ii.encode(ind as isize, *chr);
                if ind == 0 {println!("{}", chr)}
            *chr = rotor_iii.encode(ind as isize, *chr);
                if ind == 0 {println!("{}", chr)}
            *chr = reflector_a.encode(*chr);
                if ind == 0 {println!("{}", chr)}
            // after passing through the reflector, the direction of the signal is reversed, and so instead of passing again through the rotor *input -> *output it should *output -> *input

            *chr = rotor_iii.encode(ind as isize, *chr);
                if ind == 0 {println!("{}", chr)}
            *chr = rotor_ii.encode(ind as isize, *chr);
                if ind == 0 {println!("{}", chr)}
            *chr = rotor_i.encode(ind as isize, *chr);
                if ind == 0 {println!("{}", chr)}







//     pub fn encode(&self, offset:isize, char_to_encode: char,  forward: bool) -> char {
//         if char_to_encode.is_whitespace() {
//                 return ' '
//         } else {
//             let index = if forward {
//                 self.wiring.input.iter().position(|&c| c == char_to_encode).unwrap()
//             } else {
//                 self.wiring.output.iter().position(|&c| c == char_to_encode).unwrap()
//             };
//             // let effective_index = (index +offset as usize) % 26;
//             if forward {
//                 self.wiring.output[index]
//             } else {
//                 self.wiring.input[index]
//             }
//         }
// }



/// 1. Wiring Struct Never Constructed -> 
/// 
/// 
// warning: struct `Wiring` is never constructed
//  --> src\rotors.rs:2:8
//   |
// 2 | struct Wiring {
//   |        ^^^^^^
//   |
//   = note: `#[warn(dead_code)]` on by default



/// 2. Rotor Struct Never Constructed ->
/// 
/// 
// warning: struct `Rotor` is never constructed
//  --> src\rotors.rs:7:12
//   |
// 7 | pub struct Rotor {
//   |            ^^^^^



/// 3. Rotor Methods encode and new ->
/// 
/// 
// warning: associated items `encode` and `new` are never used
//   --> src\rotors.rs:19:8
//    |
// 16 | impl Rotor {
//    | ---------- associated items in this implementation
// ...
// 19 | pub fn encode(&self, offset: isize, char_to_encode: char, forward: bool) -> char {
//    |        ^^^^^^
// ...
// 37 |     pub const fn new(id: char, input: [char; 26], output:[char; 26]) ->     Rotor {
//    |                  ^^^



/// 4. Wiring Struct fields input and output never read
/// 
/// 
// warning: fields `input` and `output` are never read
//  --> src\rotors.rs:3:5
//   |
// 2 | struct Wiring {
//   |        ------ fields in this struct
// 3 |     input: [char; 26],
//   |     ^^^^^
// 4 |     output: [char; 26]
//   |     ^^^^^^



/// 5. Rotor Struct fields wiring and position are never read
/// 
/// 
// warning: fields `id`, `wiring`, and `position` are never read
//   --> src\rotors.rs:9:5
//    |
// 7  | pub struct Rotor {
//    |            ----- fields in this struct
// 8  |         /// # Rotor must have ID for correct configuration
// 9  |     id: char,
//    |     ^^
// 10 |     wiring: Wiring,
//    |     ^^^^^^
// 11 |     position: usize // For configuration should also be customizable and there for part of the constructor
//    |     ^^^^^^^^


/// 6. Rotor Method encode never used (see allowance 3)
/// 
/// 
// warning: method `encode` is never used
//   --> src\rotors.rs:19:8
//    |
// 16 | impl Rotor {
//    | ---------- method in this implementation
// ...
// 19 | pub fn encode(&self, offset: isize, char_to_encode: char, forward: bool) -> char {