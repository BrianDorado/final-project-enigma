mod input_handler;
use crate::input_handler::collect_input;

mod encode_handler;
use crate::encode_handler::encode_handler;


fn main() {
    // let encrypted_message: () = encode_handler(&mut collect_input(), initialization::ROTOR_I);

    // let message_to_encrypt = collect_input();

    encode_handler(&mut collect_input());
     
}