use std::env;
use colored::Colorize;

fn main() {
    println!("Hello {}", "World".bright_yellow().bold());
    match env::var("PATH") {
        Ok(val) => pretty_print(&val),
        Err(e) => println!("couldn't interpret: {e}"),
    }
    
}

fn pretty_print(val : &str) {
    let args: Vec<String> = env::args().collect();
    let mut toPrint : &str = val;
    if args.len() > 1 {
        let search = &args[1].as_str();
        let highlighted = search.red().to_string();
        let whole = toPrint.replace(search, highlighted.as_str());
        toPrint = whole.as_str();
        println!("{}", toPrint.replace(":", "\n"));
    } else {
        println!("{}", toPrint.replace(":", "\n"));
    }

}
