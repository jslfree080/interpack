use anyhow::Result;
use interpack::{
    cmd::configure,
    err::MyError,
    huffman_decode, huffman_encode,
    util::{cli::Builder, memory_map::LineByLine},
};
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let cli_builder = Builder::new(configure());
    let matches = cli_builder.get_matches();
    let process = matches.subcommand().unwrap();

    match process.0 {
        "encode" => {
            let fasta_name = Path::new(process.1.get_one::<String>("fasta").unwrap())
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();

            let output_str = match process.1.get_one::<String>("output") {
                Some(val) => match Path::new(val).exists() {
                    true => {
                        let mut path = PathBuf::new();
                        path.push(val);
                        path.push(fasta_name);
                        path.to_str().unwrap().to_string()
                    }
                    false => return Err(MyError::InvalidDirectory.to_anyhow_error_skip_e()),
                },
                None => fasta_name.to_string(),
            };

            let encoder = huffman_encode::Writer::new(
                process.1.get_one::<String>("fasta").unwrap().as_str(),
                output_str.as_str(),
                *process.1.get_one::<usize>("chunk").unwrap(),
                *process.1.get_one::<bool>("switch").unwrap(),
            );
            let _ = encoder.line_by_line(*process.1.get_one::<bool>("print").unwrap());
            if *process.1.get_one::<bool>("print").unwrap() as bool {
                println!()
            }
        }
        "decode" => {
            let decoder = huffman_decode::Extractor::new(
                process.1.get_one::<String>("binary").unwrap().as_str(),
            );
            let sub_seq = decoder.access(*process.1.get_one::<usize>("number").unwrap())?;

            let start_base = process.1.get_one::<usize>("start");
            let end_base = process.1.get_one::<usize>("end");
            match (start_base, end_base) {
                (Some(val_s), Some(val_e)) => {
                    if val_s > val_e {
                        return Err(MyError::InvalidDecodeRange.to_anyhow_error_skip_e());
                    }
                    if *val_e > sub_seq.len() {
                        return Err(MyError::InvalidDecodeEnd.to_anyhow_error_skip_e());
                    }
                    let sub_seq_range = &sub_seq[(val_s - 1)..=(val_e - 1)];
                    println!("{}", sub_seq_range)
                }
                (Some(val_s), None) => {
                    if *val_s > sub_seq.len() {
                        return Err(MyError::InvalidDecodeStart.to_anyhow_error_skip_e());
                    }
                    let sub_seq_range = &sub_seq[(val_s - 1)..=(sub_seq.len() - 1)];
                    println!("{}", sub_seq_range)
                }
                (None, Some(val_e)) => {
                    if *val_e > sub_seq.len() {
                        return Err(MyError::InvalidDecodeEnd.to_anyhow_error_skip_e());
                    }
                    let sub_seq_range = &sub_seq[0..=(val_e - 1)];
                    println!("{}", sub_seq_range)
                }
                (None, None) => println!("{}", sub_seq),
            }
        }
        _ => return Err(MyError::InvalidProcessZero.to_anyhow_error()),
    }

    Ok(())
}
