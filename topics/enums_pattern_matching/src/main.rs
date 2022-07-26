#[derive(Debug)]
enum ServerState {
    Stopped,
    Initializing,
    Running,
    Stopping,
    Error {
        type_: Errors,
        message: String
    },
}

#[derive(Debug)]
enum Errors {
    Unknown,
    BadConfiguration(String),
    Hacked
}

struct Server {
    state: ServerState,
    name: String,
    address: IpAddr
}

enum IpAddr {
    V4(String),
    V6(String)
}

fn main() {
    let primary_server_state = ServerState::Error { type_: Errors::Unknown, message: String::from("Unknown")};
    let auxiliary_server_state = ServerState::Initializing;

    let localhost_ip = IpAddr::V4(String::from("localhost"));

    let third_server = Server {
        state: ServerState::Initializing,
        name: String::from("localhost"),
        address: IpAddr::V6(String::from("::1")),
    };

    match third_server.state {
        ServerState::Stopped => println!("{}", "Server is currently stopped"),
        ServerState::Initializing => println!("{}", "Server is currently initializing"),
        ServerState::Error {type_, message} => {
            let error_info = get_error_info(type_);
            if let Some(info) = error_info {
                println!("{}", info);
            }
            println!("{}", message);
        },
        state => println!("{} {:?}", "Server is currently in state: ", state)
    };
}

fn get_error_info(err_type: Errors) -> Option<String> {
    match err_type {
        Errors::Unknown => None,
        Errors::BadConfiguration(filepath) => Some(format!("Bad configuration for {}", filepath)),
        Errors::Hacked => Some(String::from("We've been hacked!"))
    }
}