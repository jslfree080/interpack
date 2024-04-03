use anyhow::Result;
use interpack::{
    cmd::configure,
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
            println!("{}", sub_seq);
        }
        _ => {
            return Err(anyhow::anyhow!("Invalid process.0"));
        }
    }

    Ok(())
}

// TODO: Add test codes / Handle CLI further / Error handling

// cargo build --release
// cargo install --path .

// interpack encode -f fasta/toy.fa -o toy.fa.hfmn.bin -p true
// interpack decode -b toy.fa.hfmn.bin -n 2 // Extract second sequence from fasta/toy.fa

// time interpack encode -f fasta/human_g1k_v37_decoy.fasta
// time interpack decode -b human_g1k_v37_decoy.fasta.hfmn.bin -n 7 > human_g1k_v37_decoy_seventh_sequence.txt
