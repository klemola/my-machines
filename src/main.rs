mod cli;
mod machine_status;
mod watch;

use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    use clap::ArgMatches;
    use cli::get_matches;
    use machine_status::{get_client, list_status, save_status};
    use std::process;

    let table_name = String::from("machine-status");
    let client = get_client();
    let matches = get_matches();

    let status_command = |sub_matches: &ArgMatches| {
        match sub_matches.subcommand_name() {
            Some("list") => match list_status(&client, &table_name) {
                Ok(status_list) => println!("Status list {:?}", status_list),
                Err(error) => println!("Could not list status: {:?}", error),
            },

            Some("save") => match save_status(&client, &table_name) {
                Ok(output) => println!("Status saved {:?}", output),
                Err(error) => println!("Put item error: {:?}", error),
            },

            _ => println!("{:}", sub_matches.usage()),
        }
        Ok(())
    };

    match matches.subcommand() {
        ("watch", _) => watch::start(&client, &table_name),

        ("status", Some(sub_matches)) => status_command(&sub_matches),

        _ => {
            println!("{:}", matches.usage());
            process::exit(0)
        }
    }
}
