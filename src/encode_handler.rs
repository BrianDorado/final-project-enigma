#[path = "./initialization.rs"] mod initialization;
use initialization::{
    ROTOR_I,
    ROTOR_II,
    ROTOR_III,
    ROTOR_IV,
    ROTOR_V,
    REFLECTOR_A,
    REFLECTOR_B,
    REFLECTOR_C
};


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

pub fn encode_handler(message: &mut Vec<char>) {
        
        println!("Message recieved: {:?}", message);
        // let mut input_to_chars: Vec<char> = message.chars().collect();
       for (ind, chr) in message.iter_mut().enumerate() {
        *chr = ROTOR_I.encode(ind as isize, *chr);
        // print!("Waiting for input!")
    }

        // let encoded_output: String = input_to_chars.into_iter().collect();

        // println!("{}", encoded_output)
}
