fn main() {
    println!("Hello, world!");
    // basic function
    add(10, 20);

    // funcao com retorno de valor
    let result_multiply = multiply(10, 4);
    println!("Resultado da multiplicação = {} ", result_multiply);

    // funcao com retorno de 2 valores
    let result_multiplos = return_2_values("MasterDEV");
    println!("String = {}", result_multiplos.0);
    println!("Tamanho = {}", result_multiplos.1);

    // nested functions 
    fn inside (idade: i32) {
        println!("fn inside idade = {}", idade)
    }
    inside(25)
}

fn add (a: i32, b: i32) {
    let soma = a + b;
    println!("{} + {} = {}", a, b, soma)
}

fn multiply (a: i32, b: i32) -> i32 {
    let result = a * b;
    return result;
}

fn return_2_values (texto: &str) -> (&str, i32) {
    return (texto, texto.len() as i32)
}


