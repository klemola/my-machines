mod cli;
mod machine_status;
mod models;
mod watch;

use clap::ArgMatches;
use cli::get_matches;
use machine_status::{get_client, list_and_handle_result, save_and_handle_result};
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let table_name = String::from("machine-status");
    let client = get_client();
    let matches = get_matches();

    let status_command = |sub_matches: &ArgMatches| match sub_matches.subcommand_name() {
        Some("list") => list_and_handle_result(&client, &table_name),

        Some("save") => save_and_handle_result(&client, &table_name),

        _ => println!("{:}", sub_matches.usage()),
    };

    match matches.subcommand() {
        ("watch", _) => watch::start(&client, &table_name),

        ("status", Some(sub_matches)) => {
            status_command(&sub_matches);
            Ok(())
        }

        _ => {
            println!("{:}", matches.usage());
            Ok(())
        }
    }
}
