use std::sync::{Arc, Mutex};
use tokio::{io::AsyncWriteExt, net::TcpStream, process::Command};

use crate::control::{EnviromentState, Runstate};

mod control;

#[tokio::main]
async fn main() {
    let server_process = Arc::new(Mutex::new(control::spawn_server()));
    let ngrok_process = Arc::new(Mutex::new(control::spawn_ngrok()));
    let runstate = Arc::new(Mutex::new(Runstate::Running));
    let command_queue = Arc::new(Mutex::new(deadqueue::unlimited::Queue::<String>::new()));

    let evnivoment_state = EnviromentState {
        command_queue,
        runstate,
        server_process,
        ngrok_process,
    };

    let tcp_read_loop = tokio::spawn(control::loops::tcp_read_loop(evnivoment_state.clone()));
    let io_read_loop = tokio::spawn(control::loops::io_read_loop(evnivoment_state.clone()));
    let command_executor_loop = tokio::spawn(control::loops::command_executor_loop(
        evnivoment_state.clone(),
    ));

    command_executor_loop.await.unwrap();
    io_read_loop.await.unwrap();
    tcp_read_loop.await.unwrap();

    *evnivoment_state.runstate.lock().unwrap() = Runstate::Closed;

    match evnivoment_state.server_process.lock().unwrap().kill().await {
        Ok(_) => println!("    Server exited succesfully"),
        Err(err) => println!(
            "Server failed to exit proberly, with error {}",
            err.to_string()
        ),
    };

    match evnivoment_state.ngrok_process.lock().unwrap().kill().await {
        Ok(_) => println!("    ngrok exited succesfully"),
        Err(err) => println!(
            "ngrok failed to exit proberly, with error {}",
            err.to_string()
        ),
    };
}

#[tokio::test]
async fn remote_control_test() {
    let mut address = String::new();
    let stdin = async_std::io::stdin();
    println!("enter target ip");
    stdin.read_line(&mut address).await.unwrap();
    address = address[..address.chars().count() - 2].to_string();
    println!("READ: {:?}",address);
    let mut stream = TcpStream::connect(format!("{}:8080", address))
        .await
        .unwrap();

    println!("Connected, write command");

    let mut command = String::new();
    stdin.read_line(&mut command).await.unwrap();
    command = format!("{}", command);
    stream.write_all(command.as_bytes()).await.unwrap();
}
