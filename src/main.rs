use anyhow::Result;
use interpack::util::memory_map::LineByLine;
use interpack::{huffman_decode, huffman_encode};

fn main() -> Result<()> {
    let compressor = huffman_encode::Writer::new("fasta/toy.fa", "toy.fa.hfmn.bin", 67108864);
    let _ = compressor.line_by_line(true);
    // let compressor = huffman_encode::Writer::new(
    //     "fasta/human_g1k_v37_decoy.fasta",
    //     "human_g1k_v37_decoy.fasta.hfmn.bin",
    //     67108864,
    // );
    // let _ = compressor.line_by_line(false);

    let decoder = huffman_decode::Extractor::new("toy.fa.hfmn.bin", 67108864);
    let sub_seq = decoder.access(2);
    println!("\n{sub_seq:?}");

    Ok(())
}

// TODO: Extract information from output binary file
//       Add test codes / CLI

// cargo build --release
// time target/release/interpack
// cargo flamegraph --open --root -- target/release/interpack
