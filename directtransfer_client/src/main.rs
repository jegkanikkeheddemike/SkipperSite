use std::{net::TcpStream, rc::Rc, sync::Mutex};

use websocket::sync::Writer;

mod commands;

static mut GLOBAL_SENDER: Option<Rc<Mutex<Writer<TcpStream>>>> = None;

fn main() {
    let home_page = include_str!("html\\final_home.html");

    let mut server = websocket::sync::Server::bind("127.0.0.1:0").unwrap();

    let port = server.local_addr().unwrap();
    let port = &format!("{}",port)[10..];

    std::thread::spawn(move || {
        let request = server.accept().unwrap();

        let client = request.accept().unwrap();
        let (mut receiver, writer) = client.split().unwrap();
        let sender;
        unsafe {
            GLOBAL_SENDER = Some(Rc::new(Mutex::new(writer)));
            sender = GLOBAL_SENDER.clone().unwrap().clone();
        }

        for message in receiver.incoming_messages() {
            let message = message.unwrap();
            match message {
                websocket::OwnedMessage::Close(_) => {
                    let message = websocket::OwnedMessage::Close(None);
                    sender.lock().unwrap().send_message(&message).unwrap();
                    println!("Client disconnected");
                    return;
                }

                websocket::OwnedMessage::Text(text) => {
                    let response = handle_incoming_text(text);

                    match response {
                        Some(response) => {
                            sender
                                .lock()
                                .unwrap()
                                .send_message(&websocket::OwnedMessage::Text(response.clone()))
                                .unwrap();
                            println!("Responded: {}", response);
                        }
                        None => {}
                    };
                }
                websocket::OwnedMessage::Binary(_) => {}
                websocket::OwnedMessage::Ping(_) => {}
                websocket::OwnedMessage::Pong(_) => {}
            }
        }
    });

    

    web_view::builder()
        .title("DirectTransfer")
        .content(web_view::Content::Html(home_page))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, _arg| {webview.eval(&format!("setup({})",port)[..]).unwrap(); Ok(())})
        .run()
        .unwrap();
}

fn handle_incoming_text(text: String) -> Option<String> {
    match &text[0..5] {
        "CMD: " => {
            return commands::handle_command(&text[5..]);
        }
        "LOG: " => println!("{}", text),
        _ => println!("FAILED TO HANDLE MESSAGE: {}", text),
    }
    return None;
}
