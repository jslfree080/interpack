use crate::err::MyError;
use anyhow::Result;
use memmap2::MmapOptions;
use std::{collections::BTreeMap, fs::File};

pub struct Extractor {
    filename: String,
}

impl Extractor {
    pub fn new(filename_param: &str) -> Self {
        Self {
            filename: filename_param.to_string(),
        }
    }

    pub fn access(&self, seq_num: usize) -> Result<String> {
        let file = match File::open(self.filename.as_str()) {
            Ok(file) => file,
            Err(_) => return Err(MyError::InvalidInputBinary.to_anyhow_error_skip_e()),
        };
        // Memory map the file
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let byte_len = mmap.len();

        // (1+x+...) bytes can handle 4x bases
        let mut sub_seq = String::with_capacity(byte_len << 2);
        let mut packed_byte = 0u8;

        let mut sub_pos = 1;

        let mut two_b_three_b = ('G', 'C');
        match mmap[0] {
            239 => two_b_three_b = ('C', 'G'),
            175 => {}
            _ => return Err(MyError::InvalidFileToDecode.to_anyhow_error_skip_e()),
        }

        if (mmap[byte_len - 1] < 1) || (mmap[byte_len - 1] > 8) {
            return Err(MyError::InvalidFileToDecode.to_anyhow_error_skip_e());
        }
        let (
            mut pbc_remaining_bits,
            mut exp,
            mut pbc_io_insert,
            mut rb_to_insert,
            mut index_reader,
        ) = (BTreeMap::<usize, u8>::new(), 0, 0usize, 8u8, byte_len - 1);
        loop {
            if mmap[index_reader] == 255 {
                pbc_remaining_bits.insert(pbc_io_insert, rb_to_insert);
                break;
            }

            if (mmap[index_reader] >= 1) && (mmap[index_reader] <= 8) {
                if index_reader < byte_len - 1 {
                    pbc_remaining_bits.insert(pbc_io_insert, rb_to_insert);
                    exp = 0;
                    pbc_io_insert = 0usize;
                }
                rb_to_insert = mmap[index_reader];
            }

            if (mmap[index_reader] >= 48) && (mmap[index_reader] <= 57) {
                pbc_io_insert += (mmap[index_reader] - 48) as usize * 10usize.pow(exp);
                exp += 1;
            }

            index_reader -= 1;
        }

        if seq_num > pbc_remaining_bits.len() {
            return Err(MyError::InvalidSequenceNumber.to_anyhow_error_skip_e());
        }
        let (mut pbc_rb_pos, mut pbc_start_pos, mut rb_start_bit_pos) = (1, 1, 8u8);
        for (pbc, rb) in pbc_remaining_bits {
            if pbc_rb_pos == seq_num {
                (pbc_start_pos, rb_start_bit_pos) = (pbc, rb);
                break;
            }

            pbc_rb_pos += 1;
        }

        let mut count_1111 = 1;

        let mut pbc_pos = pbc_start_pos;
        let mut bit_pos_range = (0..rb_start_bit_pos).rev();

        while pbc_pos < byte_len {
            let sub_mmap = mmap[pbc_pos];

            for bit_pos in bit_pos_range {
                let bit_value = (sub_mmap >> bit_pos) & 1u8;

                packed_byte = (packed_byte << 1) | bit_value;

                match packed_byte {
                    0 | 1 => match sub_pos {
                        1 => sub_pos = 2,
                        2 => {
                            let nucleotide = if packed_byte == 0 { 'A' } else { 'T' };
                            if count_1111 == 1 {
                                sub_seq.push(nucleotide);
                            }
                            packed_byte = 0u8;
                            sub_pos = 1;
                        }
                        _ => return Err(MyError::InvalidSubPos.to_anyhow_error_skip_e()),
                    },
                    2 => {
                        if count_1111 == 1 {
                            sub_seq.push(two_b_three_b.0);
                        }
                        packed_byte = 0u8;
                        sub_pos = 1;
                    }
                    3 | 7 => {}
                    6 => {
                        if count_1111 == 1 {
                            sub_seq.push(two_b_three_b.1);
                        }
                        packed_byte = 0u8;
                        sub_pos = 1;
                    }
                    14 => {
                        if count_1111 == 1 {
                            sub_seq.push('N');
                        }
                        packed_byte = 0u8;
                        sub_pos = 1;
                    }
                    15 => {
                        count_1111 += 1;
                        packed_byte = 0u8;
                        sub_pos = 1;
                    }
                    _ => return Err(MyError::InvalidFileToDecode.to_anyhow_error_skip_e()),
                }
            }

            if count_1111 == 2 {
                break;
            }

            pbc_pos += 1;
            bit_pos_range = (0..8).rev();
        }

        Ok(sub_seq)
    }
}
