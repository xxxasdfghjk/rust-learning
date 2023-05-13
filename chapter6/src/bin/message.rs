enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> &str {
        let message = match self {
            Message::Quit => "quit",
            Message::Move { x, y } => "move",
            Message::Write(_) => "write",
            Message::ChangeColor(_, _, _) => "changecolor",
        };
        return message;
    }
}

fn main() {
    let message = Message::Write(String::from("aaaf"));
    println!("{}", message.call());
}
