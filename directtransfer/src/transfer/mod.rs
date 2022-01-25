use serde::{Deserialize, Serialize};

pub mod host;
pub mod leech;

#[derive(Serialize, Deserialize, Debug)]
struct FileMetaData {
    filename: String,
    chunk_amount: u64,
    chunk_size: usize,
    last_chunk_size: usize,
}

impl std::fmt::Display for FileMetaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "file meta data:\n    filename: {}\n    chunk_amount: {}\n    chunk_size: {}\n    last_chunk_size: {}",
            self.filename, self.chunk_amount, self.chunk_size, self.last_chunk_size
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TransferMetaData {
    fileamount: u32,
    total_size: u64,
}

impl std::fmt::Display for TransferMetaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "files: {}\ntotal size: {} MB",
            self.fileamount,
            self.total_size as f64 / 1000000f64
        )
    }
}

pub fn check_progress(total_transferred: u64, target_transferred: u64, progress: &mut u8, output: fn(print:String)) {
    let c_progress = ((total_transferred as f64 / target_transferred as f64) * 100.) as u8;
    if c_progress != *progress {
        *progress = c_progress;
        output(format!("PROGRESS: {}%", progress));
    }
}