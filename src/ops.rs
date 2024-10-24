use clap::builder::Str;

fn bits_to_byte(bits: &[u8; 8]) -> u8 {
    let mut byte: u8 = 0;

    for (i, &bit) in bits.iter().enumerate() {
        byte |= bit << (7 - i); 
    }
    return byte;
}

fn byte_to_bits(byte: u8) -> [u8; 8] {
    let mut bits: [u8; 8] = [0; 8]; 

    for i in 0..8 {
        bits[i] = (byte >> (7 - i)) & 1;
    }
    return bits;
}

pub fn char_to_bits(c: char) -> Vec<[u8; 8]> {
    let bits: Vec<[u8; 8]> = 
        c.to_string()
         .as_bytes()
         .iter()
         .map(|byte| byte_to_bits(*byte))
         .collect();

    return bits;
}


pub fn bits_to_char(bits: Vec<[u8; 8]>) -> char {
    let bytes: Vec<u8> = bits.iter().map(|bits: &[u8; 8]| bits_to_byte(bits)).collect();
    
    return String::from_utf8(bytes).unwrap_or(" ".to_string()).chars().nth(0).unwrap_or(' ');
}

pub fn string_into_bits(message: String) -> Vec<u8> {
    let vec_chars_bits: Vec<Vec<[u8; 8]>> = message.chars().map(|c: char| char_to_bits(c)).collect();
    
    return vec_chars_bits.concat().concat();
}

pub fn bits_into_string(bits: Vec<u8>) -> String {
    let bytes: Vec<u8> = bits
                    .chunks(8)
                    .map(|bits_i: &[u8]| {
                        bits_to_byte(&is_a_8bit_chunk(bits_i)
                        
                        )}
                     ).collect();

    return String::from_utf8(bytes).unwrap_or(" ".to_string());
}

pub fn is_a_8bit_chunk(chunk: &[u8]) -> [u8; 8] {
    let mut chunk_x: Vec<u8> = chunk.into();
    let error_chunk: [u8; 8] = [0; 8];

    while chunk_x.len() < 8 {
        chunk_x.push(0);
    }
    return chunk_x.try_into().unwrap_or(error_chunk);
}

pub fn is_a_8bytes_block(block: Vec<[u8; 8]>) -> [[u8; 8]; 8] {
    let mut block_x: Vec<[u8; 8]> = block.into();
    let fill_block: [u8; 8] = [0; 8]; 
    let error_block: [[u8; 8]; 8] = [[0; 8]; 8];

    while block_x.len() < 8 {
        block_x.push(fill_block);
    }

    return block_x.try_into().unwrap_or(error_block);
}

pub fn make_64bits_blocks(bits: Vec<u8>) -> Vec<[[u8; 8]; 8]> {
    let bytes: Vec<[u8; 8]> = bits
        .chunks(8)
        .map(|chunk: &[u8]| is_a_8bit_chunk(chunk))
        .collect();

    let blocks_vec: Vec<[[u8; 8]; 8]> = bytes
        .chunks(8)
        .map(|block: &[[u8; 8]]| is_a_8bytes_block(block.to_vec()))
        .collect();

    return blocks_vec;
}

#[cfg(test)]
mod test_ops {
    use super::*;
    
    #[test]
    fn test_bit_to_byte() -> () {
        assert_eq!(bits_to_byte(&[0,0,0,0,0,0,0,1]), 1);
        assert_eq!(bits_to_byte(&[1,1,1,1,1,1,1,1]), 255);
        assert_eq!(bits_to_byte(&[0,1,0,0,1,1,0,0]), 76);
    }

    #[test]
    fn test_byte_to_bit() -> () {
        assert_eq!(byte_to_bits(1),  [0,0,0,0,0,0,0,1]);
        assert_eq!(byte_to_bits(255),[1,1,1,1,1,1,1,1]);
        assert_eq!(byte_to_bits(76), [0,1,0,0,1,1,0,0]);
    }

    #[test]
    fn test_char_to_bits() -> () {
        assert_eq!(char_to_bits('A'), vec![[0,1,0,0,0,0,0,1]]);
        assert_eq!(char_to_bits('@'), vec![[0,1,0,0,0,0,0,0]]);
        assert_eq!(char_to_bits('~'), vec![[0,1,1,1,1,1,1,0]]);
        assert_eq!(char_to_bits('±'), vec![[1,1,0,0,0,0,1,0], [1,0,1,1,0,0,0,1]]);
        assert_eq!(char_to_bits('æ'), vec![[1,1,0,0,0,0,1,1], [1,0,1,0,0,1,1,0]]);
        assert_eq!(char_to_bits('»'), vec![[1,1,0,0,0,0,1,0], [1,0,1,1,1,0,1,1]]);
    }
    
    #[test]
    fn test_bits_to_char() -> () {
        assert_eq!(bits_to_char(vec![[0,1,0,0,0,0,0,1]]), 'A');
        assert_eq!(bits_to_char(vec![[0,1,0,0,1,0,1,1]]), 'K');
        assert_eq!(bits_to_char(vec![[0,1,0,1,1,0,1,1]]), '[');
        assert_eq!(bits_to_char(vec![[1,1,0,0,0,0,1,1], [1,0,0,1,0,0,0,0]]), 'Ð');
        assert_eq!(bits_to_char(vec![[1,1,0,0,0,0,1,1], [1,0,0,1,1,0,0,0]]), 'Ø');
        assert_eq!(bits_to_char(vec![[1,1,0,0,0,0,1,1], [1,0,1,1,0,0,0,0]]), 'ð');
    }
    
    #[test]
    fn test_string_into_bits() -> () {
        assert_eq!(string_into_bits("ALOALO".to_string()), vec![0,1,0,0,0,0,0,1,0,1,0,0,1,1,0,0,0,1,0,0,1,1,1,1,0,1,0,0,0,0,0,1,0,1,0,0,1,1,0,0,0,1,0,0,1,1,1,1]);
        assert_eq!(string_into_bits("AL MOSSAR".to_string()), vec![0,1,0,0,0,0,0,1,0,1,0,0,1,1,0,0,0,0,1,0,0,0,0,0,0,1,0,0,1,1,0,1,0,1,0,0,1,1,1,1,0,1,0,1,0,0,1,1,0,1,0,1,0,0,1,1,0,1,0,0,0,0,0,1,0,1,0,1,0,0,1,0]);
        assert_eq!(string_into_bits("× × ×Ð".to_string()), vec![1,1,0,0,0,0,1,1,1,0,0,1,0,1,1,1,0,0,1,0,0,0,0,0,1,1,0,0,0,0,1,1,1,0,0,1,0,1,1,1,0,0,1,0,0,0,0,0,1,1,0,0,0,0,1,1,1,0,0,1,0,1,1,1,1,1,0,0,0,0,1,1,1,0,0,1,0,0,0,0]);
    }

    #[test]
    fn test_bits_into_string() -> () {
        assert_eq!(bits_into_string(vec![0,1,0,0,0,0,0,1]),"A".to_string());
        assert_eq!(bits_into_string(vec![0,1,0,0,0,0,0,1,0,1,0,0,1,1,0,0,0,1,0,0,1,1,1,1,0,1,0,0,0,0,0,1,0,1,0,0,1,1,0,0,0,1,0,0,1,1,1,1]),"ALOALO".to_string());
        assert_eq!(bits_into_string(vec![0,1,1,0,0,0,0,1,0,1,1,0,0,0,1,0,0,1,1,0,0,0,1,1,1,1,0,0,0,0,1,1,1,0,0,0,1,1,0,1]),"abcÍ".to_string());
        assert_eq!(bits_into_string(vec![0,1,0,0,0,0,0,1,0,1,0,0,1,1,0,0,0,0,1,0,0,0,0,0,0,1,0,0,1,1,0,1,0,1,0,0,1,1,1,1,0,1,0,1,0,0,1,1,0,1,0,1,0,0,1,1,0,1,0,0,0,0,0,1,0,1,0,1,0,0,1,0]),"AL MOSSAR".to_string());
        assert_eq!(bits_into_string(vec![1,1,0,0,0,0,1,1,1,0,0,1,0,1,1,1,0,0,1,0,0,0,0,0,1,1,0,0,0,0,1,1,1,0,0,1,0,1,1,1,0,0,1,0,0,0,0,0,1,1,0,0,0,0,1,1,1,0,0,1,0,1,1,1,1,1,0,0,0,0,1,1,1,0,0,1,0,0,0,0]),"× × ×Ð".to_string());
    }
}