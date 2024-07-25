use std::env::current_dir;
use std::process::exit;
use clap::{arg, Command};
use kvs::{KvStore, Result};

fn main() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("get", args)) => {
            let key = args.get_one::<String>("key").expect("");
            let mut kv_store = KvStore::open(current_dir()?)?;
            if let Some(value) = kv_store.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Some(("set", args)) => {
            let key = args.get_one::<String>("key").expect("");
            let value = args.get_one::<String>("value").expect("");
            let mut kv_store = KvStore::open(current_dir()?)?;
            kv_store.set(key.into(), value.into())?;
        }
        Some(("rm", args)) => {
            let key = args.get_one::<String>("key").expect("");
            let mut kv_store = KvStore::open(current_dir()?)?;
            if let Ok(()) = kv_store.remove(key.into()) {

            } else {
                print!("Key not found");
                exit(1)
            }
        }
        _ => {}
    }

    Ok(())
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