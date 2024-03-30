use anyhow::Result;
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
        let file = File::open(self.filename.as_str()).expect("File not found");
        // Memory map the file
        let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
        let byte_len = mmap.len();

        let mut sub_seq = String::from("");
        let mut packed_byte = 0u8;

        let (mut pos, mut current_num, mut sub_pos) = (1, 1, 1);
        
        let mut two_b_three_b = ("G", "C");
        match mmap[0] {
            239 => two_b_three_b = ("C", "G"),
            175 => {}
            _ => return Err(anyhow::anyhow!("Invalid file to decode")),
        }

        while pos < byte_len {
            let sub_mmap = format!("{:08b}", &mmap[pos]);

            for bit in sub_mmap.chars() {
                let bit_value = match bit {
                    '0' => 0u8,
                    '1' => 1u8,
                    _ => panic!("Invalid character in binary sequence"),
                };

                packed_byte = (packed_byte << 1) | bit_value;
                
                match packed_byte {
                    0 => match sub_pos {
                        1 => sub_pos = 2,
                        2 => {
                            if current_num == seq_num {
                                sub_seq = format!("{}{}", sub_seq, "A");
                            }
                            packed_byte = 0u8;
                            sub_pos = 1;
                        }
                        _ => panic!("Invalid sub_pos"),
                    },
                    1 => match sub_pos {
                        1 => sub_pos = 2,
                        2 => {
                            if current_num == seq_num {
                                sub_seq = format!("{}{}", sub_seq, "T");
                            }
                            packed_byte = 0u8;
                            sub_pos = 1;
                        }
                        _ => panic!("Invalid sub_pos"),
                    },
                    2 => {
                        if current_num == seq_num {
                            sub_seq = format!("{}{}", sub_seq, two_b_three_b.0);
                        }
                        packed_byte = 0u8;
                        sub_pos = 1;
                    }
                    3 => {}
                    6 => {
                        if current_num == seq_num {
                            sub_seq = format!("{}{}", sub_seq, two_b_three_b.1);
                        }
                        packed_byte = 0u8;
                        sub_pos = 1;
                    }
                    7 => {}
                    14 => {
                        if current_num == seq_num {
                            sub_seq = format!("{}{}", sub_seq, "N");
                        }
                        packed_byte = 0u8;
                        sub_pos = 1;
                    }
                    15 => {
                        current_num += 1;
                        packed_byte = 0u8;
                        sub_pos = 1;
                    }
                    _ => return Err(anyhow::anyhow!("Invalid file to decode")),
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
