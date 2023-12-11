use std::isize;
struct Wiring {
    input: [char; 26],
    output: [char; 26]
}
    
pub struct Rotor {
        /// # Rotor must have ID for correct configuration
    id: char,
    wiring: Wiring,
    position: usize // For configuration should also be customizable and there for part of the constructor
    // Initializing the position is also crucial for decoding
    // 
}

impl Rotor {
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

    pub fn encode(&self, offset:isize, char_to_encode: char,  forward: bool) -> char {
        if char_to_encode.is_whitespace() {
                return ' '
        } else {
            let index = if forward {
                self.wiring.input.iter().position(|&c| c == char_to_encode).unwrap()
            } else {
                self.wiring.output.iter().position(|&c| c == char_to_encode).unwrap()
            };
            // let effective_index = (index +offset as usize) % 26;
            if forward {
                self.wiring.output[index]
            } else {
                self.wiring.input[index]
            }
        }
}

    pub const fn new(id: char, input: [char; 26], output:[char; 26]) ->     Rotor {

        Rotor { 
            id: id, 
            wiring: Wiring { 
                input, 
                output
            }, 
            position: 0
        }
    }
}
