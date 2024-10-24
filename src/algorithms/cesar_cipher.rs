use clap::Args;
use crate::{alphabet::{ALPHABET, ALPHABET_LEN, get_letter_position}, Operations};


#[derive(Debug, Args, Clone)]
pub struct CesarCipherAlg {
    /// Encode or Decode Operation
    #[arg(short, long)]
    pub operation: Operations,
    /// Shift number
    #[arg(short, long)]
    pub shift: i8,
    /// The message to encode or decode
    #[arg(short, long)]
    pub message: String
}


impl CesarCipherAlg {
    pub fn execute(self) -> () {
        match self.operation {
            Operations::Encrypt => {
                self.encode_decode();
            } 
            Operations::Decrypt => {
                self.encode_decode();
            } 
        }
    }
    
    fn encode_decode(self) -> () {
        let mut encoded_message: String = String::new();

        for c in self.message.chars() {
            match ALPHABET.get(self.clone().shift_pos(c)) {
                Some(new_letter) => {
                    encoded_message  = format!("{}{}", encoded_message, new_letter);
                },
                None => {
                    encoded_message  = format!("{}{}", encoded_message, " ");
                }
            }
        }
        println!("{}", encoded_message);
    }

    fn shift_pos(self, c: char) -> usize {
        let letter_pos: i8 = get_letter_position(c.to_string().as_str()).try_into().unwrap();

        if (letter_pos + self.shift) > ALPHABET_LEN {
            return (letter_pos + self.shift - ALPHABET_LEN).try_into().unwrap();
        }
        if (letter_pos + self.shift) < 0 {
            return  (letter_pos - self.shift + ALPHABET_LEN).try_into().unwrap();
        }

        return (letter_pos + self.shift).try_into().unwrap();
    }
}


