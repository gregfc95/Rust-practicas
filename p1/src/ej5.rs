/*
5- Escribir un programa que defina una variable de tipo cadena, y luego permita al usuario
ingresar una cadena por teclado para concatenar su valor. El programa debe imprimir la
cadena en mayúsculas.
*/

use std::io::stdin;

pub fn ej5() {
    let cadena: String = "cadena ".to_string();
    let mut input: String = String::new();

    println!("Ingrese una cadena:");
    stdin()
        .read_line(&mut input)
        .expect("Error al leer la cadena");

    println!(
        "La cadena en mayúsculas es: {}",
        cadena.to_uppercase() + &input.to_uppercase()
    );
}
