# rust

- documentação offiline `rustup docs --book`
- documentação online https://doc.rust-lang.org/stable/book/


# criando projeto

```bash
cargo new Nome-projeto
```

- estrutura do projeto

    hello-rust
    |- Cargo.toml
    |- src
    |- main.rs


- `Cargo.toml` - é o manifesto de um projeto Rust. Aqui você encontra todos os metadados do projeto, assim como as declarações de dependência.

- `src/main.rs` - (`arquivo principal`) é onde nos vamos escrever nossa aplicação.


- `cargo run` - roda o projeto

apos executar o run:

    $ cargo run
    Compiling hello-rust v0.1.0 (/Users/ag_dubs/rust/hello-rust)
        Finished dev [unoptimized + debuginfo] target(s) in 1.34s
        Running `target/debug/hello-rust`
    Hello, world!

## adicionando crate(libraria(biblioteca ou pacote))

no arquivo `Cargo.toml` (similar ao package.json do node) iserir:

> nesse exemplo estaremos intalando o `ferris-says`

```rs
[dependencies]
ferris-says = "0.3.1"
```
apos isso buildar o app com `cargo build` o cargo ira intalar a dependencia 

```rs

use ferris_says::say; // imoportando o ferris
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
```

o retorno de ser

    __________________________
    < Hello fellow Rustaceans! >
    --------------------------
            \
           \
              _~^~^~_
          \) /  o o  \ (/
            '_   -   _'
            / '-----' \