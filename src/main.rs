mod input_handler;
use crate::input_handler::collect_input;

mod encode_handler;
use crate::encode_handler::encode_handler;

// mod initialization;
// use initialization::{
//     ROTOR_I,
//     ROTOR_II,
//     ROTOR_III,
//     ROTOR_IV,
//     ROTOR_V,
//     REFLECTOR_A,
//     REFLECTOR_B,
//     REFLECTOR_C
// };


fn main() {
    // let encrypted_message: () = encode_handler(&mut collect_input(), initialization::ROTOR_I);

    // let message_to_encrypt = collect_input();

    encode_handler(&mut collect_input());
     
}