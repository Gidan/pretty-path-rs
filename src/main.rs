use std::env;
use colored::Colorize;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(index = 1)]
    search: Option<String>,
}

fn main() {
    let args = Args::parse();

    match env::var("PATH") {
        Ok(val) => pretty_print(&val, &args),
        Err(e) => println!("couldn't interpret: {e}"),
    }
}

fn pretty_print(val : &str, args : &Args) {
    let mut to_print : &str = val;
    let search = &args.search;
    if search.is_some() {
        let search_val = search.as_ref().unwrap();
        let highlighted = search_val.red().to_string();
        let whole = to_print.replace(search_val.as_str(), highlighted.as_str());
        to_print = &whole.as_str();
        println!("{}", to_print.replace(":", "\n"));
    } else {
        println!("{}", to_print.replace(":", "\n"));
    }
}
