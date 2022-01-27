use control::{loops, printout};
use std::sync::{Arc, Mutex};
use tokio::{self, io::AsyncWriteExt};

use crate::control::{EnviromentState, Runstate};

mod control;

#[tokio::main]
async fn main() {
    let server_process = Arc::new(Mutex::new(control::spawn_server()));
    let runstate = Arc::new(Mutex::new(Runstate::Running));
    let command_queue = Arc::new(Mutex::new(deadqueue::unlimited::Queue::<String>::new()));

    let enviroment_state = EnviromentState {
        command_queue,
        runstate,
        server_process,
    };

    let tcp_read_loop = tokio::spawn(control::loops::tcp_read_loop(enviroment_state.clone()));
    let io_read_loop = tokio::spawn(control::loops::io_read_loop(enviroment_state.clone()));
    let command_executor_loop = tokio::spawn(control::loops::command_executor_loop(
        enviroment_state.clone(),
    ));

    match command_executor_loop.await {
        Ok(_) => printout("Command executor loop exited sucessfully"),
        Err(_) => printout("Command executor loop exited poisoned"),
    }

    //In case of panic the server needs to exit to free the port 3000
    match enviroment_state.server_process.lock().unwrap().kill().await {
        Ok(_) => printout("Server exited sucessfully"),
        Err(err) => printout(format!(
            "Server failed to exit proberly, with error {}",
            err.to_string()
        )),
    };

    match io_read_loop.await {
        Ok(_) => printout("io read loop exited sucessfully"),
        Err(_) => printout("io read loop exited poisoned"),
    }
    match tcp_read_loop.await {
        Ok(_) => printout("tcp read loop exited sucessfully"),
        Err(_) => printout("tcp read loop exited poisoned"),
    }



    //If is connected to command giver, then shut down connection
    let mut lock = loops::WRITER_TO_CONNECTED.lock().unwrap();
    let option = lock.as_mut();

    match option {
        Some(writer) => match writer.shutdown() {
            _ => {}
        },
        None => todo!(),
    }
}
