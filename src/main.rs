use anyhow::Result;

use interpack::{
    cmd::configure,
    huffman_decode, huffman_encode,
    util::{cli::Builder, memory_map::LineByLine},
};

fn main() -> Result<()> {
    let cli_builder = Builder::new(configure());
    let matches = cli_builder.get_matches();
    let process = matches.subcommand().unwrap();

    match process.0 {
        "encode" => {
            let encoder = huffman_encode::Writer::new(
                process.1.get_one::<String>("fasta").unwrap().as_str(),
                process.1.get_one::<String>("output").unwrap().as_str(),
                *process.1.get_one::<usize>("chunk").unwrap(),
                *process.1.get_one::<bool>("switch").unwrap(),
            );
            let _ = encoder.line_by_line(*process.1.get_one::<bool>("print").unwrap());
        }
        "decode" => {
            let decoder = huffman_decode::Extractor::new(
                process.1.get_one::<String>("input").unwrap().as_str(),
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

// TODO: Add test codes / CLI (Validate value_parser further / Subcommand isolation / .....) / Further handling

// cargo build --release
// cargo install --path .

// interpack encode -f fasta/toy.fa -o toy.fa.hfmn.bin -p true
// interpack decode -i toy.fa.hfmn.bin -n 2

// time interpack encode -f fasta/human_g1k_v37_decoy.fasta -o human_g1k_v37_decoy.fasta.hfmn.bin
// time interpack decode -i human_g1k_v37_decoy.fasta.hfmn.bin -n 2
