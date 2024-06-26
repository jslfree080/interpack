use crate::{
    err::MyError,
    util::memory_map::{self, LineByLine},
};
use anyhow::Result;
use memmap2::MmapOptions;
use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{BufWriter, Write},
};

pub struct Writer {
    input: String,
    output: String,
    chunk: usize,
    g_is_three_bit: bool,
}

impl Writer {
    pub fn new(
        input_param: &str,
        output_param: &str,
        chunk_param: usize,
        g_is_three_bit_param: bool,
    ) -> Self {
        Self {
            input: input_param.to_string(),
            output: output_param.to_string(),
            chunk: chunk_param,
            g_is_three_bit: g_is_three_bit_param,
        }
    }
}

impl LineByLine for Writer {
    fn line_by_line(&self, print: bool) -> Result<()> {
        let file = match File::open(self.input.as_str()) {
            Ok(file) => file,
            Err(_) => return Err(MyError::InvalidInputFasta.to_anyhow_error()),
        };
        let len = file.metadata()?.len();

        let output_file = File::create(format!("{}.hfmn.bin", self.output).as_str())?;
        // Using BufWriter with less frequency of manual flushing mitigate the performance overhead
        let mut buffered_output_file = BufWriter::new(output_file);
        let (mut packed_byte, mut remaining_bits) = (0u8, 8u8);

        // Read the file line by line
        let (mut pos, mut line_number) = (0, 1);

        let c_g_encode_pair = match self.g_is_three_bit {
            true => (0b10u8, 0b110u8),
            false => (0b110u8, 0b10u8),
        };

        // Enables random access for decode
        let (mut packed_byte_count, mut pbc_remaining_bits) =
            (1usize, BTreeMap::<usize, u8>::new());

        while pos < len {
            // Calculate the size of the chunk to map
            let remaining_bytes = len - pos;
            let chunk = std::cmp::min(self.chunk, remaining_bytes as usize);

            // Memory map the file
            let mmap = unsafe { MmapOptions::new().len(chunk).offset(pos).map(&file)? };

            // Find the end of the current line
            let next_newline = mmap
                .iter()
                .position(|&c| c == b'\n' || c == b'\r') // Check for both '\n' and '\r'
                .map(|i| {
                    if mmap.get(i) == Some(&b'\r') && mmap.get(i + 1) == Some(&b'\n') {
                        i + 2 // If '\r\n', skip both characters
                    } else {
                        i + 1 // If '\n' or '\r' only, skip the newline character
                    }
                })
                .unwrap_or(chunk);

            // Extract the line, excluding the newline character(s)
            let line = if let Some(&b'\r') = mmap.get(next_newline - 2) {
                &mmap[..next_newline - 2] // If '\r\n', exclude both '\r' and '\n'
            } else {
                &mmap[..next_newline - 1] // If '\n' or '\r' only, exclude only the newline character
            };

            // Process the line
            if print {
                let line_str = memory_map::line_string(line);
                println!("{line_str:?}");
            }

            if !line.is_empty() && line[0] != b'>' {
                for mut elm_byte in line {
                    match elm_byte {
                        // Ambiguous codes commonly used in DNA sequences
                        // R: Represents "A or G" (purine)
                        // Y: Represents "C or T" (pyrimidine)
                        // W: Represents "A or T" (weak)
                        // S: Represents "C or G" (strong)
                        // K: Represents "G or T" (keto)
                        // M: Represents "A or C" (amino)
                        // H: Represents "A or C or T" (not G)
                        // B: Represents "C or G or T" (not A)
                        // V: Represents "A or C or G" (not T)
                        // D: Represents "A or G or T" (not C)
                        b'R' | b'r' | b'Y' | b'y' | b'W' | b'w' | b'S' | b's' | b'K' | b'k'
                        | b'M' | b'm' | b'H' | b'h' | b'B' | b'b' | b'V' | b'v' | b'D' | b'd' => {
                            elm_byte = &b'N'
                        }
                        _ => {}
                    }

                    if !(matches!(
                        elm_byte,
                        b'a' | b'A' | b'c' | b'C' | b'g' | b'G' | b't' | b'T' | b'n' | b'N'
                    )) {
                        fs::remove_file(self.output.as_str())?;
                        return Err(MyError::InvalidCharacter(
                            *elm_byte as char,
                            line_number,
                            String::from_utf8_lossy(line).to_string(),
                        )
                        .to_anyhow_error());
                    }

                    if print {
                        print!(
                            "{}",
                            format_args!(
                                "{:0width$b}",
                                // (0b00u8 * (!((*elm_byte >> 2) & 1u8) & !((*elm_byte >> 1) & 1u8) & 1u8))
                                //     |
                                (c_g_encode_pair.0
                                    * ((((*elm_byte >> 2) & 1u8) ^ ((*elm_byte >> 1) & 1u8))
                                        & (*elm_byte & 1u8)
                                        & 1u8))
                                    | (c_g_encode_pair.1 * (((*elm_byte >> 2) & 1u8) & (*elm_byte & 1u8) & 1u8))
                                    | (!(*elm_byte & 1u8) & !((*elm_byte >> 3) & 1u8) & 1u8) // (0b01u8 * (!(*elm_byte & 1u8) & !((*elm_byte >> 3) & 1u8) & 1u8))
                                    | (0b1110u8 * (((*elm_byte >> 3) & 1u8) & 1u8)),
                                width = match (self.g_is_three_bit, *elm_byte) {
                                    (true, b'a' | b'A' | b'c' | b'C' | b't' | b'T') => 2,
                                    (true, b'g' | b'G') => 3,
                                    (true, b'n' | b'N') => 4,
                                    (false, b'a' | b'A' | b'g' | b'G' | b't' | b'T') => 2,
                                    (false, b'c' | b'C') => 3,
                                    (false, b'n' | b'N') => 4,
                                    _ => 0,
                                }
                            )
                        );
                    }

                    let encoded = (c_g_encode_pair.0
                        * ((((*elm_byte >> 2) & 1u8) ^ ((*elm_byte >> 1) & 1u8))
                            & (*elm_byte & 1u8)
                            & 1u8))
                        | (c_g_encode_pair.1 * (((*elm_byte >> 2) & 1u8) & (*elm_byte & 1u8) & 1u8))
                        | (!(*elm_byte & 1u8) & !((*elm_byte >> 3) & 1u8) & 1u8) // (0b01u8 * (!(*elm_byte & 1u8) & !((*elm_byte >> 3) & 1u8) & 1u8))
                        | (0b1110u8 * (((*elm_byte >> 3) & 1u8) & 1u8));
                    // | (0b00u8 * (!((*elm_byte >> 2) & 1u8) & !((*elm_byte >> 1) & 1u8) & 1u8))

                    let width = match (self.g_is_three_bit, *elm_byte) {
                        (true, b'a' | b'A' | b'c' | b'C' | b't' | b'T') => 2,
                        (true, b'g' | b'G') => 3,
                        (true, b'n' | b'N') => 4,
                        (false, b'a' | b'A' | b'g' | b'G' | b't' | b'T') => 2,
                        (false, b'c' | b'C') => 3,
                        (false, b'n' | b'N') => 4,
                        _ => 0,
                    };

                    for bit_pos in (0..width).rev() {
                        let bit_value = (encoded >> bit_pos) & 1u8;

                        packed_byte = (packed_byte << 1) | bit_value;
                        remaining_bits -= 1u8;

                        if remaining_bits == 0u8 {
                            packed_byte_count += 1;
                            buffered_output_file.write_all(&[packed_byte])?;
                            packed_byte = 0u8;
                            remaining_bits = 8u8;
                        }
                    }
                }

                if print {
                    println!();
                }
            }

            if !line.is_empty() && line[0] == b'>' {
                if line_number == 1 {
                    match self.g_is_three_bit {
                        true => buffered_output_file.write_all(&[0b11101111u8])?,
                        false => buffered_output_file.write_all(&[0b10101111u8])?,
                    }
                    // Manual flushing
                    buffered_output_file.flush()?;
                } else {
                    // Sequence separate with 0b1111
                    for _ in 0..4 {
                        packed_byte = (packed_byte << 1) | 1u8;
                        remaining_bits -= 1u8;

                        if remaining_bits == 0u8 {
                            packed_byte_count += 1;
                            buffered_output_file.write_all(&[packed_byte])?;
                            // Manual flushing
                            buffered_output_file.flush()?;
                            packed_byte = 0u8;
                            remaining_bits = 8u8;
                        }
                    }
                }

                pbc_remaining_bits.insert(packed_byte_count, remaining_bits);
            }

            // Move to the next line
            pos += next_newline as u64;
            line_number += 1;
        }

        // Last sequence separate with 0b1 / 0b11 / ... / 0b11111111
        for _ in 0..8 {
            packed_byte = (packed_byte << 1) | 1u8;
            remaining_bits -= 1u8;

            if remaining_bits == 0u8 {
                packed_byte_count += 1;
                buffered_output_file.write_all(&[packed_byte])?;
                // Manual flushing
                buffered_output_file.flush()?;
                packed_byte = 0u8;
                remaining_bits = 8u8;
            }
        }

        // Separation between sequence and index with 0b11111111
        buffered_output_file.write_all(&[0b11111111u8])?;
        // Manual flushing
        buffered_output_file.flush()?;

        // Enables random access for decode
        for (pbc, rb) in &pbc_remaining_bits {
            buffered_output_file.write_all(pbc.to_string().as_bytes())?;
            buffered_output_file.write_all(&[*rb])?;
            // Manual flushing
            buffered_output_file.flush()?;
        }

        Ok(())
    }
}
