use std::process::exit;
use clap::{arg, Command};

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("get", _)) => {
            eprintln!("unimplemented");
            exit(1)
        }
        Some(("set",_)) => {
            eprintln!("unimplemented");
            exit(1)
        }
        Some(("rm",_)) => {
            eprintln!("unimplemented");
            exit(1)
        }
        _ => {}
    }
}

fn cli() -> Command {
    Command::new("kvs")
        .about("about")
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .subcommand(
            Command::new("get")
                .about("get value")
                .arg(arg!(key: [KEY]))
        )
        .subcommand(
            Command::new("set")
                .about("set value")
                .arg(arg!(key: [KEY]))
                .arg(arg!(value: [VALUE]))
        )
        .subcommand(
            Command::new("rm")
                .about("remove value")
                .arg(arg!(key: [KEY]))
        )
}