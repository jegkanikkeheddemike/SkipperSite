use std::{
    sync::{Arc, Mutex},
};

use ::futures::executor;
use tokio::{
    io::AsyncWriteExt,
    process::{Child, Command},
}; // 1.3.1
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
    let text = format!("    {}", text);
    println!("{}", text);
    if loops::WRITER_TO_CONNECTED.lock().unwrap().is_some() {



        
        let mut lock = loops::WRITER_TO_CONNECTED.lock().unwrap();
        let option = lock.as_mut();
        let writer = option.unwrap();

        let bytes = text.as_str();
        let length = [bytes.len() as u8];


        let write_task = writer.write(&length);
        match executor::block_on(write_task) {
            Ok(_) => (),
            Err(_) => return,
        }
        let write_task = writer.write(bytes.as_bytes());
        match executor::block_on(write_task) {
            Ok(_) => (),
            Err(_) => return,
        }
    }
}
