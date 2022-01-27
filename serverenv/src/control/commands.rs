use std::{thread, time::Duration};

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
        "restart" => restart(args, enviroment_state),
        "reload" => reload(args, enviroment_state).await,
        c => printout(format!("    Failed to run command \"{}\"", c)),
    }
}

fn restart(_args: Vec<String>, enviroment_state: EnviromentState) {
    let mut child_processs = enviroment_state.server_process.lock().unwrap();
    child_processs.start_kill().unwrap();

    while child_processs.try_wait().unwrap().is_none() {
        thread::sleep(Duration::from_secs_f32(0.1));
    }
    let exit_status = child_processs.try_wait().unwrap().unwrap();
    printout(format!("    Server exited with {}", exit_status));

    *child_processs = control::spawn_server();
    printout("    Server restarted");
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
