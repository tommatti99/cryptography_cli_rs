use clap::{Args, ValueEnum};

const ALPHABET: [&str; 26] = ["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];
const ALPHABET_LEN: i8 = 26;

#[derive(Debug, Args, Clone)]
pub struct CesarCipherAlg {
    /// Encode or Decode Operation
    #[arg(short, long)]
    pub operation: Operations,
    /// Shift number
    #[arg(short, long)]
    pub shift: i8,
    /// The phrase to encode or decode
    #[arg(short, long)]
    pub phrase: String
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Operations {
    Encode,
    Decode
}

impl CesarCipherAlg {
    pub fn execute(self) -> () {
        match self.operation {
            Operations::Encode => {
                self.encode_decode();
            } 
            Operations::Decode => {
                self.encode_decode();
            } 
        }
    }
    
    fn encode_decode(self) -> () {
        let mut encoded_phrase: String = String::new();

        for c in self.phrase.chars() {
            match ALPHABET.get(self.clone().shift_pos(c)) {
                Some(new_letter) => {
                    encoded_phrase  = format!("{}{}", encoded_phrase, new_letter);
                },
                None => {
                    encoded_phrase  = format!("{}{}", encoded_phrase, " ");
                }
            }
        }
        println!("{}", encoded_phrase);
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


fn get_letter_position(letter: &str) -> usize {
    return ALPHABET.iter().position(|x: &&str| *x == letter.to_uppercase()).unwrap().try_into().unwrap();
}