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

pub fn encode_handler(message: &mut Vec<char>) {
        
        println!("Message recieved: {:?}", message);
        // ensures that only one thread can access the Rotor at a time.
        let mut rotor_i = ROTOR_I.lock().unwrap();

        for (ind, chr) in message.iter_mut().enumerate() {
        *chr = rotor_i.encode(ind as isize, *chr);
    }
        println!("Message encrypted: {:?}", message)
        // let encoded_output: String = input_to_chars.into_iter().collect();

        // println!("{}", encoded_output)
}
