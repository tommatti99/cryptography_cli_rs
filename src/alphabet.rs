pub const ALPHABET: [&str; 26] = ["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];
pub const ALPHABET_LEN: i8 = 26;

pub fn get_letter_position(letter: &str) -> usize {
    return ALPHABET.iter().position(|x: &&str| *x == letter.to_uppercase()).unwrap().try_into().unwrap();
}

pub fn get_text_values(text: String) -> Vec<usize> {
    return
        text.chars()
            .map(|x:char| get_letter_position(x.to_string().as_str()))
            .collect();
}

pub fn get_letter_bin_value(letter: &str) -> Vec<u8> {
    return letter.to_ascii_uppercase().to_string().as_bytes().to_vec();
}

pub fn get_text_bin_value(text: String) -> Vec<Vec<u8>> {
    return 
        text.chars()
            .map(|x:char| x.to_string().as_bytes().to_vec())
            .collect();
}




#[cfg(test)]
mod alphabet_test {
    use crate::algorithms::vigenere_cipher::VigenereCipherAlg;

    use super::*;

    #[test]
    pub fn get_letter_position_test() -> () {
        
    }


    #[test]
    pub fn get_text_values_test() -> () {
        let test_instance_1: VigenereCipherAlg = 
            VigenereCipherAlg {
                operation: crate::Operations::Encrypt,
                key: "banana".to_string(),
                message: "aaaaaa".to_string(),
            };
        assert_eq!(get_text_values(test_instance_1.key), vec![1, 0, 13, 0, 13, 0]);
    }

}