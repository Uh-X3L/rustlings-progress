#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    Resize,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
