    struct Wiring {
        input: [char; 26],
        output: [char; 26]
    }
    
    pub struct Reflector {
        /// # Reflector must have ID for correct configuration
        id: char,
        wiring: Wiring,
    }
    
    impl Reflector {
        pub fn encode(&mut self,char_to_encode:char) -> char {
            
            if char_to_encode.is_whitespace() {
                return ' '
            } else {
                
                let index: Option<usize> = self.wiring.input.iter().position(|&c| c == char_to_encode);
                
                match index {
                    Some(i) => return self.wiring.output[i],
                    None => {
                        println!("Index not found, check rotor configuration")
                    }
                    
                }
            }
            char_to_encode
        }
        
        pub const fn new(id: char, input: [char; 26], output:[char; 26]) -> Reflector {
            
            Reflector { 
                id: id, 
                wiring: Wiring { 
                    input: input, 
                output: output
            }, 
        }
    }
    
}
