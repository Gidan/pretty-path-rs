use std::env;

fn main() {

    match env::var("PATH") {
        Ok(val) => pretty_print(&val),
        Err(e) => println!("couldn't interpret: {e}"),
    }
    
}

fn pretty_print(val : &str) {
    let nval = val.replace(":", "\n");
    println!("{}", nval);

}
