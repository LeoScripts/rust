loop {
    println!("loop infinito ate que exista um break!");
    break;
}

// variavel mutavel
// se o 'mut' for removido, o compilador irá reclamar
// pois a variavel 'number' não pode ser alterada
// sem a palavra-chave 'mut' ela é imutavel (é uma constante)
let mut number = 3;
// funcionamento igual ao do javascript
while number != 0 {
    println!("{}!", number);

    number -= 1;
}

let array = [10, 20, 30, 40, 50];
for element in array.iter() { // iter() retorna um iterador
    println!("O valor do elemento é: {}", element);
}