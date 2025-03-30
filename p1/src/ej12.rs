/*

12- Escribir un programa que defina una tupla que contenga una cadena y un arreglo de
enteros, y luego imprima la cadena y la suma de los valores en el arreglo.

*/

pub fn ej12() {
    let tupla: (&str, [u32; 5]) = ("Triangulo", [1, 2, 3, 4, 5]);

    let (nombre, arreglo) = tupla;

    let suma: u32 = arreglo.iter().sum();
    println!("El nombre es: {}", nombre);
    println!("La suma de los elementos del arreglo es: {}", suma);
}
