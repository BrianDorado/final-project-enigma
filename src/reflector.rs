/// THis is a top level doc string for the reflectors file
    #[allow(dead_code)]
    struct Wiring {
        input: [char; 26],
        output: [char; 26]
    }
    #[allow(dead_code)]
    pub struct Reflector {
        /// # Reflector must have ID for correct configuration
        id: char,
        wiring: Wiring,
    }
    
    impl Reflector {

        /// There is now a doc string in the Reflector impl
        #[allow(dead_code)]
        pub fn encode(&mut self,char_to_encode:char) -> char {
            
            if char_to_encode.is_whitespace() { return ' ' } 
                
                let index: Option<usize> = self.wiring.input.iter().position(|&c| c == char_to_encode);
                
                match index {
                    Some(i) => return self.wiring.output[i],
                    None => {
                        println!("Index not found, check rotor configuration");
                    }
                    
                }

            char_to_encode
        }

        #[allow(dead_code)]
        pub const fn new(id: char, input: [char; 26], output:[char; 26]) -> Reflector {

            Reflector { 
                id, 
                wiring: Wiring { 
                    input, 
                    output
                }, 
            }
        }
    
}


mod tests {
    use super::*;
    #[allow(dead_code)]
    static TEST_REFLECTOR_B: Reflector = Reflector::new(
        '2', 
            ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'], 
            ['Y','R','U','H','Q','S','L','D','P','X','N','G','O','K','M','I','E','B','F','Z','C','W','V','J','A','T']
    );

    #[test]
    fn reflector_initialized_correclty() {
       
        assert_eq!(TEST_REFLECTOR_B.id, '2');
        assert_eq!(TEST_REFLECTOR_B.wiring.input, ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'])

    }

    // #[test]

    // fn it_encodes_properly(){
    //     let enc_char = TEST_REFLECTOR_B.encode('Z');
    //     assert_eq!(enc_char, 'T')
    // }
}