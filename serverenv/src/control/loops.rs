use std::{panic, time::Duration};

use tokio::io::AsyncReadExt;
use crate::control::commands::handle_command;

use super::{EnviromentState, Runstate};

pub async fn io_read_loop(enviroment_state: EnviromentState) {
    let stdin = async_std::io::stdin();

    while enviroment_state
        .runstate
        .lock()
        .unwrap()
        .eq(&Runstate::Running)
    {
        let mut input_line = String::new();
        tokio::select! {
            _ = stdin.read_line(&mut input_line) => {
                let command;
                #[cfg(target_os="windows")]{
                    command = input_line[..input_line.chars().count()-2].to_string();
                }
                #[cfg(target_os="linux")]{
                    command = input_line[..input_line.chars().count()-1].to_string();
                }
                enviroment_state.command_queue.lock().unwrap().push(command);
            }
            _ = check_running(enviroment_state.clone()) => {}
        };
    }
    println!("    Io reading loop exited");
}

pub async fn tcp_read_loop(enviroment_state: EnviromentState) {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let port = String::from(&format!("{}", &addr)[8..]);

    let print_addr;
    #[cfg(target_os="windows")]
    {
        print_addr = format!(
            "{}:{}",
            ipconfig::get_adapters().unwrap()[0].ip_addresses()[0],
            port
        );
    }
    #[cfg(target_os="linux")]
    {
        print_addr = format!(
            "{}:{}",local_ip_address::local_ip().unwrap(),port
        );
    }

    
    println!("Listening for commands on: {}", print_addr);
    loop {
        //For now can only accent a single command per connection
        tokio::select! {
            conn = listener.accept() => {
                let (mut socket, addr) = conn.unwrap();

                let mut read_buffer = String::new();

                socket.read_to_string(&mut read_buffer).await.unwrap();

                let command = read_buffer[..read_buffer.chars().count()-2].to_string();
                println!("Remotely recived \"{}\" from {}",command,addr);
                enviroment_state.command_queue.lock().unwrap().push(command);
            }
            _ = check_running(enviroment_state.clone()) => {
                break;
            }
        }
    }
    println!("    Tcp reading loop exited");
}

pub async fn command_executor_loop(enviroment_state: EnviromentState) {
    while enviroment_state
        .runstate
        .lock()
        .unwrap()
        .eq(&Runstate::Running)
    {
        let wait_for_next;
        //the lock on command queue needs to be lifted before we can await on the thread!
        {
            let command_queue_lock = enviroment_state.command_queue.lock().unwrap();
            wait_for_next = command_queue_lock.available() == 0;
        }

        if wait_for_next {
            tokio::time::sleep(Duration::from_millis(10)).await;
            continue;
        }
        let command_sign;
        let mut args: Vec<String> = vec![];
        //The command lock needs to be dropped before handle_command is called
        {
            let command_queue_lock = enviroment_state.command_queue.lock().unwrap();
            let command = command_queue_lock.try_pop().unwrap();

            if command.contains(" ") {
                command_sign = command[0..command.find(" ").unwrap()].to_string();
                let mut first = true;
                for arg in command.split(" ") {
                    if !first {
                        args.push(arg.to_string());
                    }
                    first = false;
                }
            } else {
                command_sign = command;
            }
        }
        handle_command(command_sign, args, enviroment_state.clone()).await;
    }
    println!("    Command executor loop exited");
}

async fn check_running(enviroment_state: EnviromentState) {
    loop {
        if !enviroment_state
            .runstate
            .lock()
            .unwrap()
            .eq(&Runstate::Running)
        {
            return;
        } else {
            tokio::time::sleep(Duration::from_secs_f32(0.1)).await;
        }
    }
}
