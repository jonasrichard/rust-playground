use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: &TcpStream) {
    let mut buf = [0u8; 64];

    loop {
        match stream.read(&mut buf) {
            Ok(n) if n > 0 => {
                println!("Read {} bytes", n);
                match std::str::from_utf8(&buf[..n]) {
                    Ok(s) => {
                        println!("--> {}", s);
                        match stream.write(&buf[..n]) {
                            Ok(w) =>
                                println!("Written {} bytes", w),
                            Err(e) => {
                                println!("Error {}", e);
                                return
                            }
                        }
                    },
                    Err(e) =>
                        println!("Error {}", e)
                }
            },
            Ok(_) =>
                return,
            Err(_) =>
                return
        }
    }
}

fn main() -> std::io::Result<()> {
    println!("Starting echo server at 9090...");

    let listener = TcpListener::bind("127.0.0.1:9090")?;

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(mut stream) =>
                handle_client(&mut stream),
            Err(e) =>
                return Err(e)
        }
    }
    std::io::Result::Ok(())
}
