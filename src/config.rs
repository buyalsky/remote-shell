use getopts::Options;

#[derive(Clone)]
pub struct ServerConfig {
    pub port: usize,
    pub username: String,
    pub password: String,
}

pub struct ClientConfig {
    pub host_address: String,
    pub port: usize,
    pub username: String,
    pub password: String,
}

fn init_options() -> Options{
    let mut opts = Options::new();
    opts.optopt("h", "host", "host address", "127.0.0.1");
    opts.optopt("p", "port", "set port", "3333");
    opts.optopt("u", "username", "set username", "user");
    opts.optopt("P", "password", "set password", "1234");
    opts
}

impl ServerConfig{
    pub fn parse_server_config(args: Vec<String>) -> ServerConfig {
        let opts = init_options();
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        };
        let port = matches.opt_str("p").unwrap().parse::<usize>().unwrap();
        let username = matches.opt_str("u").unwrap();
        let password = matches.opt_str("P").unwrap();
        ServerConfig {
            port,
            username,
            password,
        }
    }
}


impl ClientConfig{
    pub fn parse_client_config(args: Vec<String>) -> ClientConfig {
        let opts = init_options();

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        };
        let host_address = matches.opt_str("h").unwrap();
        let port = matches.opt_str("p").unwrap().parse::<usize>().unwrap();
        let username = matches.opt_str("u").unwrap();
        let password = matches.opt_str("P").unwrap();

        ClientConfig {
            host_address,
            port,
            username,
            password,
        }
    }
}

