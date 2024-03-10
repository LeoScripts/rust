// se tiver `std` é um lib padrao do rust

use rand::Rng;  // numeros aleatorios
use std::cmp::Ordering; // enum "Ordering" = é o resultado de toda comparação (> , < ou =)
use std::io; // (input e output) interação de linha de comando, relacionado a Pipes

fn main() {            //gerador de numeros aleatorios
                                    // gera um numero entre 1 e 11
    let number = rand::thread_rng().gen_range(1, 11);
    // println!("O numero secreto é: {}", number);

    // o loop infinito e usado pra sempre reiniciar o processo 
    // quando o usuario errar por exemplo
    loop {
        println!("descubra o numero entre 1 e 10: ");

        let mut numero_secreto = String::new(); // por convenção e usado o new pra cria um objeto vazio

        io::stdin() // usuario digitando no terminal
            .read_line(&mut numero_secreto) // recebe um string (mutavel)
            // .unwrap(); // não me importo com o tratamento de erro, quero que encerre o meu programa
            .expect("Falhou em ler a linha"); // quan se quer que o programa seja terminhado com essa mesagem

        println!(" seu palpite foi: {}", numero_secreto);
        
        // ## convertendo para um numero
                                             // "trin" remove os espaços
                                                    //"parse" - converte pra tipo desejado , nesse caso u32
                                                            //"expect"(muito poluido) se der alguma coisa errado - tratamento de erro agressivo
        //let numero_secreto: u32 numero_secreto.trim().parse().expect("deve ser do tipo numero"); // corretante e usado quando alguma coisa dar errado e precisamos abortar o programa

        // tratativa de erros correta para o ploblema
        let numero_secreto: u32 = match numero_secreto.trim().parse() {
            // caso queira mostrar o erro usamamos a variavel "e"
            // Err(e) => {
            // e pra ver o erro bem de forma legal e so usar "dbg!(e)" muito bom
            Err(_) => { // no caso de nao exibir nada ao usuario colocamo o "_"
                println!("Por favor digite um numero");
                continue; // restatar - continuar a receber numeros
            },
            Ok(number) => number
        };

        // uma forma de tratamento de erro mais elegante(correto)
        // fazer um "match" e se der um erro = printar a mensagem "você não digitou um numero tente novamente"
        
            // comparando com um numero
        match numero_secreto.cmp(&number) {
            Ordering::Less => println!("seu numero É menor"),
            Ordering::Greater => println!("seu numero É maior"),
            Ordering::Equal => {
                println!("Você acertou");
                break; // encerra a execução
            }
        }

    }
}