use std::{panic, sync::Mutex, time::Duration};

use crate::control::commands::handle_command;
use once_cell::sync::Lazy;
use tokio::{io::AsyncReadExt, net::tcp::OwnedWriteHalf};

use super::{printout, EnviromentState, Runstate};

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
}
pub static WRITER_TO_CONNECTED: Lazy<Mutex<Option<OwnedWriteHalf>>> =
    Lazy::new(|| Mutex::new(None));

pub async fn tcp_read_loop(enviroment_state: EnviromentState) {
    let listener;
    match tokio::net::TcpListener::bind("0.0.0.0:8080").await {
        Ok(res_listener) => listener = res_listener,
        Err(err) => {
            printout(format!("Failed to start tcp reading loop err: {}", err));
            return;
        }
    }
    let addr = listener.local_addr().unwrap();
    let port = String::from(&format!("{}", &addr)[8..]);

    let print_addr;
    #[cfg(target_os = "windows")]
    {
        print_addr = format!(
            "{}:{}",
            ipconfig::get_adapters().unwrap()[0].ip_addresses()[0],
            port
        );
    }
    #[cfg(target_os = "linux")]
    {
        print_addr = format!("{}:{}", local_ip_address::local_ip().unwrap(), port);
    }

    printout(format!("Listening for commands on: {}", print_addr));
    loop {
        //For now can only accent a single command per connection
        tokio::select! {
            conn = listener.accept() => {
                let (stream, addr) = conn.unwrap();

                printout(format!("Connected to: {}",addr));

                let (mut reader, writer) = stream.into_split();

                *WRITER_TO_CONNECTED.lock().unwrap() = Some(writer);

                loop {

                    let mut message_size_buffer = [0u8];
                    let message_size;
                    let res = reader.read_exact(&mut message_size_buffer).await;

                    match res {
                        Ok(_) => message_size = message_size_buffer[0],
                        Err(err) => {
                            printout(format!("Error from {} caused disconnect: {}",addr,err));
                            break;
                        }
                    }
                    let mut message_buffer = vec![0u8;message_size as usize];

                    let res = reader.read_exact(&mut message_buffer).await;


                    match res {
                        Ok(_) => (),
                        Err(err) => {
                            printout(format!("Error from {} caused disconnect: {}",addr,err));
                            break;
                        }
                    }

                    let command = std::str::from_utf8(&message_buffer).unwrap();

                    printout(format!("Remotely recived \"{}\" from {}",command,addr));

                    if command.eq("cmd_exit") {
                        break;
                    }
                    enviroment_state.command_queue.lock().unwrap().push(command.to_string());

                    if command.eq("env_exit") {
                        break;
                    }
                }
                printout(format!("Disconnected from {}",addr));
            }
            _ = check_running(enviroment_state.clone()) => {
                break;
            }
        }
    }
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
