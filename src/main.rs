use clap::Parser;
use crate::app::{App, handle};
use exitcode::{DATAERR, OK};

mod app;
mod commands;
mod constants;
mod game;

fn main() {
    let app = App::parse();

    match handle(app) {
        Ok(output) => {
            println!("{}", output);
            std::process::exit(OK);
        },
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(DATAERR);
        } 
    }
}
