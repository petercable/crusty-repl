use std::io;
use std::io::prelude::*;

use crate::enums::{Command, HandleResult};
use crate::parser::CommandParser;
use crate::store::Store;
use crate::skweel_store::SkweelStore;

mod enums;
mod parser;
mod store;
mod simple_store;
mod dummy_store;
mod copy_store;
mod skweel_store;

fn main() -> Result<(), io::Error> {
    let parser = CommandParser::new();
    let mut store = SkweelStore::new().unwrap();

    println!("CRUSTY KEY/VALUE STORE");
    prompt()?;
    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?;
        let command = parser.parse_line(&line);
        match command {
            Command::Read(key) => handle_result(store.read(key)),
            Command::Write(key, value) => handle_result(store.write(key, value)),
            Command::Delete(key) => handle_result(store.delete(key)),
            Command::Start => handle_result(store.start()),
            Command::Abort => handle_result(store.abort()),
            Command::Commit => handle_result(store.commit()),
            Command::ParseError(e) => eprintln!("ERROR: {}", e),
            Command::Quit => break,
        }
        prompt()?;
    }
    println!("Exiting...");
    Ok(())
}


fn prompt() -> Result<(), io::Error> {
    print!("> ");
    Ok(io::stdout().flush()?)
}

fn handle_result(result: HandleResult) {
    match result {
        HandleResult::Success => {}
        HandleResult::Result(s) => println!("{}", s),
        HandleResult::Failure(s) => eprintln!("{}", s)
    }
}