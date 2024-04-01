use clap::{Arg, Command};

// TODO: Validate value_parser further
//       Subcommand isolation
pub fn configure() -> Command {
    Command::new("interpack")
        .about(
            "DNA FASTA encoder for compressing raw sequences into searchable binary format",
        )
        .arg_required_else_help(true)
        .subcommand(
            Command::new("code")
                .about("Encode into searchable binary format")
                .arg(
                    Arg::new("fasta")
                        .short('f')
                        .long("fasta")
                        .help("Specify the fasta name")
                        .value_parser(clap::value_parser!(String))
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Specify the output binary name")
                        .value_parser(clap::value_parser!(String)),
                )
                .arg(
                    Arg::new("chunk")
                        .short('c')
                        .long("chunk")
                        .help("Specify chunk")
                        .value_parser(clap::value_parser!(usize))
                        .default_value("67108864"),
                )
                .arg(
                    Arg::new("switch")
                        .short('s')
                        .long("switch")
                        .help("Switch the 3-bit encoded base from C to G")
                        .value_parser(clap::value_parser!(bool))
                        .default_value("false"),
                )
                .arg(
                    Arg::new("print")
                        .short('p')
                        .long("print")
                        .help("Print the encoding")
                        .value_parser(clap::value_parser!(bool))
                        .default_value("false"),
                ),
        )
        .subcommand(
            Command::new("decode")
                .about("Search for nth sequence from searchable binary file")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .help("Specify searchable binary file")
                        .value_parser(clap::value_parser!(String))
                        .required(true),
                )
                .arg(
                    Arg::new("number")
                        .short('n')
                        .long("number")
                        .help("Specify nth sequence")
                        .value_parser(clap::value_parser!(usize))
                        .required(true),
                ),
        )
}
