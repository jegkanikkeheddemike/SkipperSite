use tokio::{
    self,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() {
    let mut address = String::new();
    let stdin = async_std::io::stdin();
    println!("enter target ip");
    stdin.read_line(&mut address).await.unwrap();
    address = address[..address.chars().count() - 2].to_string();
    println!("READ: {:?}", address);

    let stream = tokio::net::TcpStream::connect(format!("{}:8080", address))
        .await
        .unwrap();

    let (mut reader, mut writer) = stream.into_split();

    println!("Connected, write command");

    let write_loop = async move {
        loop {
            let mut command = String::new();
            stdin.read_line(&mut command).await.unwrap();
            let command = command.as_str();
            let command = &command[..command.chars().count() - 2];

            let command_bytes = command.as_bytes();
            let command_bytes_length = [command_bytes.len() as u8];

            writer.write(&command_bytes_length).await.unwrap();
            writer.write(&command_bytes).await.unwrap();

            if command.eq("cmd_exit") || command.eq("env_exit") {
                break;
            }
        }
    };

    let read_loop = async move {
        loop {
            let mut message_size_buffer = [0u8];

            match reader.read_exact(&mut message_size_buffer).await {
                Ok(_) => (),
                Err(_) => {
                    println!("disconnected");
                    return;
                }
            }
            let message_size = message_size_buffer[0];

            let mut message_buffer = vec![0u8; message_size as usize];

            reader.read_exact(&mut message_buffer).await.unwrap();

            let message = std::str::from_utf8(&message_buffer).unwrap();
            println!("env: {}",message);
        }
    };

    let write_handle = tokio::task::spawn(write_loop);
    let read_handle = tokio::task::spawn(read_loop);

    write_handle.await.unwrap();
    read_handle.await.unwrap();
}
