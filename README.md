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

- https://crates.io/ onde estao todos as libs de terceiros

1. exemplos
    - `cargo install NOME-DA-LIB`
    - `cargo add Nome-da-lib`


2. no arquivo `Cargo.toml` (similar ao package.json do node) iserir:

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


## build pra debud (DEV) / release (PROD)

- desenvolvimento

```bash
cargo build --example ARQUIVO_DESEJADO

```
- produção

```bash
                    # o "--example" e pra ser usando somente em testes
cargo build --release --example ARQUIVO_DESEJADO
```

o arquivo estara na em `./target/release` 


## Condicionais

- [if, else e else if](./condicionais/esle_else-if_else.rs)

## loops

- [loops](./loops/loops.rs)

## struct (objetos)

- [structs(objetos)](./struct_objeto/struct.rs)

## enums

- [enums](./enums/enums.rs)

- [enums 2](./enums/enums_2.rs)
    
    existem 2 enums por padrao no Rust, 
    são eles: `Option` e Result

```diff
! Curiosidade

+ no rust não existem tipos anulaveis como:
+ undefined
+ null
+ none

+ todos os tipos são algum tipo

+ pra poder receber alguma coisa vasia 
+ agente vai usar o 
- enm "Option" 

```