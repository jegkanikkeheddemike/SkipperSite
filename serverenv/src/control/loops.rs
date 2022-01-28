use std::{
    net::SocketAddr,
    panic,
    sync::Mutex,
    time::Duration, io::Error,
};

use crate::control::commands::handle_command;
use once_cell::sync::Lazy;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

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
                TCP_PRINTQUEUE.lock().unwrap().push(format!("Locally recieved: {}",&command));
                enviroment_state.command_queue.lock().unwrap().push(command);
            }
            _ = check_running(enviroment_state.clone()) => {}
        };
    }
}

pub static TCP_PRINTQUEUE: Lazy<Mutex<deadqueue::unlimited::Queue<String>>> =
    Lazy::new(|| Mutex::new(deadqueue::unlimited::Queue::new()));

pub async fn tcp_io_loop(enviroment_state: EnviromentState) {
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

                let (mut reader, mut writer) = stream.into_split();
                loop {
                    tokio::select! {
                        command = tcp_read(&mut reader,addr) => {
                            if command.as_str().eq("ERR"){
                                break;
                            }
                            match command.as_str() {
                                "env_exit" => {
                                    enviroment_state.command_queue.lock().unwrap().push(command);
                                    break;
                                }, 
                                "cmd_exit" => {
                                    break;
                                }
                                _ => {
                                    enviroment_state.command_queue.lock().unwrap().push(command);
                                }
                            }
                        }
                        print = wait_for_tcp_output() => {
                            let res = tcp_write(&mut writer,print).await;
                            if res.is_err() {
                                printout("Error writing to tcp. Disconnecting from client");
                                break;
                            }
                        }
                        _ = check_running(enviroment_state.clone()) => {
                            break;
                        }
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

async fn tcp_read(reader: &mut OwnedReadHalf, addr: SocketAddr) -> String {
    let mut message_size_buffer = [0u8];
    let message_size;
    let res = reader.read_exact(&mut message_size_buffer).await;

    match res {
        Ok(_) => message_size = message_size_buffer[0],
        Err(err) => {
            printout(format!("Error from {} caused disconnect: {}", addr, err));
            return String::from("ERR");
        }
    }
    let mut message_buffer = vec![0u8; message_size as usize];

    let res = reader.read_exact(&mut message_buffer).await;

    match res {
        Ok(_) => (),
        Err(err) => {
            printout(format!("Error from {} caused disconnect: {}", addr, err));
            return String::from("ERR");
        }
    }

    let command = String::from(std::str::from_utf8(&message_buffer).unwrap());

    printout(format!("Remotely recived \"{}\" from {}", command, addr));
    command
}

async fn wait_for_tcp_output() -> String {
    loop {
        let res;
        {
            res = TCP_PRINTQUEUE.lock().unwrap().try_pop();
        }
        match res {
            Some(print) => return print,
            None => {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }
    }
}

async fn tcp_write(writer: &mut OwnedWriteHalf, print: String) -> Result<(),Error> {
    let message_bytes = print.as_str().as_bytes();
    let message_length = message_bytes.len();

    match writer.write(&[message_length as u8]).await {
        Ok(_) => {},
        Err(err) => {
            printout(format!("tcp failed to write \"{}\" (forcefully disconnected)",&print));
            return Err(err)
        },  
    }
    match writer.write(message_bytes).await {
        Ok(_) => {},
        Err(err) => {
            printout(format!("tcp failed to write \"{}\" (forcefully disconnected)",&print));
            return Err(err);
        }
    }
    Ok(())
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
