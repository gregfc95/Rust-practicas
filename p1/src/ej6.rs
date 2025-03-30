/*
6- Escribir un programa que defina una variable de tipo entero sin signo, y luego permita al
usuario ingresar un número entero por teclado para sumarse con la variable definida. El
programa debe imprimir el valor del número elevado al cuadrado.
 */

use std::io::stdin;

pub fn ej6() {
    println!("Ingrese un número entero:");
    let mut input: String = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Error al leer el número");

    let input: u32 = input.trim().parse().expect("Error al convertir el número");
    let numero: u32 = 128;

    let resultado = (input + numero).pow(2);

    println!(
        "El número elevado al cuadrado es: {}, con la suma de {}",
        resultado, numero
    );
}
