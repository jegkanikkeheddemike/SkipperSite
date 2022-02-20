use std::{thread, time::Duration, fs};
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
        "env_update" => env_update(args, enviroment_state),
        "restart" => restart(args, enviroment_state),
        "reload" => reload(args, enviroment_state).await,
        "host_shutdown" => host_shutdown(args,enviroment_state),
        "env_panic" => panic!("panic test"),
        "wipe_db" => wipe_db(args,enviroment_state),
        "get_db" => get_db(args,enviroment_state),
        c => printout(format!("    Failed to run command \"{}\"", c)),
    }
}

fn get_db(_args: Vec<String>, _enviroment_state: EnviromentState) {
    match fs::read_to_string("./serverenv/chatdb.json") {
        Ok(res) => {
            let res = res.replace("{", "{\n");
            let res = res.replace("}", "\n}");
            printout(res)
        },
        Err(_) => printout("Failed to load db"),
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

    loop {
        let res;
        {
            let mut lock = enviroment_state.server_process.lock().unwrap();
            res = lock.try_wait().unwrap();
        }
        match res {
            Some(exit_status) => {
                printout(format!("Server exited with status {}",exit_status));
                break;
            },
            None => {
                tokio::time::sleep(Duration::from_millis(10)).await;
                continue;
            },
        }

        
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
        env_exit(_args, _enviroment_state);
}

fn wipe_db(args: Vec<String>, enviroment_state: EnviromentState){
    restart(args,enviroment_state);
    match std::fs::remove_file("./serverenv/chatdb.json") {
        Ok(_) => printout("Wiped DB"),
        Err(err) => printout(format!("Failed to wipe db with err: {}",err)),
    }
}