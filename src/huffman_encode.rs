use crate::util::memory_map::{self, LineByLine};
use anyhow::Result;
use memmap2::MmapOptions;
use std::fs::{self, File};
use std::io::{BufWriter, Write};

pub struct Writer {
    input: String,
    output: String,
    chunk: usize,
}

impl Writer {
    pub fn new(input_param: &str, output_param: &str, chunk_param: usize) -> Self {
        Self {
            input: input_param.to_string(),
            output: output_param.to_string(),
            chunk: chunk_param,
        }
    }
}

impl LineByLine for Writer {
    fn line_by_line(&self, print: bool) -> Result<()> {
        let file = File::open(self.input.as_str()).expect("File not found");
        let len = file.metadata()?.len();

        let output_file = File::create(self.output.as_str())?;
        // Using BufWriter with less frequency of manual flushing mitigate the performance overhead
        let mut buffered_output_file = BufWriter::new(output_file);
        let (mut packed_byte, mut remaining_bits) = (0u8, 8u8);

        // Read the file line by line
        let (mut pos, mut line_number) = (0, 1);

        while pos < len {
            // Calculate the size of the chunk to map
            let remaining_bytes = len - pos;
            let chunk = std::cmp::min(self.chunk, remaining_bytes as usize);

            // Memory map the file
            let mmap = unsafe { MmapOptions::new().len(chunk).offset(pos).map(&file)? };

            // Find the end of the current line
            let next_newline = mmap
                .iter()
                .position(|&c| c == b'\n')
                .map(|i| i + 1)
                .unwrap_or(chunk);

            let line = &mmap[..next_newline - 1]; // Exclude the newline character

            // Process the line
            if print {
                let line_str = memory_map::line_string(line);
                println!("{line_str:?}");
            }

            // Interpretation: Sequence separate with 0b1111
            //                 Last sequence separate with 0b1 / 0b11 / ... / 0b11111111

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
                        return Err(anyhow::anyhow!(
                            "Invalid character {} in line{}: {}",
                            *elm_byte as char,
                            line_number,
                            String::from_utf8_lossy(line)
                        ));
                    }

                    if print {
                        print!(
                            "{}",
                            format_args!(
                                "{:0width$b}",
                                // (0b00u8 * (!((*elm_byte >> 2) & 1u8) & !((*elm_byte >> 1) & 1u8) & 1u8))
                                //     +
                                (0b110u8
                                    * ((((*elm_byte >> 2) & 1u8) ^ ((*elm_byte >> 1) & 1u8))
                                        & (*elm_byte & 1u8)
                                        & 1u8))
                                    + (0b10u8 * (((*elm_byte >> 2) & 1u8) & (*elm_byte & 1u8) & 1u8))
                                    + (!(*elm_byte & 1u8) & !((*elm_byte >> 3) & 1u8) & 1u8) // (0b01u8 * (!(*elm_byte & 1u8) & !((*elm_byte >> 3) & 1u8) & 1u8))
                                    + (0b1110u8 * (((*elm_byte >> 3) & 1u8) & 1u8)),
                                width = match *elm_byte {
                                    b'a' | b'A' | b'g' | b'G' | b't' | b'T' => 2,
                                    b'c' | b'C' => 3,
                                    b'n' | b'N' => 4,
                                    _ => 0,
                                }
                            )
                        );
                    }

                    for bit in format!(
                        "{:0width$b}",
                        // (0b00u8 * (!((*elm_byte >> 2) & 1u8) & !((*elm_byte >> 1) & 1u8) & 1u8))
                        //     +
                        (0b110u8
                            * ((((*elm_byte >> 2) & 1u8) ^ ((*elm_byte >> 1) & 1u8))
                                & (*elm_byte & 1u8)
                                & 1u8))
                            + (0b10u8 * (((*elm_byte >> 2) & 1u8) & (*elm_byte & 1u8) & 1u8))
                            + (!(*elm_byte & 1u8) & !((*elm_byte >> 3) & 1u8) & 1u8) // (0b01u8 * (!(*elm_byte & 1u8) & !((*elm_byte >> 3) & 1u8) & 1u8))
                            + (0b1110u8 * (((*elm_byte >> 3) & 1u8) & 1u8)),
                        width = match *elm_byte {
                            b'a' | b'A' | b'g' | b'G' | b't' | b'T' => 2,
                            b'c' | b'C' => 3,
                            b'n' | b'N' => 4,
                            _ => 0,
                        }
                    )
                    .chars()
                    {
                        let bit_value = match bit {
                            '0' => 0u8,
                            '1' => 1u8,
                            _ => panic!("Invalid character in binary sequence"),
                        };

                        packed_byte = (packed_byte << 1) | bit_value;
                        remaining_bits -= 1u8;

                        if remaining_bits == 0u8 {
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

            if line[0] == b'>' {
                for _ in 0..4 {
                    packed_byte = (packed_byte << 1) | 1u8;
                    remaining_bits -= 1u8;

                    if remaining_bits == 0u8 {
                        buffered_output_file.write_all(&[packed_byte])?;
                        // Manual flushing
                        buffered_output_file.flush()?;
                        packed_byte = 0u8;
                        remaining_bits = 8u8;
                    }
                }
            }

            // Move to the next line
            pos += next_newline as u64;
            line_number += 1;
        }

        for _ in 0..8 {
            packed_byte = (packed_byte << 1) | 1u8;
            remaining_bits -= 1u8;

            if remaining_bits == 0u8 {
                buffered_output_file.write_all(&[packed_byte])?;
                // Manual flushing
                buffered_output_file.flush()?;
                packed_byte = 0u8;
                remaining_bits = 8u8;
            }
        }

        Ok(())
    }
}
