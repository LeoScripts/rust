// O "Option" e o "Result" estao sempre no "namespace" do Rust, então não precisamos importá-los
// podemos usá-los diretamente

enum Option<T> { //"Option" é um enum padrão do Rust
    Some(T), // "T" e um tipo genérico
    None,
}

enum Result<T, E> { //"Result" é um enum padrão do Rust
    Ok(T),
    Err(E), // "E" é um tipo genérico de erro
}

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // como esse match  eu posso receber um numero ou não
    match some_number {
        Some(x) => println!("o número é: {}", x),
        None => println!("Nenhum número"),
    }
}