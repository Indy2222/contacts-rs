use anyhow::{bail, Context, Result};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

mod actions;
mod add;
mod contact;
mod init;
mod print;
mod search;

fn main() -> Result<()> {
    let add_cmd = SubCommand::with_name("add").about("Add a new contact.");
    let init_cmd = SubCommand::with_name("init").about("(Re-)initialize contacts storage.");

    let print_cmd = SubCommand::with_name("print").about("Pretty print search matches.");

    let search_cmd = SubCommand::with_name("search")
        .about(
            "Search through contacts with various filters and perform an \
             action on matches. If no filters are specified then all \
             contacts are included.",
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(print_cmd)
        .arg(
            Arg::with_name("full-name")
                .long("full-name")
                .short("n")
                .help(
                    "A regular expression. Contacts whose full name does not \
                     match the regex won't be included in the search output. \
                     Non-person contacts are excluded unless --entity-name is \
                     also provided.",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("entity-name")
                .long("entity-name")
                .short("e")
                .help(
                    "A regular expression. Contacts whose entity name does \
                     not match the regex won't be included in the search \
                     output. Person contacts are excluded unless \
                     --full-name is also provided.",
                )
                .takes_value(true),
        );

    let matches = App::new("con-rs")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Martin Indra <martin.indra@mgn.cz>")
        .about(
            "CLI base contact management tool. \
             See https://github.com/Indy2222/contacts-rs",
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(add_cmd)
        .subcommand(init_cmd)
        .subcommand(search_cmd)
        .get_matches();

    match matches.subcommand() {
        ("add", _) => add::add_contact(),
        ("init", _) => init::init(),
        ("search", Some(matches)) => handle_search(matches),
        _ => panic!("Unrecognized command"),
    }
}

fn handle_search(matches: &ArgMatches) -> Result<()> {
    let mut options = search::SearchOptions::new();
    if let Some(full_name_regex) = matches.value_of("full-name") {
        options
            .set_full_name_regex(full_name_regex)
            .context("Invalid full-name regex.")?;
    }
    if let Some(entity_name_regex) = matches.value_of("entity-name") {
        options
            .set_entity_name_regex(entity_name_regex)
            .context("Invalid entity-name regex.")?;
    }

    let action_subcommand = matches.subcommand();
    let action: Box<dyn actions::MatchAction> = match action_subcommand {
        ("print", _) => Box::new(print::PrintExporter::new()),
        _ => bail!("Invalid export method."),
    };

    search::search(options, action)
}
