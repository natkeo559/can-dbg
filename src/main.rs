use clap::Parser;
use lib::cli::{Cli, Commands, DecodeCommands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Decode(decode_args) => match &decode_args.command {
            DecodeCommands::Id(_id_args) => {}
            DecodeCommands::Data(_data_args) => {}
            DecodeCommands::Msg(_msg_args) => {}
            DecodeCommands::File(_file_args) => {}
        },
        Commands::Stat(_stat_args) => {}
    }
}
