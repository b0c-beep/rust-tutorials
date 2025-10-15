use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                handle_client(&mut stream)?;
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}


fn handle_client(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            return Ok(()); // Connection was closed
        }

        stream.write_all(&buffer[0..bytes_read])?;

        println!("Received and echoed back {}", String::from_utf8_lossy(&buffer[0..bytes_read]));
    }   
}