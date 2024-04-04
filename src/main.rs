use anyhow::Result;
use interpack::{
    cmd::configure,
    err::MyError,
    huffman_decode, huffman_encode,
    util::{cli::Builder, memory_map::LineByLine},
};
use std::path::Path;

fn main() -> Result<()> {
    let cli_builder = Builder::new(configure());
    let matches = cli_builder.get_matches();
    let process = matches.subcommand().unwrap();

    match process.0 {
        "encode" => {
            let encoder = huffman_encode::Writer::new(
                process.1.get_one::<String>("fasta").unwrap().as_str(),
                process
                    .1
                    .get_one::<String>("output")
                    .unwrap_or(&format!(
                        "{}.hfmn.bin",
                        Path::new(process.1.get_one::<String>("fasta").unwrap())
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                    ))
                    .as_str(),
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

// TODO: Add test codes / Handle CLI further

// cargo build --release
// cargo install --path .

// interpack encode -f fasta/toy.fa -o toy.fa.hfmn.bin -p true
// interpack decode -b toy.fa.hfmn.bin -n 2 // Extract second sequence from fasta/toy.fa
// interpack decode -b toy.fa.hfmn.bin -n 2 -s 5 // Extract second sequence from 5th to end
// interpack decode -b toy.fa.hfmn.bin -n 2 -e 10 // Extract second sequence from start to 10th

// time interpack encode -f fasta/human_g1k_v37_decoy.fasta
// time interpack decode -b human_g1k_v37_decoy.fasta.hfmn.bin -n 1 -s 10001 -e 10100 // Extract first sequence from 10001th to 10100th
// time interpack decode -b human_g1k_v37_decoy.fasta.hfmn.bin -n 7 > human_g1k_v37_decoy_seventh_sequence.txt
