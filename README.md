# remote-shell
This repository contains a remote shell for learning purposes. 
Remote shell has two modes: client and server. Basically the client authenticates sending the specified username and password with server.
After that the client takes commands from command line and sends those commands to the server through a tcp connection. 
Server executes corresponding commands and sends the result back to client.

## Usage
If you provide host name then the application acts as a client.

### For server 

```
cargo run -- -u [username] -P [password] -p [port]
```

### For client
```
cargo run -- -u [username] -P [password] -h [host name] -p [port]
```

#### Run with executable
Compile using `cargo`:
```
cargo build --release
```

Finally, run the executable located at `target/release` directory:
```
./remote-shell -u [username] -P [password] -p [port]
```
