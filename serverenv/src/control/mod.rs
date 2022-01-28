use std::sync::{Arc, Mutex};

use chrono::{Datelike, Timelike};
use tokio::process::{Child, Command};

pub mod commands;
pub mod loops;

#[derive(Clone)]
pub struct EnviromentState {
    pub command_queue: Arc<Mutex<deadqueue::unlimited::Queue<String>>>,
    pub runstate: Arc<Mutex<Runstate>>,
    pub server_process: Arc<Mutex<Child>>,
}

#[derive(PartialEq, Debug)]
pub enum Runstate {
    Running,
    Closing,
}

pub fn spawn_server() -> tokio::process::Child {
    let child;
    #[cfg(target_os = "windows")]
    {
        child = Command::new("C:\\Program Files\\nodejs\\node.exe")
            .arg("serverenv\\server\\server.js")
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        child = Command::new("node")
            .arg("serverenv/server/server.js")
            .spawn()
            .unwrap();
    }
    printout("server starting");
    child
}

pub fn printout(text: impl std::fmt::Display) {
    let time = chrono::prelude::Local::now();
    let time = format!(
        "{}-{}-{} {}:{}:{}",
        time.year(),
        time.month(),
        time.day(),
        time.hour(),
        time.minute(),
        time.second()
    );
    let text = format!("{} || {}", text, time);
    println!("{}", text);
    loops::TCP_PRINTQUEUE.lock().unwrap().push(text);
}
