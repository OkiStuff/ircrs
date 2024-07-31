mod commands;
use commands::Command;
use std::net::{TcpStream, ToSocketAddrs, Shutdown};
use std::io::{self, Write, BufRead, BufReader};

fn get_input(question: &str) -> String {
    let mut buffer = String::new();

    print!("{}", question);

    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut buffer).expect("Failed to read stdin");
    
    return buffer;
}

fn send_command(mut stream: &TcpStream, command: Command) {
    let _ = stream.write_all(command.to_string().as_bytes());
}

fn main() -> std::io::Result<()> {
    let domain: String = {
        let mut input: String = get_input("Domain (chat.freenode.net:6667): ");
        
        if input.is_empty() {
            input = String::from("chat.freenode.net:6667");
        }

        input
    };
    
    let domain: &str = domain.as_str().trim();
    
    let nick: String = {
        let mut input: String = String::new();

        while input.is_empty() {
            input = get_input("Nickname: ");
        }

        input
    };
    
    let nick: &str = nick.as_str().trim();
    
    let addrs = domain.to_socket_addrs().expect("Unable to resolve domain");
    let mut maybe_stream: Option<TcpStream> = None;

    for addr in addrs {
        println!("Attempting connection to {}", addr);
        match TcpStream::connect(addr) {
            Ok(s) => {
                println!("Connected to {}", addr);
                maybe_stream = Some(s);
                break;
            }

            Err(_) => {
                println!("Failed to establish connection, trying next address");
                continue;
            }
        }
    }

    let stream: TcpStream = match maybe_stream {
        Some(s) => s,
        None => {
            println!("Could not establish any connection to {}", domain);
            std::process::exit(-1);
        }
    };

    send_command(&stream, Command::SetNickname(nick));
    send_command(&stream, Command::SetUser(nick));
    send_command(&stream, Command::JoinChannel("freenode"));

    let mut reader = BufReader::new(&stream);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer)? > 0 {
        println!("{}", buffer.trim_end());
        buffer.clear();
    }

    println!("Shutting down connection");
    stream.shutdown(Shutdown::Both).expect("TCP Connection shutdown failed");
    
    Ok(())
}
