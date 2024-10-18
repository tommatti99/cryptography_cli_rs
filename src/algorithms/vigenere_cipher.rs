use clap::Args;
use crate::{alphabet::{get_text_values, ALPHABET}, Operations};

#[derive(Debug, Args, Clone, PartialEq)]
pub struct VigenereCipherAlg {
    /// Encode or Decode Operation
    #[arg(short, long)]
    pub operation: Operations,
    /// The secret word for encrypt
    #[arg(short, long)]
    pub key: String,
    /// The message to encode or decode
    #[arg(short, long)]
    pub message: String,
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
        let message_values: Vec<usize> = get_text_values(self.clone().message);
        let key_values: Vec<usize> = get_text_values(self.extend_key().key);
        
        if message_values.len() != key_values.len() {
            
            println!("internal error");
            return ();
        }

        let encoded_msg_vec: Vec<usize> = message_values.iter().zip(key_values.iter()).map(|(x, y)| x + y).collect::<Vec<usize>>();
        let encoded_msg_string: String = encoded_msg_vec.iter().map(|x: &usize| ALPHABET[*x]).collect::<Vec<&str>>().concat();

        println!("{}", encoded_msg_string);
    }

    fn decode(self) -> () {
        let message_values: Vec<usize> = get_text_values(self.clone().message);
        let key_values: Vec<usize> = get_text_values(self.extend_key().key);
        
        if message_values.len() != key_values.len() {
            println!("internal error");
            return (); 
        }

        let dencoded_msg_vec: Vec<usize> = message_values.iter().zip(key_values.iter()).map(|(x, y)| x - y).collect::<Vec<usize>>();
        let dencoded_msg_string: String = dencoded_msg_vec.iter().map(|x: &usize| ALPHABET[*x]).collect::<Vec<&str>>().concat();

        println!("{}", dencoded_msg_string);
    }

    fn extend_key(self) -> Self {
        let mut count: usize = 0;
        let mut extended_key: String = String::new();
        let original_key_vec: Vec<char> = self.key.chars().into_iter().map(|x: char| x).collect::<Vec<char>>();

        for _ in 1..=self.message.len() {
            extended_key = format!("{}{}", extended_key, original_key_vec[count]);
            count += 1;

            if count == original_key_vec.len() {
                count = 0
            }
        } 
        return Self {
            operation: self.operation,
            key: extended_key,
            message: self.message
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
                message: "aaabbbcccdddeee".to_string(),
            };
        assert_eq!(test_instance_1.clone().extend_key(), VigenereCipherAlg {
                                                                    operation: test_instance_1.operation,
                                                                    key: "bananabananaban".to_string(), 
                                                                    message: test_instance_1.message});
    }

    #[test]
    pub fn encode_test() -> () {
        let test_instance_1: VigenereCipherAlg = 
            VigenereCipherAlg {
                operation: crate::Operations::Encode,
                key: "banana".to_string(),
                message: "aaabbbcccdddeee".to_string(),
            };
        println!("{:?}", test_instance_1.clone().encode());
    }

    #[test]
    pub fn decode_test() -> () {
        let test_instance_1: VigenereCipherAlg = 
            VigenereCipherAlg {
                operation: crate::Operations::Encode,
                key: "banana".to_string(),
                message: "BANBOBDCPDQDFER".to_string(),
            };
        println!("{:?}", test_instance_1.clone().decode());
    }
}