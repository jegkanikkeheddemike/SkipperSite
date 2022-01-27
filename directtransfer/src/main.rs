use std::{io::{Error, ErrorKind}};

mod transfer;
mod settings;
mod help;

fn main() -> Result<(),Error> {
    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..];

    if args.len() == 0 {
        println!("Please use one of the following arguments. host, leech, settings or help");
        return Err(Error::new(ErrorKind::InvalidInput, "No arguments supplied to main function!"));
    }

    match args[0].to_lowercase().as_str() {
        "host" => transfer::host::host(&args[1..],settings::std_output)?,
        "leech" => transfer::leech::leech(&args[1..], settings::std_output)?,
        "settings" => settings::settings(&args[1..])?,
        "help" => help::help(args),
        _ => println!("Failed to read arguments. Please use LEECH, HOST or SETTINGS")
    }
    Ok(())
}

#[derive(Debug)]
struct Thing;