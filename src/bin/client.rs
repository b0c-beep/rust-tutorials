use std::net::TcpStream;
use std::io::{self, Read, Write};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to the server!");

    loop {
        let mut input = String::new();
        println!("Enter message to send (or 'exit' to quit):");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        stream.write_all(input.as_bytes())?;

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            println!("Connection closed by server.");
            break;
        }

        println!("Received from server: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    }

    Ok(())
}