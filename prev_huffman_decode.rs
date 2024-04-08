// use crate::err::MyError;
// use anyhow::Result;
// use memmap2::MmapOptions;
// use std::fs::File;

// pub struct Extractor {
//     filename: String,
// }

// impl Extractor {
//     pub fn new(filename_param: &str) -> Self {
//         Self {
//             filename: filename_param.to_string(),
//         }
//     }

//     pub fn access(&self, seq_num: usize) -> Result<String> {
//         let file = match File::open(self.filename.as_str()) {
//             Ok(file) => file,
//             Err(_) => return Err(MyError::InvalidInputBinary.to_anyhow_error_skip_e()),
//         };
//         // Memory map the file
//         let mmap = unsafe { MmapOptions::new().map(&file)? };
//         let byte_len = mmap.len();

//         // (1+x+1) bytes can handle 4x bases
//         let mut sub_seq = String::with_capacity(byte_len << 2);
//         let mut packed_byte = 0u8;

//         let (mut pos, mut current_num, mut sub_pos) = (1, 1, 1);

//         let mut two_b_three_b = ('G', 'C');
//         match mmap[0] {
//             239 => two_b_three_b = ('C', 'G'),
//             175 => {}
//             _ => return Err(MyError::InvalidFileToDecode.to_anyhow_error_skip_e()),
//         }

//         while pos < byte_len {
//             let sub_mmap = mmap[pos];

//             for bit_pos in (0..8).rev() {
//                 let bit_value = (sub_mmap >> bit_pos) & 1u8;

//                 packed_byte = (packed_byte << 1) | bit_value;

//                 match packed_byte {
//                     0 | 1 => match sub_pos {
//                         1 => sub_pos = 2,
//                         2 => {
//                             let nucleotide = if packed_byte == 0 { 'A' } else { 'T' };
//                             if current_num == seq_num {
//                                 sub_seq.push(nucleotide);
//                             }
//                             packed_byte = 0u8;
//                             sub_pos = 1;
//                         }
//                         _ => return Err(MyError::InvalidSubPos.to_anyhow_error_skip_e()),
//                     },
//                     2 => {
//                         if current_num == seq_num {
//                             sub_seq.push(two_b_three_b.0);
//                         }
//                         packed_byte = 0u8;
//                         sub_pos = 1;
//                     }
//                     3 | 7 => {}
//                     6 => {
//                         if current_num == seq_num {
//                             sub_seq.push(two_b_three_b.1);
//                         }
//                         packed_byte = 0u8;
//                         sub_pos = 1;
//                     }
//                     14 => {
//                         if current_num == seq_num {
//                             sub_seq.push('N');
//                         }
//                         packed_byte = 0u8;
//                         sub_pos = 1;
//                     }
//                     15 => {
//                         current_num += 1;
//                         packed_byte = 0u8;
//                         sub_pos = 1;
//                     }
//                     _ => return Err(MyError::InvalidFileToDecode.to_anyhow_error_skip_e()),
//                 }
//             }

//             if current_num > seq_num {
//                 break;
//             }

//             pos += 1;
//         }

//         Ok(sub_seq)
//     }
// }
