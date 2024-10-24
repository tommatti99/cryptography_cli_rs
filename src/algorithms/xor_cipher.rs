use clap::Args;
use crate::{alphabet::get_text_bin_value, Operations};
use rand::{Rng, distributions::Alphanumeric};

#[derive(Debug, Args, Clone, PartialEq)]
pub struct XorCipherAlg {
    /// Encode or Decode Operation
    #[arg(short, long)]
    pub operation: Operations,
    /// The secret word for encrypt
    #[arg(short, long)]
    pub key: Option<String>,
    /// The message to encode or decode
    #[arg(short, long)]
    pub message: String,
}
impl XorCipherAlg {
    pub fn execute(self) -> () {
        let mut rand_i = rand::thread_rng(); 
        let key_len: usize; 
        let mut rand_key: String = String::new();
        
        if self.key.clone().is_some() {
            rand_key = self.key.clone().unwrap();
        }

        if self.key.is_none() {
            key_len = rand_i.gen();

            rand_key = {rand_i
                .sample_iter(&Alphanumeric)
                .take(key_len)
                .map(|x: u8| char::from(x).to_string())
                .collect::<Vec<String>>()
                .concat()
            };
        }

        let new_key_instance = XorCipherAlg {
            key: Some(rand_key),
            operation: self.operation,
            message: self.message
        };

        match new_key_instance.operation {
            Operations::Encrypt => {
                new_key_instance.encode_decode();
            }
            Operations::Decrypt => {
                new_key_instance.encode_decode();
            }
        }
    }

    fn encode_decode(self) -> () {
        let bin_values_message: Vec<Vec<u8>> = get_text_bin_value(self.clone().extend_key().key.unwrap()); 
        let bin_values_key: Vec<Vec<u8>> = get_text_bin_value(self.message);

        let encoded_message_bytes: Vec<Vec<u8>> = 
            bin_values_message.iter()
                .zip(bin_values_key.iter())
                .map(|(vec_x, vec_y)| {
                    
                    vec_x.iter()
                    .zip(vec_y.iter())
                    .map(|(x, y)| {x ^ y})
                    .collect::<Vec<u8>>()}

                ).collect::<Vec<Vec<u8>>>();
        
        let encoded_message: String = encoded_message_bytes.iter().map(|x: &Vec<u8>| {String::from_utf8(x.clone()).unwrap()}).collect::<String>();
        
        println!("{:?}", encoded_message);
    }

    fn extend_key(self) -> Self {
        let mut count: usize = 0;
        let mut extended_key: String = String::new();
        let original_key_vec: Vec<char> = self.key.unwrap().chars().into_iter().map(|x: char| x).collect::<Vec<char>>();

        for _ in 1..=self.message.len() {
            extended_key = format!("{}{}", extended_key, original_key_vec[count]);
            count += 1;

            if count == original_key_vec.len() {
                count = 0
            }
        } 
        return Self {
            operation: self.operation,
            key: Some(extended_key),
            message: self.message
        };
    }
}

#[cfg(test)]
mod xor_cipher_test {
    use super::*;

    #[test]
    fn xor_cipher_encode_test() -> () {
        let instance_1: XorCipherAlg = 
            XorCipherAlg {
                operation: Operations::Decrypt ,
                key: Some("AAAAAAAAAAAA".to_string()),
                message: "- 3.8$".to_string()
            };

            let instance_2: XorCipherAlg = 
            XorCipherAlg {
                operation: Operations::Encrypt ,
                key: Some("AAAAAAAAAAAA".to_string()),
                message: "laroye".to_string()
            };
        
        println!("{:?}", instance_1.execute());
        println!("{:?}", instance_2.execute());
    }
}