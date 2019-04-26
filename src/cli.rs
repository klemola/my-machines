use clap::{App, ArgMatches, SubCommand};

pub fn get_matches() -> ArgMatches<'static> {
    App::new("Butsku CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Setup and monitor your personal infrastructure")
        .subcommand(
            SubCommand::with_name("status")
                .about("Get or set machine status")
                .subcommand(SubCommand::with_name("list").about("List all registered statuses"))
                .subcommand(
                    SubCommand::with_name("save").about("Save status from current machine"),
                ),
        )
        .subcommand(
            SubCommand::with_name("watch").about("Keep track of machine status and save updates"),
        )
        .get_matches()
}
