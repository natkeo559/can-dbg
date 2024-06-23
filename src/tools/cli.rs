use std::path::PathBuf;

#[derive(Debug)]
pub struct J1939;

#[derive(Debug)]
pub struct Can;

#[derive(Debug)]
pub struct A;

#[derive(Debug)]
pub struct B;

pub trait IsProtocol {}
pub trait IsLogFormat {}

impl IsProtocol for J1939 {}
impl IsProtocol for Can {}

impl IsLogFormat for A {}
impl IsLogFormat for B {}

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Protocol {
    Can,
    J1939,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    Dec,
    Hex,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OutputFormatId {
    Dec,
    Hex,
    Decoded,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputFormatData {
    Compact,
    Spaced,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OutputFormatMsg {
    DecodedLe,
    DecodedBe,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CandumpFormat {
    /// can0 FFF [8] FF FF FF FF FF FF FF FF
    A,
    /// (0000000000.000000) can0 FFFFFFFF#FFFFFFFFFFFFFFFF
    B,
}

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Decode arbitration ID, data fields, or candump messages
    #[command(arg_required_else_help = true)]
    Decode(DecodeArgs),
    /// Generate statistics for a candump file
    #[command(arg_required_else_help = true)]
    Stat(StatArgs),
}

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub struct DecodeArgs {
    #[command(subcommand)]
    pub command: DecodeCommands,
}

#[derive(Subcommand, Debug)]
pub enum DecodeCommands {
    /// Decode arbitration ID
    #[command(arg_required_else_help = true)]
    Id(DecodeIdArgs),
    /// Decode data field
    #[command(arg_required_else_help = true)]
    Data(DecodeDataArgs),
    /// Decode a CAN message/frame
    #[command(arg_required_else_help = true)]
    Msg(DecodeMsgArgs),
    /// Decode a file
    #[command(arg_required_else_help = true)]
    File(DecodeFileArgs),
}

#[derive(Args, Debug)]
pub struct DecodeIdArgs {
    /// Specifies the protocol type
    #[arg(
        value_enum,
        name = "proto",
        long,
        short,
        value_name = "PROTOCOL",
        default_value = "j1939"
    )]
    pub protocol: Protocol,

    /// Specifies the input base for numerical values
    #[arg(
        value_enum,
        name = "base",
        long,
        short,
        value_name = "BASE",
        default_value = "hex"
    )]
    pub input_base: Base,

    #[arg(value_enum, long, default_value = "decoded")]
    pub output_format: OutputFormatId,

    /// Specifies the input to be decoded
    #[arg(name = "input", short, long, value_name = "INPUT", required = true)]
    pub input_value: String,
}

#[derive(Args, Debug)]
#[command(flatten_help = true)]
pub struct DecodeDataArgs {
    /// Specifies the input base for numerical values
    #[arg(
        value_enum,
        name = "base",
        long,
        short,
        value_name = "BASE",
        default_value = "hex"
    )]
    pub input_base: Base,

    /// [byte formats]
    ///  - [compact] FFFFFFFF
    ///  - [spaced]  FF FF FF FF
    #[arg(value_enum, long, default_value = "compact", verbatim_doc_comment)]
    pub input_format: InputFormatData,

    /// Specifies the input to be decoded
    #[arg(name = "input", short, long, value_name = "INPUT", required = true)]
    pub input_value: String,
}

#[derive(Args, Debug)]
#[command(flatten_help = true)]
pub struct DecodeMsgArgs {
    /// Specifies the protocol type
    #[arg(
        value_enum,
        name = "proto",
        long,
        short,
        value_name = "PROTOCOL",
        default_value = "j1939"
    )]
    pub protocol: Protocol,

    /// [candump formats]
    ///  - [a] can0 FFF [8] FF FF FF FF FF FF FF FF
    ///  - [b] (0000000000.000000) can0 FFFFFFFF#FFFFFFFFFFFFFFFF
    #[arg(
        value_enum,
        long,
        value_name = "INPUT-FORMAT",
        default_value = "b",
        verbatim_doc_comment
    )]
    pub input_format: CandumpFormat,

    /// Specifies the input to be decoded
    #[arg(name = "input", short, long, required = true)]
    pub input_value: String,
}

#[derive(Args, Debug)]
#[command(flatten_help = true)]
pub struct DecodeFileArgs {
    /// Specifies the protocol type
    #[arg(
        value_enum,
        name = "proto",
        long,
        short,
        value_name = "PROTOCOL",
        default_value = "j1939"
    )]
    pub protocol: Protocol,

    #[arg(short = 'f', long, value_name = "FILEPATH")]
    pub input_file: PathBuf,

    /// [candump formats]
    ///  - [a] can0 FFF [8] FF FF FF FF FF FF FF FF
    ///  - [b] (0000000000.000000) can0 FFFFFFFF#FFFFFFFFFFFFFFFF
    #[arg(
        value_enum,
        long,
        value_name = "INPUT-FORMAT",
        default_value = "b",
        verbatim_doc_comment
    )]
    pub input_format: CandumpFormat,

    #[arg(short, long, value_name = "FILEPATH")]
    pub output_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct StatArgs {
    #[command(subcommand)]
    pub command: StatCommands,
}

#[derive(Subcommand, Debug)]
pub enum StatCommands {
    /// Show statistics for a whole candump file with additional metadata
    #[command(arg_required_else_help = true)]
    File(StatFileArgs),
    /// Show statistics for a specific data field, id, or message in a file
    #[command(arg_required_else_help = true)]
    Spec,
}

#[derive(Args, Debug)]
#[command(flatten_help = true)]
pub struct StatFileArgs {
    /// Specifies the protocol type
    #[arg(
        value_enum,
        name = "proto",
        long,
        short,
        value_name = "PROTOCOL",
        default_value = "j1939"
    )]
    pub protocol: Protocol,

    /// [candump formats]
    ///  - [a] can0 FFF [8] FF FF FF FF FF FF FF FF
    ///  - [b] (0000000000.000000) can0 FFFFFFFF#FFFFFFFFFFFFFFFF
    #[arg(
        value_enum,
        long,
        value_name = "INPUT-FORMAT",
        default_value = "b",
        verbatim_doc_comment
    )]
    pub input_format: CandumpFormat,

    #[arg(short = 'f', long, value_name = "FILEPATH")]
    pub input_file: PathBuf,

    /// Shows stats for arbitration IDs in the log file
    #[arg(short, long)]
    pub id_stats: bool,
}
