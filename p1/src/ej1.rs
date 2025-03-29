/*
Escribir un programa que defina una variable de tipo flotante con algún valor, y luego
permita al usuario ingresar un número decimal por teclado para multiplicar, dividir, sumar y
restar su valor. Se deben imprimir los resultados.
*/

use std::io::stdin; //libreria estandar I/O

pub fn ej1() {
    // declarar variable
    let num_programa: f32 = 8.0;
    let mut input = String::new();
    println!("Ingrese un numero:");

    // leer el numero
    stdin()
        .read_line(&mut input)
        .expect("Error al leer el numero");

    //Tomamos el string de teclado y TypeCasting a float
    let num_usuario: f32 = input.trim().parse().expect("Error al convertir el numero");

    //Imprimimos
    println!("Calculando... ");
    println!("Numero del programa: {}", num_programa);
    println!("Numero ingresado: {}", num_usuario);

    println!("\nResultados de las operaciones:");
    println!("---------------------------------");
    println!("Suma: {}", num_programa + num_usuario);
    println!("Resta: {}", num_programa - num_usuario);
    println!("Multiplicación: {}", num_programa * num_usuario);
    println!("División: {}", num_programa / num_usuario);
}
