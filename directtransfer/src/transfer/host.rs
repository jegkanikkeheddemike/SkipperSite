use std::{
    fs::{self, File},
    io::{Error, Write, ErrorKind},
    net::TcpListener,
    os::windows::prelude::FileExt,
    path::Path,
};
use walkdir::WalkDir;

use crate::{
    settings,
    transfer::{self, FileMetaData, TransferMetaData},
};

pub fn host(args: &[String], output: fn(print: String)) -> Result<(), Error> {
    let target_path = &args[0];
    output(format!("Target path is {}", &target_path));

    let (files, transfer_meta_data) = get_file_data(&target_path)?;

    output(format!("{}", transfer_meta_data));

    let listener = TcpListener::bind("0.0.0.0:0")?;
    let addr = listener.local_addr()?;

    let port = String::from(&format!("{}", &addr)[8..]);
    let ip_addr = ipconfig::get_adapters().unwrap()[0].ip_addresses()[0];
    output("STATUS: Waiting for connection".to_owned());
    output(format!("listening on {}:{}", ip_addr, port));

    let mut progress = 0u8;
    let mut total_transferred = 0u64;

    let (mut stream, _) = listener.accept()?;

    output("STATUS: Transfer in progess".to_owned());

    //First send transfer meta data! Such as file amount and total transfer size
    //  send size of transfer meta data
    //      get size of transfer meta data

    let transfer_meta_data_bytes = bincode::serialize(&transfer_meta_data).unwrap();
    let size = transfer_meta_data_bytes.len() as u16;

    //      transfer the size
    stream.write(&bincode::serialize(&size).unwrap())?;

    //Then send the transfer meta data itself!
    stream.write(&transfer_meta_data_bytes)?;

    //chunk size needs to be saved before file reading starts, as it could be
    // changed by another process while transfer is in progress
    let chunk_size = settings::chunk_size();

    for file_index in 0..transfer_meta_data.fileamount {
        //Create meta data for the file
        let chunk_amount =
            files[file_index as usize].1.metadata().unwrap().len() / chunk_size as u64 + 1;
        let last_chunk_size =
            files[file_index as usize].1.metadata().unwrap().len() as usize % chunk_size;

        let file_meta_data = FileMetaData {
            filename: String::from(
                files[file_index as usize]
                    .0
                    .clone()
                    .split_at(target_path.rfind("\\").unwrap() + 1)
                    .1,
            ),
            chunk_amount,
            chunk_size: chunk_size,
            last_chunk_size,
        };

        let file_meta_data_bytes = bincode::serialize(&file_meta_data).unwrap();
        let file_meta_data_size = file_meta_data_bytes.len() as u16;

        let size_bytes = bincode::serialize(&file_meta_data_size).unwrap();

        //Send the size of the file meta data as u16 as two bytes
        stream.write(&size_bytes)?;

        //Send the file meta data itself
        stream.write(&file_meta_data_bytes)?;

        //Send all the chunks except the last.
        // since the last chunk is another size it will be sent seperately
        for chunk_index in 0..file_meta_data.chunk_amount - 1 {
            //load the chunk into memory from the file
            let mut chunk_bytes = vec![0u8; chunk_size];
            let offset = chunk_index * chunk_size as u64;
            files[file_index as usize]
                .1
                .seek_read(&mut chunk_bytes, offset)?;

            //Transfer the chunk
            stream.write(&chunk_bytes)?;
            total_transferred += chunk_size as u64;
            transfer::check_progress(
                total_transferred,
                transfer_meta_data.total_size,
                &mut progress,
                output,
            )
        }

        //Send the last chunk, which has a different size
        let mut last_chunk_bytes = vec![0u8; last_chunk_size];
        let offset = (chunk_amount - 1) * file_meta_data.chunk_size as u64;

        files[file_index as usize]
            .1
            .seek_read(&mut last_chunk_bytes, offset)?;
        stream.write(&last_chunk_bytes)?;

        total_transferred += last_chunk_size as u64;
        transfer::check_progress(
            total_transferred,
            transfer_meta_data.total_size,
            &mut progress,
            output,
        );
    }
    output("STATUS: Transfer complete".to_owned());
    Ok(())
}

fn get_file_data(target_path: &String) -> Result<(Vec<(String, File)>, TransferMetaData), Error> {
    let target_file = Path::new(target_path);

    if !target_file.exists() {
        return Err(Error::new(ErrorKind::InvalidInput, "File or directory not found"));
    }

    let md = fs::metadata(target_file).unwrap();
    let mut files: Vec<(String, File)> = vec![];

    let mut transfer_meta_data = TransferMetaData {
        fileamount: 0,
        total_size: 0,
    };

    if md.is_dir() {
        let walker = WalkDir::new(target_file).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let entry = entry.unwrap();
            let metadata = fs::metadata(entry.path()).unwrap();
            if metadata.is_file() {
                files.push((
                    String::from(entry.path().as_os_str().to_string_lossy()),
                    File::open(entry.path()).unwrap(),
                ));
                transfer_meta_data.fileamount += 1;
                transfer_meta_data.total_size += metadata.len();
            }
        }
    } else {
        let file = std::fs::File::open(target_file).unwrap();
        files.push((
            String::from(target_file.as_os_str().to_string_lossy()),
            file,
        ));
        transfer_meta_data.fileamount = 1;
        transfer_meta_data.total_size = files[0].1.metadata().unwrap().len();
    }

    Ok((files, transfer_meta_data))
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}