use std::io::{Read, Write};
use std::net::TcpStream;

use anyhow::{anyhow, Result};

fn send_cli_command(command: &str) -> Result<()> {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:45678") {
        stream.write_all(command.as_bytes()).unwrap();
        stream.write_all(b"\n")?;
        stream.flush()?;
        stream.shutdown(std::net::Shutdown::Write)?;
        
        // Improvement: Read available data instead of trying to fill the entire buffer
        let mut response = String::new();
        match stream.read_to_string(&mut response) {
            Ok(_) => {
                if !response.is_empty() {
                    println!("Server response: {}", response);
                }
                Ok(())
            },
            Err(e) => Err(anyhow!("Failed to read response from server: {}", e))
        }
    } else {
        Err(anyhow!("Failed to connect to server. Please check if the server is running."))
    }
}

fn main() {
    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    // Exit with error message if not enough arguments
    if args.len() < 2 {
        println!("Usage: {} <TODO name>", args[0]);
        return;
    }
    
    // Use the second argument (index 1) as the TODO name
    let todo_name = &args[1];
    
    // Create and send JSON command
    let command = format!("{{\"command\": \"add_todo\", \"args\": \"{}\"}}", todo_name);
    
    match send_cli_command(&command) {
        Ok(_) => println!("TODO added: {}", todo_name),
        Err(e) => eprintln!("Error: {}", e),
    }
}
