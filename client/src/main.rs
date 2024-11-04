use std::fs;
use std::io::{self, Write, BufReader, BufRead};
use std::net::TcpStream;
mod lib;
use lib::encode_to_base64;

fn main() {
    if let Err(e) = start_data_transfer() {
        eprintln!("Error: {:?}", e);
    }
}

fn start_data_transfer() -> io::Result<()> {
    let branch_code = "DOBNF";
    let file_path = "../data/DOBNF/branch_weekly_sales.csv";
    let file_content = fs::read_to_string(file_path)?;
    if file_content.is_empty() {
        eprintln!("Error: File is empty.");
        return Err(io::Error::new(io::ErrorKind::Other, "File is empty."));
    }
    let encoded_content = encode_to_base64(file_content);
    let mut stream = TcpStream::connect("127.0.0.1:12345")?;
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
            eprintln!("Unexpected server response: {}", response);
        }
    } else {
        eprintln!("Unexpected server response: {}", response);
    }
    Ok(())
}
