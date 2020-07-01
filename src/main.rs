use crate::config::{parse_client_config, parse_server_config, ClientConfig, ServerConfig};
use std::env;
use std::io;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::process::{exit, Command};
use std::str;

mod config;

const NULL_RESPONSE: &[u8] = " ".as_bytes();

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        7 => server(parse_server_config(args)),
        9 => client(parse_client_config(args)),
        _ => {
            println!(
                "Usage for server: -p port -u username -s password\nUsage for client: -h host_address -p port -u username -s password"
            );
            exit(1);
        }
    };
}

fn server(config: ServerConfig) {
    println!("server");
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), config.port as u16);
    let listener = TcpListener::bind(address).unwrap();
    let mut buffer = [0; 1024];

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
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

        if *username == config.username && *password == config.password {
            println!("Authentication succeed!");
        } else {
            println!("Authentication failed!");
            exit(1);
        }
        loop {
            let size = stream.read(&mut buffer).unwrap();
            let command = String::from_utf8_lossy(&buffer[..size]);
            println!("{}", command);
            let command: Vec<&str> = command.split(" ").collect();
            let output = Command::new(command.get(0).unwrap().trim().clone())
                .arg(command.get(1..).unwrap().join(" "))
                .output();
            match output {
                Ok(output) => {
                    let output = str::from_utf8(&output.stdout).unwrap().trim();
                    println!("{:?}", output);
                    if output.len() == 0{
                        stream.write(NULL_RESPONSE).unwrap();
                    }
                    else {
                        stream.write(output.as_bytes()).unwrap();
                    }
                }
                Err(e) => {
                    stream
                        .write(format!("Error while executing command ({})", e).as_bytes())
                        .unwrap();
                }
            }
        }
    }
}

fn client(config: ClientConfig) {
    println!("client");
    let mut buffer = [0; 1024];
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), config.port as u16);
    let mut stream = TcpStream::connect(address).unwrap();
    println!("connection succeed");
    stream
        .write(format!("{} {}", config.username, config.password).as_bytes())
        .unwrap();
    loop {
        let mut command = String::with_capacity(1024);
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        //remove trailing newline
        command.truncate(command.len() - 1);

        println!("{:?}", command);
        stream.write(command.as_bytes()).unwrap();
        let size = stream.read(&mut buffer).unwrap();
        let command = String::from_utf8_lossy(&buffer[..size]);
        println!("Response: {}", command);
    }
}
