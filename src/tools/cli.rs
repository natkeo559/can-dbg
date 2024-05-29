use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

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
#[command(flatten_help = true)]
pub struct DecodeArgs {
    #[command(subcommand)]
    pub command: DecodeCommands,
}

#[derive(Args, Debug)]
#[command(flatten_help = true)]
pub struct IdArgs {
    #[arg(value_enum, name = "type", short = 't', long, default_value = "ext")]
    pub id_type: IdType,

    #[arg(value_enum, name = "base", short, long, default_value = "hex")]
    pub input_base: Base,

    #[arg(
        value_enum,
        name = "output-format",
        short,
        long,
        default_value = "parts"
    )]
    pub output_id_format: OutputFormatId,

    #[arg(name = "input", short, long, required = true)]
    pub id: String,
}

#[derive(Args, Debug)]
#[command(flatten_help = true)]
pub struct DataArgs {
    #[arg(value_enum, name = "base", short, long, default_value = "hex")]
    pub input_base: Base,

    #[arg(
        value_enum,
        name = "output-format",
        short,
        long,
        default_value = "bytes-le"
    )]
    pub output_data_format: OutputFormatData,

    #[arg(name = "input", short, long, required = true)]
    pub data: String,
}

#[derive(Args, Debug)]
#[command(flatten_help = true)]
pub struct MsgArgs {
    #[arg(value_enum, name = "type", short = 't', long, default_value = "ext")]
    pub id_type: IdType,

    #[arg(value_enum, name = "base", short, long, default_value = "hex")]
    pub input_base: Base,

    #[arg(
        value_enum,
        name = "output-format",
        short,
        long,
        default_value = "parts-le"
    )]
    pub output_msg_format: OutputFormatMsg,

    #[arg(short, long, default_value = "#")]
    pub sep: char,

    #[arg(name = "input", short, long, required = true)]
    pub message: String,
}

#[derive(Args, Debug)]
#[command(flatten_help = true)]
pub struct FileArgs {
    #[arg(short, long)]
    filepath: PathBuf,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IdType {
    Std,
    Ext,
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
    Parts,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OutputFormatData {
    BytesLe,
    BytesBe,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OutputFormatMsg {
    DecLe,
    HexLe,
    PartsLe,
    DecBe,
    HexBe,
    PartsBe,
}
