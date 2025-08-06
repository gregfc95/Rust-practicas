/*
14-Definir una función llamada incrementar que recibe como parámetro un número flotante
e incrementa en 1 su valor.
*/
use std::io::stdin;

pub fn ej14() {
    println!("Ingrese un número: ");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    let num: f32 = input.trim().parse().expect("Error al convertir el número");
    let num = incrementar(num);
    println!("El nuevo valor es: {}", num);
}

fn incrementar(mut num: f32) -> f32 {
    num += 1.0;
    num
}
