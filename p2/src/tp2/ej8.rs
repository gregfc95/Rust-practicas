/*
8- Definir la función llamada sumar_arreglos que recibe 2 arreglos del mismo tamaño de
números flotantes y retorna un nuevo arreglo que contiene la suma de los elementos de los
arreglos pasados por parámetro, correspondiéndose el resultado con cada posición de los
arreglos pasados por parámetro.
*/

pub fn ej8() {
    let arr1: [f32; 3] = [1.0, 2.0, 3.0];
    let arr2: [f32; 3] = [4.0, 5.0, 6.0];
    let result: [f32; 3] = sumar_arreglos(&arr1, &arr2);

    println!("Arreglo 1: {:?}", arr1);
    println!("Arreglo 2: {:?}", arr2);
    println!("Suma de ambos arreglos es: {:?}", result);
}

fn sumar_arreglos(arr1: &[f32; 3], arr2: &[f32; 3]) -> [f32; 3] {
    //Sintaxis de Array fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
    let mut result: [f32; 3] = [0.0; 3];
    for i in 0..arr1.len() {
        result[i] = arr1[i] + arr2[i];
    }
    result
}
