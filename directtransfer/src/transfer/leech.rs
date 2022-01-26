use std::{
    io::{Error, Read},
    net::TcpStream,
};
#[cfg(target_os = "windows")] 
    use std::os::windows::prelude::FileExt;
#[cfg(target_os="linux")]
    use std::os::unix::fs::FileExt;

use crate::{
    settings,
    transfer::{self, FileMetaData},
};

use super::TransferMetaData;

pub fn leech(args: &[String], output: fn(print: String)) -> Result<(), Error> {
    let target_ip = &args[0];
    let mut output_dir = &settings::output_path();
    if args.len() > 0 {
        output_dir = &args[1];
        output(format!("Settings ignored, using {} instead",output_dir));
    }

    let mut progress = 0u8;
    let mut total_transferred = 0u64;

    output("STATUS: Connecting".to_string());

    let mut stream = TcpStream::connect(target_ip)?;

    output("STATUS: Transfer in progress".to_string());

    //First, read the transfer meta data. Such as file amount and total transfer size
    //The size of the transfer meta data is a u16 sent as two u8 bytes.
    // so to get the size of the transfer meta data we need to read the
    // two fist bytes

    let mut transfer_meta_data_size_bytes = [0u8; 2]; //Array of two u8 with value of 0
    stream
        .read_exact(&mut transfer_meta_data_size_bytes)
        .unwrap();

    let size: u16 = bincode::deserialize(&transfer_meta_data_size_bytes).unwrap();

    //Then read the transfer meta data by reading the amount of bytes as specified by size
    let mut transfer_meta_data_bytes = vec![0u8; size.into()];
    stream.read_exact(&mut transfer_meta_data_bytes).unwrap();

    let transfer_meta_data: TransferMetaData =
        bincode::deserialize(&transfer_meta_data_bytes).unwrap();
    output(format!("{}", transfer_meta_data));

    for _ in 0..transfer_meta_data.fileamount {
        //Read the size of the file meta data as u16 as two bytes
        let mut file_meta_data_size_buffer = [0u8; 2];
        stream.read_exact(&mut file_meta_data_size_buffer).unwrap();

        //Create a buffer for the file meta data, with the size of the recieved size
        let file_meta_data_size: u16 = bincode::deserialize(&file_meta_data_size_buffer).unwrap();
        let mut file_meta_data_buffer = vec![0u8; file_meta_data_size.into()];

        stream.read_exact(&mut file_meta_data_buffer)?;

        //Place into file_meta_data scruct
        let file_meta_data: FileMetaData = bincode::deserialize(&file_meta_data_buffer).unwrap();

        //create the file
        let mut path;
        #[cfg(target_os="windows")]
        {
            path = format!("{}\\{}", output_dir, file_meta_data.filename);
            path = path.replace("/","\\");
            std::fs::create_dir_all(path.split_at(path.rfind("\\").unwrap()).0)?;
        }
        #[cfg(target_os="linux")]{
            path = format!("{}/{}", output_dir, file_meta_data.filename);
            path = path.replace("\\", "/");
            std::fs::create_dir_all(path.split_at(path.rfind("/").unwrap()).0)?;
        }
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .unwrap();

        //Read all the chunks except the last
        // since the last chunk is another size it needs to be read seperately
        for chunk_index in 0..file_meta_data.chunk_amount - 1 {
            //Read the chunk from the stream
            let mut chunk_bytes = vec![0u8; file_meta_data.chunk_size];
            stream.read_exact(&mut chunk_bytes)?;

            //then write the chunk into the file
            let offset = chunk_index * file_meta_data.chunk_size as u64;
            #[cfg(target_os = "linux")]
            {
                file.write_at(&mut chunk_bytes, offset)?;
            }
            #[cfg(target_os = "windows")]
            {
                file.seek_write(&mut chunk_bytes, offset)?;
            }

            total_transferred += file_meta_data.chunk_size as u64;
            transfer::check_progress(
                total_transferred,
                transfer_meta_data.total_size,
                &mut progress,
                output,
            );
        }

        //Read the last chunk, which has a different size
        let mut last_chunk_bytes = vec![0u8; file_meta_data.last_chunk_size];
        stream.read_exact(&mut last_chunk_bytes)?;
        let offset = (file_meta_data.chunk_amount - 1) * file_meta_data.chunk_size as u64;

        #[cfg(target_os = "linux")]
        {
            file.write_at(&last_chunk_bytes, offset)?;
        }
        #[cfg(target_os = "windows")]
        {
            file.seek_write(&last_chunk_bytes, offset)?;
        }
        total_transferred += file_meta_data.last_chunk_size as u64;

        transfer::check_progress(
            total_transferred,
            transfer_meta_data.total_size,
            &mut progress,
            output,
        );
    }
    output("STATUS: Transfer complete".to_string());
    Ok(())
}
