pub fn handle_command(command: &str) -> Option<String> {
    let command_type = if command.contains(" ") {
        command.split_once(" ").unwrap().0
    } else {
        command
    };
    match command_type {
        "PICK_FOLDER" => {
            let path = native_dialog::FileDialog::new()
                .show_open_single_file()
                .unwrap();

            let path = match path {
                Some(path) => format!("PATH:{}", path.to_string_lossy()),
                None => String::from(""),
            };
            return Some(path);
        }
        "LEECH" => {
            let target_ip = command.split_once(" ").unwrap().1;
            println!("LEECHING {}", target_ip);

            let output_function = |print| {
                let sender = unsafe { crate::GLOBAL_SENDER.clone().unwrap().clone() };
                println!("Output: {}", print);
                sender
                    .lock()
                    .unwrap()
                    .send_message(&websocket::OwnedMessage::Text(print))
                    .unwrap();
            };

            directtransfer::transfer::leech::leech(&[target_ip.to_string()], output_function)
                .unwrap_or_else(|err| {
                    output_function(format!("STATUS: ERROR: {}", err));
                });
        }

        "HOST" => {
            let hosting_location = command.split_once(" ").unwrap().1;
            println!("HOSTING {}", hosting_location);

            let output_function = |print| {
                let sender = unsafe { crate::GLOBAL_SENDER.clone().unwrap().clone() };
                println!("Output: {}", print);
                sender
                    .lock()
                    .unwrap()
                    .send_message(&websocket::OwnedMessage::Text(print))
                    .unwrap();
            };

            directtransfer::transfer::host::host(&[hosting_location.to_string()], output_function)
                .unwrap_or_else(|err| {
                    output_function(format!("STATUS: ERROR: {}", err));
                });
        }
        _ => {
            println!("Failed to handle command {}", command);
        }
    }
    None
}
