use clap::{Arg, Command};

// TODO: Validate value_parser further
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
                        .value_parser(clap::value_parser!(usize))
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
                        .value_parser(clap::value_parser!(usize))
                        .required(true),
                ),
        )
}
