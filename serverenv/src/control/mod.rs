use std::{sync::{Arc, Mutex}, process::Stdio};

use tokio::process::{Child, Command};

pub mod loops;
pub mod commands;

#[derive(Clone)]
pub struct EnviromentState {
    pub command_queue: Arc<Mutex<deadqueue::unlimited::Queue<String>>>,
    pub runstate: Arc<Mutex<Runstate>>,
    pub server_process: Arc<Mutex<Child>>,
    pub ngrok_process: Arc<Mutex<Child>>,
}

#[derive(PartialEq,Debug)]
pub enum Runstate {
    Running,
    Closing,
    Closed
}

pub fn spawn_server() -> tokio::process::Child {
    #[cfg(target_os="windows")]{
        Command::new("C:\\Program Files\\nodejs\\node.exe").arg("server\\server.js").spawn().unwrap()
    }

    #[cfg(target_os="linux")] {
        Command::new("node").arg("server/server.js").spawn().unwrap()
    }
}

pub fn spawn_ngrok() -> tokio::process::Child {
    #[cfg(target_os="windows")]{
        Command::new("sub/windows/ngrok.exe").arg("http").arg("3000").stdout(Stdio::null()).spawn().unwrap()
    }
    #[cfg(target_os="linux")] {
        //let ngrok_path = format!("{}/sub/linux/ngrok",std::env::current_dir().unwrap().display());
        //println!("{}",ngrok_path);
        //Command::new("ngrok").arg("http").arg("3000").stdout(Stdio::null()).spawn().unwrap()
        Command::new("ls").spawn().unwrap() //Ngrok is unused, so it will just spawn a useless process instead
    }
}