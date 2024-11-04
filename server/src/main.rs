use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
mod lib;
use lib::decode_base64;

fn main() -> io::Result<()> {
    fs::create_dir_all("data")?;
    start_listening()
}

fn start_listening() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:12345")?;
    println!("Server listening on port 12345");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|e| eprintln!("Error: {:?}", e));
                });
            }
            Err(e) => eprintln!("Connection failed: {:?}", e),
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
    fs::create_dir_all(&branch_dir)?;
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
    fs::write(format!("{}/weekly_sales_report.txt", branch_dir), decoded_content)?;
    writeln!(stream, "OK")?;
    stream.flush()?;
    Ok(())
}
