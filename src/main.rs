mod r#struct;

use std::io::{self, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct WelcomeMessage {
    number: i32,

}

fn main() -> io::Result<()> {
    let message = WelcomeMessage{  number : 1 };
    let serialized = serde_json::to_string(&message).unwrap();
    println!("serialized = {}", serialized);
    // Establish a TCP connection with the farend
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;


    // Buffer the bytes
    let data = b"Hello";
    let bytes_written = stream.write(data)?;
    println!(" test {}",bytes_written);
    if bytes_written < data.len() {
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            format!("Sent {}/{} bytes", bytes_written, data.len()),
        ));
    }
    // Tell TCP to send the buffered data on the wire
    stream.flush()?;

    Ok(())
}

