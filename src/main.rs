use std::{env, io::Write};
use chrono;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        println!("Usage: {} port", args[0]);
        return;
    }

    let ip = "127.0.0.1";
    
    let port = args[1].parse::<u16>().unwrap();

    let listener = std::net::TcpListener::bind((ip, port)).unwrap();


    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut stream = stream;
                println!("New connection: {}", stream.peer_addr().unwrap());
                let daytime = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                stream.write(daytime.as_bytes()).unwrap();
                // close stream
                match stream.shutdown(std::net::Shutdown::Both) {
                    Ok(_) => println!("Connection closed"),
                    Err(e) => println!("Failed to close connection: {}", e),
                }
                
            }
            Err(e) => { println!("Error: {}", e); }
        }
    }
}
    
