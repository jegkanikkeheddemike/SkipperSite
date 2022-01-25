use std::{thread, time::Duration};

use tokio::process::Command;

use crate::control;
use crate::control::Runstate;

use super::EnviromentState;

pub async fn handle_command(
    command_sign: String,
    args: Vec<String>,
    enviroment_state: EnviromentState,
) {
    match command_sign.to_lowercase().as_ref() {
        "env_exit" => env_exit(args, enviroment_state),
        "restart" => restart(args, enviroment_state),
        "reload" => reload(args, enviroment_state).await,
        c => println!("    Failed to run command \"{}\"", c),
    }
}

fn restart(_args: Vec<String>, enviroment_state: EnviromentState) {
    let mut child_processs = enviroment_state.server_process.lock().unwrap();
    child_processs.start_kill().unwrap();

    while child_processs.try_wait().unwrap().is_none() {
        thread::sleep(Duration::from_secs_f32(0.1));
    }
    let exit_status = child_processs.try_wait().unwrap().unwrap();
    println!("    Server exited with {}", exit_status);

    *child_processs = control::spawn_server();
    println!("    Server restarted");
}

fn env_exit(_args: Vec<String>, enviroment_state: EnviromentState) {
    println!("    Starting exit");
    *enviroment_state.runstate.lock().unwrap() = Runstate::Closing;
}

async fn reload(args: Vec<String>, enviroment_state: EnviromentState) {
    if args.len() < 1 {
        println!("Reload needs an ipaddress to load new server from. ");
        return;
    }
    {
        let mut lock = enviroment_state.server_process.lock().unwrap();
        lock.start_kill().unwrap();
    }
    println!("    beginning download of new server version");

    directtransfer::transfer::leech::leech(&[args[0].clone(), ".".to_string()], |text| println!("{}",text))
        .unwrap();

    let mut child_processs = enviroment_state.server_process.lock().unwrap();
    *child_processs = control::spawn_server();
    println!("    Server restarted");
}
