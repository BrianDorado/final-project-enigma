mod input_handler;
use crate::input_handler::collect_input;

mod encode_handler;
use crate::encode_handler::encode_handler;


fn main() {
    let ec = encode_handler(&mut collect_input());

    println!("Enigma'd: {}", ec)
     
}