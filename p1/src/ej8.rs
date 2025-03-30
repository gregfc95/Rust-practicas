/*
8- Escribir un programa que defina una constante de tipo cadena, y luego imprima el
número de veces que un caracter específico ingresado por el usuario aparece en la cadena.
Se debe imprimir el resultado.
 */

use std::io::stdin;
pub fn ej8() {
    const CADENA: &str = "holaYOsoyUNAcadenadeJOSE";

    println!("Ingrese un caracter: ");
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Error al leer entrada");
    let mut contador: u32 = 0;

    //trim() elimina los espacios en blanco
    //chars() convierte el String en un iterador de caracteres
    //next() devuelve el primer elemento del iterador
    let input_char = input
        .trim()
        .chars()
        .next()
        .expect("Debes ingresar un carácter");
    for c in CADENA.chars() {
        if c == input_char {
            contador += 1;
        }
    }
    println!(
        "El caracter {} aparece {} veces en la cadena",
        input_char, contador
    );
}
