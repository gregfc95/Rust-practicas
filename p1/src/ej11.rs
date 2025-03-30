/*
11- Escribir un programa que defina un arreglo de 5 cadenas, y luego permita al usuario
ingresar una cadena por teclado. El programa debe imprimir un mensaje si la cadena
ingresada por el usuario se encuentra en el arreglo.
*/

use std::io::stdin;
pub fn ej11() {
    let cadena_array: [&str; 5] = ["hola", "como", "estas", "yo", "soy"];
    let mut input: String = String::new();
    println!("Ingrese una cadena: ");

    stdin()
        .read_line(&mut input)
        .expect("Error al leer la cadena");
    let input: &str = input.trim();

    if cadena_array.contains(&input) {
        println!("La cadena {} se encuentra en el arreglo", input);
    } else {
        println!("La cadena {} no se encuentra en el arreglo", input);
    }
}
