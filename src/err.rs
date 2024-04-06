use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Invalid character {0} in line {1}: {2}")]
    InvalidCharacter(char, usize, String),

    #[error("Check path to input dna fasta file")]
    InvalidInputFasta,

    #[error("Check path to input searchable binary file")]
    InvalidInputBinary,

    #[error("Invalid file to decode")]
    InvalidFileToDecode,

    #[error("Invalid sub_pos")]
    InvalidSubPos,

    #[error("Invalid process.0")]
    InvalidProcessZero,

    #[error("Start position should be smaller than end position")]
    InvalidDecodeRange,

    #[error("Start position should not be larger than length of nth sequence")]
    InvalidDecodeStart,

    #[error("End position should not be larger than length of nth sequence")]
    InvalidDecodeEnd,

    #[error("No such directory exists")]
    InvalidDirectory,
}

impl MyError {
    pub fn to_anyhow_error(&self) -> anyhow::Error {
        let error_message = self.to_string();
        eprintln!("Error: {}", error_message);
        anyhow::Error::msg(error_message)
    }

    pub fn to_anyhow_error_skip_e(&self) -> anyhow::Error {
        let error_message = self.to_string();
        anyhow::Error::msg(error_message)
    }
}
