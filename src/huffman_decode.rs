pub struct Extractor {
    filename: String,
    chunk: usize,
}

impl Extractor {
    pub fn new(filename_param: &str, chunk_param: usize) -> Self {
        Self {
            filename: filename_param.to_string(),
            chunk: chunk_param,
        }
    }

    pub fn access(&self, seq_num: usize) -> &str {
        "TODO: Extract information from output binary file"
    }
}
