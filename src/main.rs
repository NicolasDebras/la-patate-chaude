mod struct_game;
mod create_message;

use std::io::{self, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use crate::struct_game::{Welcome, WelcomeMessage};
use crate::create_message::welcome_message;

fn main() -> io::Result<()> {
    // Establish a TCP connection with the farend
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;


    // Buffer the bytes
    let data = b"Hello";
    let bytes_written = stream.write(data)?;
    println!(" bytes_writtent {}",bytes_written);

    //let bytes_written = stream.write(welcome_message(1).as_ref());

    // Tell TCP to send the buffered data on the wire
    stream.flush()?;

    Ok(())
}

