mod commands;
use commands::{Command, send_command};
use std::net::{TcpStream, ToSocketAddrs, Shutdown};
use std::io::{self, Write, BufRead, BufReader};
use std::thread;
use std::sync::{Arc, Mutex};

fn get_input(question: &str) -> String {
    let mut buffer = String::new();

    print!("{}", question);

    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut buffer).expect("Failed to read stdin");
    
    return buffer;
}

fn is_input_empty(input: &str) -> bool {
    let first_char: Option<char> = input.chars().nth(0);
    return input.is_empty() || first_char.unwrap() == '\r' || first_char.unwrap() == '\n';
}

fn main() -> std::io::Result<()> {
    let domain: String = {
        let mut input: String = get_input("Domain (chat.freenode.net:6667): ");

        if is_input_empty(input.as_str()) {
            input = String::from("chat.freenode.net:6667");
        }

        input
    };
  
    let domain: &str = domain.as_str().trim();
    
    let nick: String = {
        let mut input: String = String::new();

        while is_input_empty(input.as_str()) {
            input = get_input("Nickname: ");
        }

        input
    };
    
    let nick: &str = nick.as_str().trim();

    let channel: String = {
        let mut input: String = String::new();

        while is_input_empty(input.as_str()) {
            input = get_input("Channel: ");
        }

        input
    };

    let channel: &'static str = String::leak(channel).trim();
    
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
    send_command(&stream, Command::JoinChannel(channel));

    let stream: Arc<Mutex<TcpStream>> = Arc::new(Mutex::new(stream));

    let stream_mutex: Arc<Mutex<TcpStream>> = Arc::clone(&stream);
    let reciever_thread: thread::JoinHandle<()> = thread::spawn(move || {
        let mut guard = stream_mutex.lock().unwrap();
        let mut reader = BufReader::new(&*guard);
        let mut buffer = String::new();

        while reader.read_line(&mut buffer).unwrap() > 0 {
            drop(reader);
            drop(guard);
            
            println!("{}", buffer.trim_end());
            buffer.clear();
            
            guard = stream_mutex.lock().unwrap();
            reader = BufReader::new(&*guard);
        }
    });

    let stream_mutex_: Arc<Mutex<TcpStream>> = Arc::clone(&stream);
    let sender_thread: thread::JoinHandle<()> = thread::spawn(move || {
        let mut buffer = String::new();

        loop {
            io::stdin().read_line(&mut buffer).expect("Failed to read stdin");
            
            if !is_input_empty(buffer.as_str()) {
                let guard = stream_mutex_.lock().unwrap();
                send_command(&*guard, Command::SendMessageToChannel(channel, buffer.as_str().trim_end()));
            }
            
            buffer.clear();
        }
    });
    
    reciever_thread.join().expect("Could not join with the reciever thread");
    sender_thread.join().expect("Could not join with sender thread");

    println!("Shutting down connection");
    stream.lock().unwrap().shutdown(Shutdown::Both).expect("TCP Connection shutdown failed");
    
    Ok(())
}
