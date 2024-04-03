use anyhow::{bail, Result};
use memmap2::MmapOptions;
use std::fs::File;

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
        let file =
            File::open(self.filename.as_str()).expect("Check path to input searchable binary file");
        // Memory map the file
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let byte_len = mmap.len();

        let mut sub_seq = String::with_capacity(byte_len * 4);
        let mut packed_byte = 0u8;

        let (mut pos, mut current_num, mut sub_pos) = (1, 1, 1);

        let mut two_b_three_b = ('G', 'C');
        match mmap[0] {
            239 => two_b_three_b = ('C', 'G'),
            175 => {}
            _ => panic!("Invalid file to decode"),
        }

        while pos < byte_len {
            let sub_mmap = format!("{:08b}", &mmap[pos]);

            for bit in sub_mmap.chars() {
                let bit_value = match bit {
                    '0' => 0u8,
                    '1' => 1u8,
                    _ => bail!("Invalid character in binary sequence"),
                };

                packed_byte = (packed_byte << 1) | bit_value;

                match packed_byte {
                    0 => match sub_pos {
                        1 => sub_pos = 2,
                        2 => {
                            if current_num == seq_num {
                                sub_seq.push('A');
                            }
                            packed_byte = 0u8;
                            sub_pos = 1;
                        }
                        _ => bail!("Invalid sub_pos"),
                    },
                    1 => match sub_pos {
                        1 => sub_pos = 2,
                        2 => {
                            if current_num == seq_num {
                                sub_seq.push('T');
                            }
                            packed_byte = 0u8;
                            sub_pos = 1;
                        }
                        _ => bail!("Invalid sub_pos"),
                    },
                    2 => {
                        if current_num == seq_num {
                            sub_seq.push(two_b_three_b.0);
                        }
                        packed_byte = 0u8;
                        sub_pos = 1;
                    }
                    3 => {}
                    6 => {
                        if current_num == seq_num {
                            sub_seq.push(two_b_three_b.1);
                        }
                        packed_byte = 0u8;
                        sub_pos = 1;
                    }
                    7 => {}
                    14 => {
                        if current_num == seq_num {
                            sub_seq.push('N');
                        }
                        packed_byte = 0u8;
                        sub_pos = 1;
                    }
                    15 => {
                        current_num += 1;
                        packed_byte = 0u8;
                        sub_pos = 1;
                    }
                    _ => panic!("Invalid file to decode"),
                }
            }

            if current_num > seq_num {
                break;
            }

            pos += 1;
        }

        Ok(sub_seq)
    }
}
