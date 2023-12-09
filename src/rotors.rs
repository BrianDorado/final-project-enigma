pub struct Wiring {
    input: Vec<char>,
    output: Vec<char>
}

pub struct Rotor {
    /// # Rotor must have ID for correct configuration
    id: char,
    wiring: Wiring,
    position: usize
}

impl Rotor {
    pub fn encode(&mut self, offset:isize, char_to_encode:char) -> char {
        /// # Offset argument -> Postive encodes forward; Negative encodes backward
        println!( "Char to Encode: {}", char_to_encode); 
        println!("Current Offset: {:?}", offset);
        if char_to_encode.is_whitespace() {
            println!("Character is a space!"); 
            return ' '
        } else {

            let index = self.wiring.input.iter().position(|&c| c == char_to_encode);
            
            match index {
                Some(i) => self.wiring.output[i],
                None => {
                    println!("Index not found, check rotor configuration");
                    char_to_encode
                } 
            }
        }
        // println!("Current Letter: {:?}", c)
    }

    pub fn new(id: char, input: Vec<char>, output:Vec<char>) -> Rotor {

        Rotor { 
            id: id, 
            wiring: Wiring { 
                input: input, 
                output: output }, 
            position: 0 }
    }
    
}