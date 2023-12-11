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

pub fn encode_handler(message: &mut Vec<char>) -> String {
        
        println!("Message recieved: {:?}", message);
        
        let mut rotor_i = ROTOR_I.lock().unwrap(); // ensures that only one thread can access the Rotor at a time.
        let mut rotor_ii = ROTOR_II.lock().unwrap(); // ensures that only one thread can access the Rotor at a time.
        let mut rotor_iii = ROTOR_III.lock().unwrap(); // ensures that only one thread can access the Rotor at a time.
        let mut reflector_a = REFLECTOR_A.lock().unwrap(); // ensures that only one thread can access the Rotor at a time.

        for (ind, chr) in message.iter_mut().enumerate() {

            // There is a better way to do this... this is just the naive initial version to get it working.
            
            *chr = rotor_i.encode(ind as isize, *chr, true);
            *chr = rotor_ii.encode(ind as isize, *chr, true);
            *chr = rotor_iii.encode(ind as isize, *chr, true);
            *chr = reflector_a.encode(*chr);
            // after passing through the reflector, the     direction of the signal is reversed, and so instead of passing again through the rotor *input -> *output it should *output -> *input

            // It should not return the same letter

            *chr = rotor_iii.encode( ind as isize, *chr, false);
            *chr = rotor_ii.encode( ind as isize, *chr, false);
            *chr = rotor_i.encode( ind as isize, *chr, false);
    }

        message.iter().collect()
}
