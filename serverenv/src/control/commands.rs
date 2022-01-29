use std::fs::remove_file;
use std::{thread, time::Duration, fs::File};
use std::io::prelude::*;
#[cfg(target_os="linux")]
    use tokio::process::Command;
use crate::control;
use crate::control::Runstate;

use super::{EnviromentState, printout};

pub async fn handle_command(
    command_sign: String,
    args: Vec<String>,
    enviroment_state: EnviromentState,
) {
    match command_sign.to_lowercase().as_ref() {
        "env_exit" => env_exit(args, enviroment_state),
        "env_update" => env_update(args, enviroment_state),
        "restart" => restart(args, enviroment_state),
        "reload" => reload(args, enviroment_state).await,
        "host_shutdown" => host_shutdown(args,enviroment_state),
        "env_panic" => panic!("panic test"), 
        c => printout(format!("    Failed to run command \"{}\"", c)),
    }
}

fn restart(_args: Vec<String>, enviroment_state: EnviromentState) {
    let mut child_processs = enviroment_state.server_process.lock().unwrap();
    match child_processs.start_kill() {
        Ok(_) => {},
        Err(_) => {printout("server process already dead")},
    }

    while child_processs.try_wait().unwrap().is_none() {
        thread::sleep(Duration::from_secs_f32(0.1));
    }
    let exit_status = child_processs.try_wait().unwrap().unwrap();
    printout(format!("    Server exited with {}", exit_status));

    *child_processs = control::spawn_server();
}

fn env_exit(_args: Vec<String>, enviroment_state: EnviromentState) {
    printout("    Starting exit");
    *enviroment_state.runstate.lock().unwrap() = Runstate::Closing;
}

async fn reload(args: Vec<String>, enviroment_state: EnviromentState) {
    if args.len() < 1 {
        printout("Reload needs an ipaddress to load new server from. ");
        return;
    }
    {
        let mut lock = enviroment_state.server_process.lock().unwrap();
        lock.start_kill().unwrap();
    }

    //Once we can delete and remake the server.js file we know we can replace it with directtransfer!
    let mut file = File::open("./serverenv/server/server.js").unwrap();
    let mut file_content = String::new(); 
    file.read_to_string(&mut file_content).unwrap();
    let mut res = false;
    while !res {
        let success= remove_file("./serverenv/server/server.js");
        res = success.is_ok();
    }

    let mut serverjs = File::create("./serverenv/server/server.js").unwrap();
    serverjs.write_all(file_content.as_str().as_bytes()).unwrap();

    printout("    Server process ended");

    printout("    beginning download of new server version");
    

    match directtransfer::transfer::leech::leech(&[args[0].clone(), "./serverenv".to_string()], |text| {
        printout(format!("{}", text))
    }) {
        Ok(_) => (),
        Err(err) => 
            printout(format!("Failed to download new version of server, attempting start of old version (may be corrupted due if some of the server was transferred) err: {}",err)),
        
    }

    let mut child_processs = enviroment_state.server_process.lock().unwrap();
    *child_processs = control::spawn_server();
}

fn host_shutdown(_args: Vec<String>, _enviroment_state: EnviromentState){
    #[cfg(target_os="linux")] {
        match Command::new("sudo").arg("shutdown").spawn() {
            Ok(_) => {
                env_exit(_args, _enviroment_state);
            },
            Err(err) => printout(format!("shutdown failed with error {}",err))
        }   
    }
    #[cfg(target_os="windows")] {
        printout("host_shutdown only supported on linux");
    }
}

fn env_update(_args: Vec<String>, _enviroment_state: EnviromentState) {
    *control::REPEAT_ON_EXIT.lock().unwrap() = true;
    #[cfg(target_os = "linux")] {
        env_exit(_args, _enviroment_state);
    }
}