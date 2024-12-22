mod serial;
mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long)]
    action: String,
}

fn main() {
    let args = Args::parse();

    match args.action.as_str() {
        "calculate" => utils::pair_up(),
        "start" => {
            utils::new_entry("start".to_string());
            println!("Start time added")
        }
        "stop" => {
            utils::new_entry("stop".to_string());
            println!("Stop time added")
        }
        _ => println!("Need to choose: calculate, start, or stop"),
    }
}
