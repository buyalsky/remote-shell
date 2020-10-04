use std::net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr, TcpStream};
use crate::config::ServerConfig;
use std::process::{Stdio, Child, Command};
use std::io::{Write, Read};
use std::str;
use crate::{FAILED_AUTHENTICATION, NULL_RESPONSE, AUTHENTICATED};

pub fn initialize_server(config: ServerConfig) {
    let listener = bind_to_socket(config.port as u16);
    listen_for_client(listener, config);
}

fn bind_to_socket(port: u16) -> TcpListener{
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    TcpListener::bind(address).unwrap()
}

fn listen_for_client(listener: TcpListener, config: ServerConfig){
    for stream in listener.incoming() {
        serve(stream.unwrap(), config.clone());
    }
}

fn is_authenticated(mut stream: TcpStream, server_username: String, server_password: String) -> bool {
    println!("New connection: {}", stream.peer_addr().unwrap());
    let mut login_info = [0; 24];

    stream.read(&mut login_info).unwrap();
    let login_info: Vec<String> = String::from_utf8(login_info.to_vec())
        .unwrap()
        .split(" ")
        .map(|s| s.to_string())
        .collect();
    let username = login_info.get(0).unwrap().trim();

    // remove trailing null characters
    let password = login_info.get(1).unwrap().trim_matches(char::from(0));
    return if *username == server_username && *password == server_password {
        println!("Authentication succeed!");
        stream.write_all(AUTHENTICATED).unwrap();
        true
    } else {
        println!("Authentication failed!");
        stream.write_all(FAILED_AUTHENTICATION).unwrap();
        false
    }
}

fn serve(stream: TcpStream, config: ServerConfig) {
    if is_authenticated(stream.try_clone().unwrap(), config.username, config.password) {
        serve_client(stream);
    }
}

fn serve_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        let mut previous_command = None;
        let size = stream.read(&mut buffer).unwrap();
        let commands = String::from_utf8_lossy(&buffer[..size]);
        if commands.len() == 0 {
            break
        }
        let mut commands= commands.trim().split(" | ").peekable();

        while let Some(command) = commands.next() {
            let stdin = previous_command.map_or(Stdio::inherit(),
                                                |output: Child| Stdio::from(output.stdout.unwrap()));
            let stdout = Stdio::piped();
            println!("{}", command);
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;
            let output = Command::new(command).args(args).stdin(stdin).stdout(stdout).spawn();
            match output {
                Ok(output) => { previous_command = Some(output); },
                Err(e) => {
                    previous_command = None;
                    eprintln!("{}", e);
                },
            };
        }
        match previous_command.unwrap().wait_with_output() {
            Ok(output) => {
                let output = str::from_utf8(&output.stdout).unwrap().trim();
                println!("{:?}", output);
                if output.len() == 0 {
                    stream.write(NULL_RESPONSE).unwrap();
                } else {
                    stream.write(output.as_bytes()).unwrap();
                }
                stream.flush().unwrap();
            }
            Err(e) => {
                stream
                    .write(format!("Error while executing command ({})", e).as_bytes())
                    .unwrap();
            }
        }
    }
}
