/*
9- Escribir un programa que defina un arreglo de 5 números enteros, y luego imprima la
suma de los valores del arreglo.
*/

pub fn ej9() {
    let arreglo: [u32; 5] = [1, 2, 3, 4, 5];
    println!("El arreglo es: {:?} ", arreglo);
    let mut suma: u32 = 0;
    for num in arreglo.iter() {
        suma += num;
    }
    println!("La suma de los elementos del arreglo es: {}", suma);
}
