use std::path::PathBuf;

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
pub enum OutputFormatData {
    BytesLe,
    BytesBe,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OutputFormatMsg {
    DecodedLe,
    DecodedBe,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CandumpFormat {
    /// can0 00000000 [8] FF FF FF FF FF FF FF FF
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
pub struct DecodeArgs {
    #[command(subcommand)]
    pub command: DecodeCommands,
}

#[derive(Subcommand, Debug)]
pub enum DecodeCommands {
    /// Decode arbitration ID
    #[command(arg_required_else_help = true)]
    Id(IdArgs),
    /// Decode data field
    #[command(arg_required_else_help = true)]
    Data(DataArgs),
    /// Decode a CAN message/frame
    #[command(arg_required_else_help = true)]
    Msg(MsgArgs),
    /// Decode a file
    #[command(arg_required_else_help = true)]
    File(FileArgs),
}

#[derive(Args, Debug)]
pub struct IdArgs {
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
pub struct DataArgs {
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

    #[arg(value_enum, short, long, default_value = "bytes-le")]
    pub output_format: OutputFormatData,

    /// Specifies the input to be decoded
    #[arg(name = "input", short, long, value_name = "INPUT", required = true)]
    pub input_value: String,
}

#[derive(Args, Debug)]
#[command(flatten_help = true)]
pub struct MsgArgs {
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
    ///  - [a] can0 00000000 [8] FF FF FF FF FF FF FF FF
    ///  - [b] (0000000000.000000) can0 FFFFFFFF#FFFFFFFFFFFFFFFF
    #[arg(
        value_enum,
        long,
        value_name = "INPUT-FORMAT",
        default_value = "b",
        verbatim_doc_comment
    )]
    pub input_format: CandumpFormat,

    #[arg(
        value_enum,
        name = "output-format",
        short,
        long,
        default_value = "decoded-le"
    )]
    pub output_format: OutputFormatMsg,

    /// Specifies the input to be decoded
    #[arg(name = "input", short, long, required = true)]
    pub input_value: String,
}

#[derive(Args, Debug)]
#[command(flatten_help = true)]
pub struct FileArgs {
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
    ///  - [a] can0 00000000 [8] FF FF FF FF FF FF FF FF
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
#[command(flatten_help = true)]
pub struct StatArgs {}
