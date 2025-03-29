/*
Escribir un programa que defina una variable de tipo entero sin signo, y luego imprima su
valor en hexadecimal.
*/

//use std::io::stdin;

pub fn ej2() {
    let entero_signo: u16 = 15;
    println!("El entero sin signo es: {:x}", entero_signo);
}
