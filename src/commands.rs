use std::net::TcpStream;
use std::io::Write;

pub enum Command<'a> {
	SetNickname(&'a str),
	SetUser(&'a str),
	JoinChannel(&'a str)
}

impl Command<'_> {
	pub fn to_string(&self) -> String {
		return match self {
			Command::SetNickname(s) => format!("NICK {}\r\n", s),
			Command::SetUser(s) => format!("USER guest 0 * :{}\r\n", s),
			Command::JoinChannel(s) => format!("JOIN {}\r\n", s)
		};
	}
}

pub fn send_command(mut stream: &TcpStream, command: Command) {
    stream.write_all(command.to_string().as_bytes()).expect(format!("Failed to write bytes to {}", stream.peer_addr().unwrap()).as_str());
}
