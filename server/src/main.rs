use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
mod lib;
use lib::decode_base64;

fn main() -> io::Result<()> {
    if let Err(e) = fs::create_dir_all("data") {
        log_to_file(&format!("Failed to create 'data' directory: {:?}", e));
        panic!("Critical failure: {:?}", e);
    }
    start_listening()
}

fn start_listening() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:12345").unwrap_or_else(|e| {
        log_to_file(&format!("Failed to bind to port 12345: {:?}", e));
        panic!("Critical failure: {:?}", e);
    });
    println!("Server listening on port 12345");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|e| {
                        log_to_file(&format!("Error handling client: {:?}", e));
                    });
                });
            }
            Err(e) => log_to_file(&format!("Connection failed: {:?}", e)),
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let branch_code;
    {
        let mut reader = BufReader::new(&mut stream);
        let mut branch_code_line = String::new();
        reader.read_line(&mut branch_code_line)?;
        branch_code = branch_code_line.trim().replace("bcode~", "");
    }
    println!("Branch code: {}", branch_code);

    let branch_dir = format!("data/{}", branch_code);
    if let Err(e) = fs::create_dir_all(&branch_dir) {
        log_to_file(&format!(
            "Failed to create branch directory '{}': {:?}",
            branch_dir, e
        ));
        panic!("Critical failure: {:?}", e);
    }

    writeln!(stream, "OK")?;
    stream.flush()?;

    let mut encoded_content = String::new();
    {
        let mut reader = BufReader::new(&mut stream);
        reader.read_line(&mut encoded_content)?;
    }
    encoded_content = encoded_content.trim().trim_matches('~').to_string();
    println!("Received base64 content: {}", encoded_content);

    let decoded_content = decode_base64(encoded_content);
    if let Err(e) = fs::write(
        format!("{}/weekly_sales_report.txt", branch_dir),
        decoded_content,
    ) {
        log_to_file(&format!(
            "Failed to write to file in '{}': {:?}",
            branch_dir, e
        ));
        panic!("Critical failure: {:?}", e);
    }

    writeln!(stream, "OK")?;
    stream.flush()?;
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
