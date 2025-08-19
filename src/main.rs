use std::env;
use std::io;
use std::net::{TcpListener, TcpStream};

fn connect_client() -> io::Result<()> {
    println!("connecting client");
    Ok(())
}

fn start_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5000")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    println!("client has connected");
    println!("{stream:?}");
}

fn main() -> std::io::Result<()> {
    let arg = env::args().nth(1);

    match arg.as_deref() {
        Some("server") => start_server(),
        Some("client") => connect_client(),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Please enter server or client",
        )),
    }
}
