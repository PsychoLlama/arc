use clap::{App, AppSettings, SubCommand};

fn main() {
    let matches = App::new("arc")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Node version manager")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("ls-remote"))
        .get_matches();

    match matches.subcommand() {
        ("ls-remote", Some(_)) => unimplemented!(),
        _ => unreachable!(),
    }
}
