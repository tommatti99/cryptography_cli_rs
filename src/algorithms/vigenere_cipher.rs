use clap::Args;
use crate::{alphabet::{get_text_values, ALPHABET}, Operations};

#[derive(Debug, Args, Clone,PartialEq)]
pub struct VigenereCipherAlg {
    /// Encode or Decode Operation
    #[arg(short, long)]
    pub operation: Operations,
    /// The secret word for encrypt
    #[arg(short, long)]
    pub key: String,
    /// The phrase to encode or decode
    #[arg(short, long)]
    pub phrase: String,
}

impl VigenereCipherAlg {
    pub fn execute(self) -> () {
        match self.operation {
            Operations::Decode => {
                self.decode();
            },
            Operations::Encode => {
                self.encode();
            }
        }
    }
    fn encode(self) -> () {
        let phrase_values: Vec<usize> = get_text_values(self.clone().phrase);
        let key_values: Vec<usize> = get_text_values(self.extend_key().key);
        
        if phrase_values.len() != key_values.len() {
            println!("internal error");
            return ();
        }

        let encoded_msg_vec: Vec<usize> = phrase_values.iter().zip(key_values.iter()).map(|(x, y)| x + y).collect::<Vec<usize>>();
        let encoded_msg_string: String = encoded_msg_vec.iter().map(|x: &usize| ALPHABET[*x]).collect::<Vec<&str>>().concat();

        println!("{}", encoded_msg_string);
    }

    fn decode(self) -> () {
        let phrase_values: Vec<usize> = get_text_values(self.clone().phrase);
        let key_values: Vec<usize> = get_text_values(self.extend_key().key);
        
        if phrase_values.len() != key_values.len() {
            println!("internal error");
            return (); 
        }

        let dencoded_msg_vec: Vec<usize> = phrase_values.iter().zip(key_values.iter()).map(|(x, y)| x - y).collect::<Vec<usize>>();
        let dencoded_msg_string: String = dencoded_msg_vec.iter().map(|x: &usize| ALPHABET[*x]).collect::<Vec<&str>>().concat();

        println!("{}", dencoded_msg_string);
    }

    fn extend_key(self) -> Self {
        let mut count: usize = 0;
        let mut extended_key: String = String::new();
        let original_key_vec: Vec<char> = self.key.chars().into_iter().map(|x: char| x).collect::<Vec<char>>();

        for _ in 1..=self.phrase.len() {
            extended_key = format!("{}{}", extended_key, original_key_vec[count]);
            count += 1;

            if count == original_key_vec.len() {
                count = 0
            }
        } 
        return Self {
            operation: self.operation,
            key: extended_key,
            phrase: self.phrase
        };
    }
}

#[cfg(test)]
mod vigenere_cipher_test {
    use super::*;

    #[test]
    pub fn extend_key_test() -> () {
        let test_instance_1: VigenereCipherAlg = 
            VigenereCipherAlg {
                operation: crate::Operations::Encode,
                key: "banana".to_string(),
                phrase: "aaabbbcccdddeee".to_string(),
            };
        assert_eq!(test_instance_1.clone().extend_key(), VigenereCipherAlg {
                                                                    operation: test_instance_1.operation,
                                                                    key: "bananabananaban".to_string(), 
                                                                    phrase: test_instance_1.phrase});
    }

    #[test]
    pub fn encode_test() -> () {
        let test_instance_1: VigenereCipherAlg = 
            VigenereCipherAlg {
                operation: crate::Operations::Encode,
                key: "banana".to_string(),
                phrase: "aaabbbcccdddeee".to_string(),
            };
        println!("{:?}", test_instance_1.clone().encode());
    }

    #[test]
    pub fn decode_test() -> () {
        let test_instance_1: VigenereCipherAlg = 
            VigenereCipherAlg {
                operation: crate::Operations::Encode,
                key: "banana".to_string(),
                phrase: "BANBOBDCPDQDFER".to_string(),
            };
        println!("{:?}", test_instance_1.clone().decode());
    }
}