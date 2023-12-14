#[path = "./initialization.rs"] mod initialization;
use initialization::{
    ROTOR_I,
    ROTOR_II,
    ROTOR_III,
    // ROTOR_IV,
    // ROTOR_V,
    REFLECTOR_A,
    // REFLECTOR_B,
    // REFLECTOR_C
};
/// # This doc string describes the encode handler in detail

#[allow(dead_code)]
pub fn encode_handler(message: &mut Vec<char>) -> String {
        
        // println!("Message recieved: {:?}", message);

        // ROTOR_X.lock().unwrap(); ensures that only one thread can access the given rotor [x] at a time

        // they do need to be mutable
        #[allow(unused_mut)] // not sure why this is needed. It does in fact need to be mutable, but it says it doesn't and yet when I remove the mut it complains that it must be mut... so its mut and surpressing the warning
        let mut rotor_i = ROTOR_I.lock().unwrap(); 
        #[allow(unused_mut)]
        let mut rotor_ii = ROTOR_II.lock().unwrap(); 
        #[allow(unused_mut)]
        let mut rotor_iii = ROTOR_III.lock().unwrap(); 
        let mut reflector_a = REFLECTOR_A.lock().unwrap(); 

        for (ind, chr) in message.iter_mut().enumerate() {

            // There is a better way to do this... this is just the naive initial version to get it working.
            
            *chr = rotor_i.encode(ind as isize, *chr, true);
            *chr = rotor_ii.encode(ind as isize, *chr, true);
            *chr = rotor_iii.encode(ind as isize, *chr, true);
            *chr = reflector_a.encode(*chr);
            // after passing through the reflector, the direction of the signal is reversed, and so instead of passing again through the rotor *input -> *output it should *output -> *input

            // It should not return the same letter

            *chr = rotor_iii.encode( ind as isize, *chr, false);
            *chr = rotor_ii.encode( ind as isize, *chr, false);
            *chr = rotor_i.encode( ind as isize, *chr, false);
    }

        message.iter().collect()
}

mod tests {
    #[allow(unused_imports)] // not sure why this in necessary since I am clearly using encode_handler in the test function but it keeps warning me that its not being used.
    use super::encode_handler;
    #[test]
    fn it_encodes_correctly() {
        let mut test_message: Vec<char> =  vec!['H', 'E', 'L', 'L', 'O', '!'];
        let encoded = encode_handler(&mut test_message);

        // With starting positions of each rotor being zero
        assert_eq!(encoded, "BHHPV!")

    }
}
