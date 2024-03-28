use anyhow::Result;
use interpack::huffman_encode;
use interpack::util::memory_map::LineByLine;

fn main() -> Result<()> {
    let compressor = huffman_encode::Writer::new("fasta/toy.fa", "toy.fa.hfmn.bin", 67108864);
    let _ = compressor.line_by_line(true);
    Ok(())
}

// TODO: Extract information from output binary file
//       Add test codes / CLI
// cargo build --release
// time target/release/interpack
// cargo flamegraph --open --root -- target/release/interpack
