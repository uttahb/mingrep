use std::env;
use std::process;
use mingrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!(" error in reading args: {err}");
        process::exit(1);
    });
   

    if let Err(e) = mingrep::run(config) {
        println!("read error: {:?}", e);
    }   
}

