use clap::{Arg, Command};

pub fn configure() -> Command {
    Command::new("interpack")
        .about("DNA FASTA encoder for compressing raw sequences into searchable binary format")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("encode")
                .about("Encode into searchable binary format")
                .arg(
                    Arg::new("fasta")
                        .short('f')
                        .long("fasta")
                        .help("Specify path to input dna fasta file")
                        .value_parser(clap::value_parser!(String))
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Specify path to output binary file")
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("chunk")
                        .short('c')
                        .long("chunk")
                        .help("Specify chunk size to memory map a file for reading")
                        .value_parser(parse_encode_chunk)
                        .default_value("67108864"),
                )
                .arg(
                    Arg::new("switch")
                        .short('s')
                        .long("switch")
                        .help("Switch 3-bit encoded base from C to G")
                        .value_parser(clap::value_parser!(bool))
                        .default_value("false"),
                )
                .arg(
                    Arg::new("print")
                        .short('p')
                        .long("print")
                        .help("Print raw sequences with its encoding")
                        .value_parser(clap::value_parser!(bool))
                        .default_value("false"),
                ),
        )
        .subcommand(
            Command::new("decode")
                .about("Extract nth sequence from searchable binary format")
                .arg(
                    Arg::new("binary")
                        .short('b')
                        .long("binary")
                        .help("Specify path to input searchable binary file")
                        .value_parser(clap::value_parser!(String))
                        .required(true),
                )
                .arg(
                    Arg::new("number")
                        .short('n')
                        .long("number")
                        .help("Specify nth sequence to extract")
                        .value_parser(parse_decode_number)
                        .required(true),
                )
                .arg(
                    Arg::new("start")
                        .short('s')
                        .long("start")
                        .help("Specify sth base from nth sequence to start")
                        .value_parser(parse_decode_number),
                )
                .arg(
                    Arg::new("end")
                        .short('e')
                        .long("end")
                        .help("Specify eth base from nth sequence to end")
                        .value_parser(parse_decode_number),
                ),
        )
}

fn parse_encode_chunk(val: &str) -> Result<usize, String> {
    let encode_chunk = val
        .parse::<usize>()
        .map_err(|_| "Chunk size must be an unsigned integer".to_string())?;
    if encode_chunk >= 67108864 {
        Ok(encode_chunk)
    } else {
        Err("Chunk size must be at least 67108864 (64MB)".to_string())
    }
}

fn parse_decode_number(val: &str) -> Result<usize, String> {
    let decode_number = val
        .parse::<usize>()
        .map_err(|_| "Sequence number must be an unsigned integer".to_string())?;
    if decode_number >= 1 {
        Ok(decode_number)
    } else {
        Err("Sequence number must be at least 1".to_string())
    }
}
