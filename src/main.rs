use pig_lating::run;
use std::process;

fn main() {

    if let Err(e) = run(){
        eprintln!("{e}");
        process::exit(1);
    }
}