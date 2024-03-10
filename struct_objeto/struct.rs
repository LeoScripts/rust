// ex: 2

// struct User<'a> {
//     // username: &'a str, // 'str' = escrito com 's' minusculo que dizer que ela e imutavel
//     // e nesse exemplo acima ela faz parte do codigo
    
//     email: &'a str,
//     sign_in_count: u64,
//     active: bool,
// }


// ex: 1

struct User {
    username: String, // 'String' = escrito com 'S' maiusculo que dizer que ela e mutavel
    // username: &'a str, // 'str' = escrito com 's' minusculo que dizer que ela e imutavel
    // e nesse exemplo acima ela faz parte do codigo
    
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    //ex: 2

    // let mut joe = User {
    //     // string que sera alocada em memoria (string alocada na pilha)
    //     username: "joe",
    //     email: "joe@gmail.com",
    //     sign_in_count: 2,
    //     active: true,
    // };    


    // ex: 1

    let mut joe = User {
                // string que sera alocada em memoria (string alocada na pilha)
        username: String::from("joe"),
               // string que sera alocada em memoria
               // caso eu queira que ela seja statica(parte do codigo)
               // e so remover o 'to.owned()'
               // e com isso terei um uso maior de memoria
        email: "joe@gmail.com".to_owned(),
        sign_in_count: 2,
        active: true,
    };

    if joe.active {
        joe.sign_in_count += 1; // o mesmo que joe.sign_in_count = joe.sign_in_count + 1;
        println!( "Seja bem vindo, {}!", joe.username);
    } else {
        println!("verifique se email @: {}", joe.username);
    }    
    // os parenteses serao substituido pelo valor das variaveis
    println!("{} logou {} vezes",joe.username, joe.sign_in_count);
}