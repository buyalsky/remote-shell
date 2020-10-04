use crate::config::ClientConfig;
use std::net::{TcpStream, Shutdown};
use crate::AUTHENTICATED;
use std::io::{Read, Write};
use std::process::exit;
use std::io;

fn connect_to_server(host_address: String, port: u16) -> TcpStream{
    let addr = format!("{}:{}", host_address, port);
    TcpStream::connect(addr).unwrap()
}

fn authenticate_with_server(mut stream: TcpStream, username: String, password: String) {
    let mut buffer = [0; 1024];
    stream
        .write(format!("{} {}", username, password).as_bytes())
        .unwrap();

    stream.read(&mut buffer).unwrap();
    // TODO come up with an elegant way
    if buffer[..AUTHENTICATED.len()].to_vec() != AUTHENTICATED.to_vec() {
        println!("Authentication failed!");
        exit(1);
    }
    println!("Authenticated with server!");
}

fn run_client(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    loop {
        let mut input = String::with_capacity(1024);
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        //remove trailing newline
        let command = input.trim();

        if command.ends_with("quit") {
            stream.shutdown(Shutdown::Both).unwrap();
            exit(0);
        }

        stream.write(command.as_bytes()).unwrap();
        let size = stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..size]);
        if response.len() > 1 {
            println!("{}", response);
        }
    }
}

pub fn initialize_client(config: ClientConfig) {
    let stream = connect_to_server(config.host_address, config.port);
    authenticate_with_server(stream.try_clone().unwrap(), config.username, config.password);
    run_client(stream);
}