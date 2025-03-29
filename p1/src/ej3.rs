/*Escribir un programa que defina una variable de tipo booleano, y luego permita al usuario
ingresar un valor booleano por teclado para actualizar su valor haciendo las operaciones
and y or. Se deben imprimir ambos resultados.
*/

use std::io::stdin;

pub fn ej3() {
    let mut input = String::new();

    println!("Ingrese un valor booleano: true O false");
    stdin()
        .read_line(&mut input)
        .expect("Error al leer el booleano");

    let valor_boolean: bool = input
        .trim()
        .parse()
        .expect("Error al convertir el booleano");

    println!("El valor booleano es: {}", valor_boolean);
    /*match input.trim() {
        "true" => println!("El valor booleano esTRUE: {}", valor_boolean),
        "false" => println!("El valor booleano esFALSE: {}", valor_boolean),
        _ => println!("Entrada no valida"),
    }*/
}
