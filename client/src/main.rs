use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
mod lib;
use lib::encode_to_base64;

fn main() {
    if let Err(e) = start_data_transfer() {
        log_to_file(&format!("Error during data transfer: {:?}", e));
    }
}

fn start_data_transfer() -> io::Result<()> {
    let branch_code = "WHOSE";
    let file_path = "../data/WHOSE/branch_weekly_sales.csv";

    let file_content = fs::read_to_string(file_path).unwrap_or_else(|e| {
        log_to_file(&format!("Failed to read file '{}': {:?}", file_path, e));
        panic!("Critical failure: {:?}", e);
    });

    if file_content.is_empty() {
        let error_message = format!("File is empty: {}", file_path);
        log_to_file(&error_message);
        return Err(io::Error::new(io::ErrorKind::Other, error_message));
    }

    let encoded_content = encode_to_base64(file_content);
    let mut stream = TcpStream::connect("127.0.0.1:12345").unwrap_or_else(|e| {
        log_to_file(&format!("Failed to connect to server: {:?}", e));
        panic!("Critical failure: {:?}", e);
    });

    println!("Connected to server");
    writeln!(stream, "bcode~{}", branch_code)?;
    stream.flush()?;

    let mut response = String::new();
    {
        let mut reader = BufReader::new(&stream);
        reader.read_line(&mut response)?;
    }

    if response.trim() == "OK" {
        writeln!(stream, "~{}~", encoded_content)?;
        stream.flush()?;
        stream.shutdown(std::net::Shutdown::Write)?;
        response.clear();
        {
            let mut reader = BufReader::new(&stream);
            reader.read_line(&mut response)?;
        }
        if response.trim() == "OK" {
            println!("File transferred successfully.");
        } else {
            log_to_file(&format!("Unexpected server response: {}", response));
        }
    } else {
        log_to_file(&format!("Unexpected server response: {}", response));
    }
    Ok(())
}

fn log_to_file(message: &str) {
    let log_file = "error.log";
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .expect("Failed to open log file");

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    writeln!(file, "[{}] {}", timestamp, message).expect("Failed to write to log file");
}
