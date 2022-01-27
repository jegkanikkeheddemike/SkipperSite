use std::{
    io::{Error, ErrorKind, Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

pub fn settings(args: &[String]) -> Result<(), Error> {
    if args.len() < 1 {
        println!("Please augment, get, set or reset");
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid argument, please augment, get, set or reset",
        ));
    }

    match args[0].to_lowercase().as_str() {
        "get" => print_settings(),
        "set" => set_settings(&args[1..]),
        "reset" => reset_settings(),
        _ => {
            println!("Failed to read settings argument, please augment, get, set or reset");
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid argument, please augment, get, set or reset",
            ));
        }
    }
    Ok(())
}

fn reset_settings() {
    save_settings(&std_settings());
    println!("settings reset");
    print_settings();
}

fn set_settings(args: &[String]) {
    let mut settings = get_settings();
    if args.len() < 2 {
        println!("Please write the new value of {}", args[0]);
        return;
    }

    match args[0].to_ascii_lowercase().as_str() {
        "output_path" => settings.output_path = String::from(&args[1]),
        "chunk_size" => settings.chunk_size = args[1].parse().unwrap(),
        _ => println!(
            "Failed to read the settings to change, please augment output_path or chunk_size"
        ),
    }
    save_settings(&settings);
    println!("Set {} to {}", args[0], args[1]);
}

fn print_settings() {
    let settings = get_settings();
    print!(
        "Settings:\n    output_path: {}\n    chunk_size: {}",
        settings.output_path, settings.chunk_size
    );
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    output_path: String,
    chunk_size: usize,
}

fn std_settings() -> Settings {
    Settings {
        output_path: format!("{}/output", directtransfer_folder()),
        chunk_size: 262144,
    }
}
fn save_settings(settings: &Settings) {
    //Make sure the folder exitsts
    if !Path::exists(Path::new(&directtransfer_folder())) {
        std::fs::create_dir_all(directtransfer_folder()).unwrap();
    }

    let settings_bytes = bincode::serialize(&settings).unwrap();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}/settings", directtransfer_folder()))
        .unwrap();
    file.write_all(&settings_bytes).unwrap();
}

fn get_settings() -> Settings {
    //if the settings file exitst, then load from it. If not then create new
    if Path::new(&format!("{}/settings",directtransfer_folder())).exists() {
        let mut file = std::fs::File::open(&format!("{}/settings",directtransfer_folder())).unwrap();
        let mut settings_buffer = vec![0u8; file.metadata().unwrap().len() as usize];
        file.read(&mut settings_buffer).unwrap();
        let settings: Settings = bincode::deserialize_from(&settings_buffer[..]).unwrap();

        return settings;
    }

    //if the settings file does not exist, create new one and save it
    let settings_new = std_settings();
    save_settings(&settings_new);
    settings_new
}
pub fn output_path() -> String {
    get_settings().output_path
}

pub fn chunk_size() -> usize {
    get_settings().chunk_size
}

pub fn std_output(print: String) {
    println!("{}", print);
}

fn directtransfer_folder() -> String {
    #[cfg(target_os = "windows")]
    {
        String::from("C:\\ProgramData\\directtransfer")
    }
    #[cfg(target_os = "linux")]
    {
        let home = String::from(home::home_dir().unwrap().to_str().unwrap());
        let res = format!("{}/Documents/directtransfer", home);
        println!("Directtransfer folder: {}",res);
        res
    }
}
