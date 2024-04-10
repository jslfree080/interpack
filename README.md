# interpack

[![CI](https://github.com/jslfree080/interpack/actions/workflows/ci.yml/badge.svg)](https://github.com/jslfree080/interpack/actions/workflows/ci.yml)

**interpack** compresses DNA FASTA format (faster than (b)gzip with default compression level on a single CPU core without multithreading) based on the Huffman code described below into interpretable binary format.

```
A: 00
T: 01
G: 10 (C as option)
C: 110 (G as option)
N: 1110 (every ambiguous bases)
-: 1111 (separator between sequences)
```

This project aims to provide an alternative to random access to subsequences by index file, which is dependent on header or SeqID. It outputs a compressed binary format that can solely be used to search the nth subsequence in a non-linear approach using embedded information of byte and bit position pair.

## Install

Install [Rust] and use `cargo` to install `interpack`.

```
$ git clone https://github.com/jslfree080/interpack.git
$ cd interpack
$ cargo build --release
$ cargo install --path . --locked
```

[Rust]: https://www.rust-lang.org/tools/install


## Usage

```
$ interpack -h

interpack 0.1.0
Jung Soo Lee <jslfree080@gmail.com>
DNA FASTA encoder for compressing raw sequences into searchable binary format

Usage: interpack [COMMAND]

Commands:
  encode  Encode into searchable binary format
  decode  Extract nth sequence from searchable binary format
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

interpack has two subcommands: `encode` and `decode`.

### `encode`

```
$ interpack encode -h
Encode into searchable binary format

Usage: interpack encode [OPTIONS] --fasta <fasta>

Options:
  -f, --fasta <fasta>    Specify path to input dna fasta file
  -o, --output <output>  Specify directory to output searchable binary file
  -c, --chunk <chunk>    Specify chunk size to memory map a file for reading [default: 67108864]
  -s, --switch <switch>  Switch 3-bit encoded base from C to G [default: false] [possible values: true, false]
  -p, --print <print>    Print raw sequences with its encoding [default: false] [possible values: true, false]
  -h, --help             Print help
```

The output is a binary file whose name ends with '.hfmn.bin'. This file alone
is used for the subcommand 'decode' below.

### `decode`

```
$ interpack decode -h
Extract nth sequence from searchable binary format

Usage: interpack decode [OPTIONS] --binary <binary> --number <number>

Options:
  -b, --binary <binary>  Specify path to input searchable binary file
  -n, --number <number>  Specify nth sequence to extract
  -s, --start <start>    Specify sth base from nth sequence to start
  -e, --end <end>        Specify eth base from nth sequence to end
  -h, --help             Print help
```

This prints out the nth subsequence with a given range (sth base to eth base).
The range can be omitted.

## interpack Examples

### human_g1k_v37.fasta (3.15GB)

```
$ wget -qO - https://ftp.ncbi.nlm.nih.gov/1000genomes/ftp/technical/reference/human_g1k_v37.fasta.gz | gunzip -c > human_g1k_v37.fasta

$ printf "%s bytes\n" "$(stat -f %z human_g1k_v37.fasta)"
3153506519 bytes

$ time interpack encode -f human_g1k_v37.fasta

real	5m18.340s
user	1m42.438s
sys	3m34.595s

$ printf "%s bytes\n" "$(stat -f %z human_g1k_v37.fasta.hfmn.bin)"
907920668 bytes

$ time interpack decode -b human_g1k_v37.fasta.hfmn.bin -n 1 -s 10001 -e 10100
TAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAAC

real	0m3.492s
user	0m2.875s
sys	0m0.171s

$ time interpack decode -b human_g1k_v37.fasta.hfmn.bin -n 42 -s 10001 -e 10100
ATGCCATACAACTCATTAAGTGTACAAGTCAATTATTTTTTATAAACTTACAAAGTTGTGCAGATATCACTACAATTTAATTTTAGAACATTTCTATCAC

real	0m0.011s
user	0m0.004s
sys	0m0.005s

$ time interpack decode -b human_g1k_v37.fasta.hfmn.bin -n 84 -s 10001 -e 10100
TACTGTTTCCATGGGGTACAACCCCTTCTTCCTCCTGAAACACATTATTCCTCTGGCCCGCTGTTGCCAGAGACACTGAGTCTTGTCTTTGGATAAGTTC

real	0m0.023s
user	0m0.016s
sys	0m0.006s
```

### hs37d5.fa (3.19GB)

```
$ wget -qO - https://ftp.ncbi.nlm.nih.gov/1000genomes/ftp/technical/reference/phase2_reference_assembly_sequence/hs37d5.fa.gz | gunzip -c > hs37d5.fa

$ printf "%s bytes\n" "$(stat -f %z hs37d5.fa)"
3189750467 bytes

$ time interpack encode -f hs37d5.fa
            
real	6m31.059s
user	1m49.875s
sys	4m6.886s

$ printf "%s bytes\n" "$(stat -f %z hs37d5.fa.hfmn.bin)"
917757163 bytes

$ time interpack decode -b hs37d5.fa.hfmn.bin -n 1 -s 10001 -e 10100
TAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAAC

real	0m3.660s
user	0m2.868s
sys	0m0.194s

$ time interpack decode -b hs37d5.fa.hfmn.bin -n 43 -s 10001 -e 10100
CTTGGGGAAACTGGAGCAAGATAGAATACATTTTTTCTCTTTGTCTCAACATCAGGGCTCTAAGTATATTGTAAAAGTGTAGCACCTTTTCTTCTCCAGG

real	0m0.011s
user	0m0.004s
sys	0m0.004s

$ time interpack decode -b hs37d5.fa.hfmn.bin -n 86 -s 10001 -e 10100
AATCTTCATGGGACTCTGTGCCCAAGAAAACCAGCTTGACAGGATTGGGCACAAGAGCTATTCCTTCCATCCTACGCTGCCAGAGCACACAGCACAGTTT

real	0m0.472s
user	0m0.406s
sys	0m0.028s
```

### GCA_000001405.15_GRCh38_no_alt_analysis_set.fna (3.15GB)

```
$ wget -qO - https://ftp.ncbi.nlm.nih.gov/genomes/all/GCA/000/001/405/GCA_000001405.15_GRCh38/seqs_for_alignment_pipelines.ucsc_ids/GCA_000001405.15_GRCh38_no_alt_analysis_set.fna.gz | gunzip -c > GCA_000001405.15_GRCh38_no_alt_analysis_set.fna

$ printf "%s bytes\n" "$(stat -f %z GCA_000001405.15_GRCh38_no_alt_analysis_set.fna)"
3144230986 bytes

$ time interpack encode -f GCA_000001405.15_GRCh38_no_alt_analysis_set.fna

real	5m38.299s
user	1m40.262s
sys	3m26.329s

$ printf "%s bytes\n" "$(stat -f %z GCA_000001405.15_GRCh38_no_alt_analysis_set.fna.hfmn.bin)"
891079627 bytes

$ time interpack decode -b GCA_000001405.15_GRCh38_no_alt_analysis_set.fna.hfmn.bin -n 1 -s 10001 -e 10100
TAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAACCCTAAC

real	0m3.725s
user	0m2.938s
sys	0m0.191s

$ time interpack decode -b GCA_000001405.15_GRCh38_no_alt_analysis_set.fna.hfmn.bin -n 97 -s 10001 -e 10100
CCTATCTTTTGATTAAGCCGTTTTGAAACTCTCTTTTTGTAGTATCTGCAAGTGGATATTTACAGCCTTTTGTGGCCTGTTGTGGAAAAGGAAATGCCTT

real	0m0.013s
user	0m0.006s
sys	0m0.005s

$ time interpack decode -b GCA_000001405.15_GRCh38_no_alt_analysis_set.fna.hfmn.bin -n 195 -s 10001 -e 10100
AAAGACCCTACGGCTCCGCCTGCGCAGGTGCCACAGGTTCCTGCCGTGTGAGAACAAGAGTAAAGCTGTGGAACAGATAAAAAATGCCTTTAACAAGCTG

real	0m0.014s
user	0m0.007s
sys	0m0.004s
```

### fasta/toy.fa (toy example)

```
$ printf "%s bytes\n" "$(stat -f %z fasta/toy.fa)"
104 bytes

$ interpack encode -f fasta/toy.fa -p true
">ref"
"AGCATGTTAGATAAGATAGCTGTGCTAGTAGGCAGTCAGCGCCAT"
00101100001100101001000010000100001001011001100110110010010010010101100010011100010110101101100001
">ref2"
"aggttttataaaacaattaagtctacagagcaactacgcgAAAAAA"
001010010101010001000000001100000010100001001110010011000100010110000011001001101011010000000000000


$ printf "%s bytes\n" "$(stat -f %z toy.fa.hfmn.bin)"
33 bytes

$ interpack decode -b toy.fa.hfmn.bin -n 2
AGGTTTTATAAAACAATTAAGTCTACAGAGCAACTACGCGAAAAAA

$ interpack decode -b toy.fa.hfmn.bin -n 2 -s 5
TTTATAAAACAATTAAGTCTACAGAGCAACTACGCGAAAAAA

$ interpack decode -b toy.fa.hfmn.bin -n 2 -e 10
AGGTTTTATA
```

## Limitations

  * In cases where header or SeqID information is necessary, using index 
    files other than interpack would be suitable.
  * When the file size is large, the compression rate is generally lower 
    than (b)gzip (tens of megabytes difference for three gigabytes FASTA
    format), probably because interpack does not substitute repetitive
    sequences into shorter code like LZ compression.
  * interpack does not offer support for multithreading or multi-core CPU
    functionality.
  * The encoding scheme does not consider small letter bases (ex. a -> A). 
    In addition, any ambiguous bases (ex. M) other than N is treated as N.
