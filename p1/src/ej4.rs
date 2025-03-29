/*
4- Escribir un programa que defina una tupla que contenga una cadena, un número entero
con signo y un valor booleano, y luego imprima cada valor de la tupla
 */

pub fn ej4() {
    let tupla: (&str, i32, bool) = ("cadena", 15, false);
    println!("La tupla es: ({},{},{})", tupla.0, tupla.1, tupla.2);
}
