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
