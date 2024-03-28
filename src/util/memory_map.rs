use anyhow::Result;
use memmap2::MmapOptions;
use std::fs::File;

pub fn line_string(line: &[u8]) -> String {
    String::from_utf8_lossy(line).to_string()
}

pub trait LineByLine {
    fn line_by_line(&self, print: bool) -> Result<()>;
}

pub struct Reader {
    filename: String,
    chunk: usize,
}

impl Reader {
    pub fn new(filename_param: &str, chunk_param: usize) -> Self {
        Self {
            filename: filename_param.to_string(),
            chunk: chunk_param,
        }
    }
}

impl LineByLine for Reader {
    #[allow(unused_variables)]
    fn line_by_line(&self, print: bool) -> Result<()> {
        let file = File::open(self.filename.as_str()).expect("File not found");
        let len = file.metadata()?.len();
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
                let line_str = line_string(line);
                println!("{line_str:?}");
            }

            // Move to the next line
            pos += next_newline as u64;
            line_number += 1;
        }

        Ok(())
    }
}
