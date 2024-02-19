use clap::Parser;
use kv::config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    scheduler: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    addr: u8,
    path: String,
    loglevel: String,
}

fn main() {
    let args = Args::parse();
    let mut config = config::Config::default();

    if args.path!="" {
        config.d
    }
}
