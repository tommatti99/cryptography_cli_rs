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

    if chunk.len() > 8 {
        return error_chunk;
    }

    while chunk_x.len() < 8 {
        chunk_x.push(0);
    }
    return chunk_x.try_into().unwrap_or(error_chunk);
}




pub fn is_a_7bit_chunk(chunk: &[u8]) -> [u8; 7] {
    let mut chunk_x: Vec<u8> = chunk.into();
    let error_chunk: [u8; 7] = [0; 7];

    if chunk.len() > 7 {
        return error_chunk;
    }

    while chunk_x.len() < 7 {
        chunk_x.push(0);
    }
    return chunk_x.try_into().unwrap_or(error_chunk);
}

pub fn is_a_xbit_chunk(chunk: &[u8], x_bits_must_have: usize) -> Vec<u8> {
    let mut chunk_x: Vec<u8> = chunk.into();
    let error_chunk: Vec<u8> = {(0..x_bits_must_have).push(0)}

    if chunk.len() > 7 {
        return error_chunk;
    }

    while chunk_x.len() < 7 {
        chunk_x.push(0);
    }
    return chunk_x.try_into().unwrap_or(error_chunk);
}





pub fn is_a_8bytes_block(block: Vec<[u8; 8]>) -> [[u8; 8]; 8] {
    let mut block_x: Vec<[u8; 8]> = block.clone().into();
    let fill_block: [u8; 8] = [0; 8]; 
    let error_block: [[u8; 8]; 8] = [[0; 8]; 8];

    if block.len() > 8 {
        return error_block;
    }
    
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




pub fn byte_parity_verify(byte: [u8; 8]) -> [u8; 8] {
    if byte.iter()
           .map(|x: &u8| { if *x == 1 {1} else {0} })
           .sum::<u32>() % 2 == 0 {

        if byte[7] == 0 {
            return [byte[0], byte[1], byte[2], byte[3], byte[4], byte[5], byte[6], 1];
        }
        return [byte[0], byte[1], byte[2], byte[3], byte[4], byte[5], byte[6], 0];
    }
    return byte;
}




pub fn permutation(block: Vec<u8>, concatenated_permut_table: Vec<u8>) -> Vec<u8> {
    return concatenated_permut_table
            .iter()
            .map(|pos:&u8 | 
                {
                    block[pos.clone() as usize - 1]
                })
            .collect::<Vec<u8>>();
} 




pub fn left_shift_vec(bits_vec: Vec<u8>, shift: usize) -> Vec<u8> {
    let mut shift_vec = bits_vec.clone();

    for _ in 0..shift {
        shift_vec.remove(0);
        shift_vec.push(0);
    }

    return shift_vec;
}



#[cfg(test)]
mod test_ops {
    use crate::tables::{DES_FINAL_PERMUTATION_TABLE, DES_INITIAL_PERMUTATION_TABLE};

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

    #[test]
    fn test_is_a_8bit_chunk() -> () {
        assert_eq!(is_a_8bit_chunk(&[1,0,1]), [1,0,1,0,0,0,0,0]);
        assert_eq!(is_a_8bit_chunk(&[1,0,1,0,0,1]), [1,0,1,0,0,1,0,0]);
        assert_eq!(is_a_8bit_chunk(&[1,0,1,0,0,1,0,0,0]), [0,0,0,0,0,0,0,0]);
    }

    #[test]
    fn test_is_a_7bit_chunk() -> () {
        assert_eq!(is_a_7bit_chunk(&[1,0,1]), [1,0,1,0,0,0,0]);
        assert_eq!(is_a_7bit_chunk(&[1,0,1,0,0,1]), [1,0,1,0,0,1,0]);
        assert_eq!(is_a_7bit_chunk(&[1,0,1,0,0,1,0,0,0]), [0,0,0,0,0,0,0]);
    }
    
    #[test]
    fn test_is_a_8bytes_block() -> () {
        assert_eq!(is_a_8bytes_block(vec![[0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0]]),
       
                                         [[0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0]]
         );

        assert_eq!(is_a_8bytes_block(vec![[0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0]]),
       
                                         [[0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0]]
         );

        assert_eq!(is_a_8bytes_block(vec![[0, 0, 0, 0, 0, 0, 0, 0],
                                          [1, 0, 1, 0, 1, 0, 0, 0],
                                          [1, 0, 1, 0, 1, 0, 0, 0]]),
       
                                         [[0, 0, 0, 0, 0, 0, 0, 0],
                                          [1, 0, 1, 0, 1, 0, 0, 0],
                                          [1, 0, 1, 0, 1, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0],
                                          [0, 0, 0, 0, 0, 0, 0, 0]]
         );
    }
    
    #[test]
    fn test_make_64bits_blocks() -> () {
        assert_eq!(make_64bits_blocks(vec![0,1]), vec![[[0, 1, 0, 0, 0, 0, 0, 0],
                                                        [0, 0, 0, 0, 0, 0, 0, 0],
                                                        [0, 0, 0, 0, 0, 0, 0, 0],
                                                        [0, 0, 0, 0, 0, 0, 0, 0],
                                                        [0, 0, 0, 0, 0, 0, 0, 0],
                                                        [0, 0, 0, 0, 0, 0, 0, 0],
                                                        [0, 0, 0, 0, 0, 0, 0, 0],
                                                        [0, 0, 0, 0, 0, 0, 0, 0]]]
            );

        assert_eq!(make_64bits_blocks(vec![0, 1, 0, 0, 0, 0, 0,
                                           0, 0, 0, 1, 0, 0, 0,
                                           0, 0, 0, 0, 0, 1, 0,
                                           0, 0, 0, 0, 0, 0, 0,
                                           1, 0, 0, 0, 0, 0, 0,
                                           0, 0, 1, 0, 0, 0, 0,
                                           0, 0, 0, 0, 1, 0, 0,
                                           0, 0, 0, 0, 0, 0, 1,
                                           0, 0, 0, 0, 0, 0, 0,
                                           0, 1, 0, 0, 0, 0, 0,
                                           0, 0, 0, 1, 0, 0, 0,
                                           0, 0, 0, 0, 0, 1, 0,
                                           0, 0, 0, 0, 0, 0, 0,
                                           1, 0, 0, 0, 0, 0, 0,
                                           0, 0, 1, 0, 0, 0, 0,
                                           0, 0, 0, 0, 1, 0, 0,
                                           0, 0, 0, 0, 0, 0, 1,
                                           0, 0, 0, 0, 0, 0, 0,
                                           0, 1, 0, 0, 0, 0, 0,
                                           0, 0, 0]), 
                                    
                                    vec![[[0, 1, 0, 0, 0, 0, 0, 0], 
                                          [0, 0, 1, 0, 0, 0, 0, 0], 
                                          [0, 0, 0, 1, 0, 0, 0, 0], 
                                          [0, 0, 0, 0, 1, 0, 0, 0], 
                                          [0, 0, 0, 0, 0, 1, 0, 0], 
                                          [0, 0, 0, 0, 0, 0, 1, 0], 
                                          [0, 0, 0, 0, 0, 0, 0, 1], 
                                          [0, 0, 0, 0, 0, 0, 0, 0]], 

                                         [[1, 0, 0, 0, 0, 0, 0, 0], 
                                          [0, 1, 0, 0, 0, 0, 0, 0], 
                                          [0, 0, 1, 0, 0, 0, 0, 0], 
                                          [0, 0, 0, 1, 0, 0, 0, 0], 
                                          [0, 0, 0, 0, 1, 0, 0, 0], 
                                          [0, 0, 0, 0, 0, 1, 0, 0], 
                                          [0, 0, 0, 0, 0, 0, 1, 0], 
                                          [0, 0, 0, 0, 0, 0, 0, 1]], 

                                         [[0, 0, 0, 0, 0, 0, 0, 0], 
                                          [0, 0, 0, 0, 0, 0, 0, 0], 
                                          [0, 0, 0, 0, 0, 0, 0, 0], 
                                          [0, 0, 0, 0, 0, 0, 0, 0], 
                                          [0, 0, 0, 0, 0, 0, 0, 0], 
                                          [0, 0, 0, 0, 0, 0, 0, 0], 
                                          [0, 0, 0, 0, 0, 0, 0, 0], 
                                          [0, 0, 0, 0, 0, 0, 0, 0]]
        ]);
    }

    #[test]
    fn test_byte_parity_verify() -> () {
        assert_eq!(byte_parity_verify([0,0,0,0,0,0,0,0]), [0,0,0,0,0,0,0,1]);
        assert_eq!(byte_parity_verify([1,1,1,1,1,1,1,1]), [1,1,1,1,1,1,1,0]);
        assert_eq!(byte_parity_verify([1,1,1,1,0,0,0,0]), [1,1,1,1,0,0,0,1]);
        assert_eq!(byte_parity_verify([1,1,1,0,0,0,0,0]), [1,1,1,0,0,0,0,0]);
        assert_eq!(byte_parity_verify([1,1,1,0,0,1,1,0]), [1,1,1,1,0,1,1,0]);
        assert_eq!(byte_parity_verify([1,1,1,1,1,1,1,0]), [1,1,1,1,1,1,1,0]);
    }

    #[test]
    fn test_permutation() -> () {
        assert_eq!(permutation([[0; 8];8].concat(), DES_INITIAL_PERMUTATION_TABLE.concat()), [[0; 8];8].concat());
        assert_eq!(permutation([[1; 8];8].concat(), DES_INITIAL_PERMUTATION_TABLE.concat()), [[1; 8];8].concat());
        let vec_test = make_64bits_blocks((1..=64).collect())[0];
        assert_eq!(permutation(vec_test.concat(), DES_INITIAL_PERMUTATION_TABLE.concat()), DES_INITIAL_PERMUTATION_TABLE.concat());
        assert_eq!(permutation([[0; 8];8].concat(), DES_FINAL_PERMUTATION_TABLE.concat()), [[0; 8];8].concat());
        assert_eq!(permutation([[1; 8];8].concat(), DES_FINAL_PERMUTATION_TABLE.concat()), [[1; 8];8].concat());

        let vec_test = make_64bits_blocks((1..=64).collect())[0];
        assert_eq!(permutation(vec_test.concat(), DES_FINAL_PERMUTATION_TABLE.concat()), DES_FINAL_PERMUTATION_TABLE.concat());
    }

    #[test]
    fn test_left_shift() -> () {
        assert_eq!(left_shift_vec(vec![0, 0, 0, 0, 0, 1, 1, 0], 1), vec![0, 0, 0, 0, 1, 1, 0, 0]);
        assert_eq!(left_shift_vec(vec![1, 0, 1, 0, 1, 0], 10), vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(left_shift_vec(vec![1, 0, 1, 0, 1, 0], 5), vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(left_shift_vec(vec![1, 0, 1, 0, 1, 0], 2), vec![1, 0, 1, 0, 0, 0]);
        assert_eq!(left_shift_vec(vec![1, 0, 1, 0, 1, 0], 1), vec![0, 1, 0, 1, 0, 0]);
        assert_eq!(left_shift_vec(vec![1, 0, 1, 0, 1, 0], 0), vec![1, 0, 1, 0, 1, 0]); 
    }
}
