use std::isize;
#[allow(dead_code)]
struct Wiring {
    input: [char; 26],
    output: [char; 26]
}
    #[allow(dead_code)]
pub struct Rotor {
        /// # Rotor must have ID for correct configuration
    id: char,
    wiring: Wiring,
    position: usize // For configuration should also be customizable and there for part of the constructor
    // Initializing the position is also crucial for decoding
    // 
}

impl Rotor {

#[allow(dead_code)]
pub fn encode(&self, offset: isize, char_to_encode: char, forward: bool) -> char {
        // handle special characters:
            // Currently special characters like Punctuation and spaces are not encoded and simply returned as themselves.
        if char_to_encode.is_ascii_punctuation() {return char_to_encode}
        if char_to_encode.is_whitespace() {return ' '}

        if forward {
            let input_index = self.wiring.input.iter().position(|&c| c == char_to_encode).unwrap();
            let effective_index = (input_index as isize + offset).rem_euclid(26) as usize; // rem_euclid is mod 26 but ensures postive remainder!
            self.wiring.output[effective_index]
        } else {
            // Handle reverse signal direction (after the reflector)
            let output_index = self.wiring.output.iter().position(|&c| c == char_to_encode).unwrap();
            let effective_index = (output_index as isize - offset).rem_euclid(26) as usize; // rem_euclid is mod 26 but ensures postive remainder!
            self.wiring.input[effective_index]
        }
    }
    #[allow(dead_code)]
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

mod tests {

    use super::*;
    #[allow(dead_code)]
    static TEST_ROTOR_V: Rotor = Rotor::new(
        '2', 
        ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'], 
        ['Y','R','U','H','Q','S','L','D','P','X','N','G','O','K','M','I','E','B','F','Z','C','W','V','J','A','T']
    );

    #[test]
    fn reflector_initialized_correclty() {
       
        assert_eq!(TEST_ROTOR_V.id, '2')

    }
}

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