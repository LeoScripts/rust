enum Message {
    Quit,
    Move { x: i32, y: i32 }, // o i32 e o tipo de dado = inteiro de 32 bits
    Write(String),
    ChangeColor(i32, i32, i32),
    // Paste, //se eu nao passar nada ele vai dar erro  
}

fn main() {
    let move_msg = Message::Move { x: 10, y: 20 };
    handle_message(move_msg);
                            // o :: e o operador de namespace = o do php
    let change_color_msg = Message::ChangeColor(255, 0, 0);
    handle_message(change_color_msg);

    let write_msg = Message::Write(String::from("hello"));
    handle_message(write_msg);

    let quit_msg = Message::Quit;
    handle_message(quit_msg);
}

fn handle_message(msg: Message) {
    use Message::*;
    match msg { // igual ao switch case do javascript
        // se for qu
        Quit => println!("saindo da aplicação....."),
        Move { x, y } => {
            println!("mover para x {} e para y {}", x, y);
        }
        // Message::Write(text) => println!("Text message: {}", text),
        Write(text) => println!("O texto: {}", text),
        ChangeColor(r, g, b) => {
            println!("editar as cores do  RGB({},{},{})", r, g, b);
        }
    }
}