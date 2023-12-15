//!  # Enigma Emulator
//! This emulator aims to replicate the functionality of the original Enigma Machine, allowing users to encrypt and decrypt messages using the same mechanisms.
//! Each Rotor and has it's own configuration. There are five Rotors in total and three reflectors. They can be chained in any configuration and in any order. Although to best reflect the accuracy of the actual machine, only three rotors and one reflector should be used. 
//! 
//! ## Encoding Mesage
//! Each rotor and reflector has its own encoding method. Which takes an offset, the character you wish to encode, and the direction of the signal as arguments
//! ```
//! fn encode(offset: isize, char_to_encode: char, forward: bool){
//! ... encoding logic
//! }
//! ```
//! The offset represents the physical rotation of each rotor during encryption (see README for more information)
//! The direction is which direction the signal is traveling through the rotor (again, see README for more info). Note that direction does NOT specify whether the message is being encoded or decoded.The direction of travel is determined by which side of the reflector are you on? 
//! If you are moving towards the reflector than true is the argument. If you have passed through the reflector than the direciton will be false.
//! 
//! ## Example Code
//! This code is taken directly from the Encode handler and demonstrates how it may be used. 
//! 
//! - Note: ROTOR_X.lock().unwrap(); ensures that only one thread can access the given rotor [x] at a time
//! ``` 
//! encode_handler(message: &mut Vec<char>) -> String {

//!     let mut rotor_i = ROTOR_I.lock().unwrap(); 
//!     let mut rotor_ii = ROTOR_II.lock().unwrap(); 
//!     let mut rotor_iii = ROTOR_III.lock().unwrap(); 
//!     let mut reflector_a = REFLECTOR_A.lock().unwrap(); 

//!     for (ind, chr) in message.iter_mut().enumerate() {
            
    //!     *chr = rotor_i.encode(ind as isize, *chr, true);
    //!     *chr = rotor_ii.encode(ind as isize, *chr, true);
    //!     *chr = rotor_iii.encode(ind as isize, *chr, true);
    //!     *chr = reflector_a.encode(*chr);
    // after passing through the reflector, the direction of the signal is reversed, and so instead of passing again through the rotor *input -> *output it should *output -> *input

    //!     *chr = rotor_iii.encode( ind as isize, *chr, false);
    //!     *chr = rotor_ii.encode( ind as isize, *chr, false);
    //!     *chr = rotor_i.encode( ind as isize, *chr, false);
//! }

    //!     message.iter().collect()
//! }
//! ```
//! 
//! Running the code above and sending the message "Hello!" should result in the message being encrypted as BHHPV! 
//! 
//! # Decoding
//! In order to decode, all you have to do is simple pass the encoded message, in the case above "BHHPV" though the exact same set up as before you will get the orignal message out. No seperate arguments or decode mode needed. 
//! 
//! The process should work for a message of any length
//! - Note that special characters and spaces are not encoded and are simply returned as themselves. 
mod rotors;
mod reflector;
mod initialization;
mod input_handler;
mod encode_handler;