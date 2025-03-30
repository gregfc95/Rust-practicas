/*
10- Escribir un programa que defina dos arreglos de 5 números enteros cada uno, y luego
cree un tercer arreglo que contenga la suma de los elementos de los dos arreglos
originales.
*/

pub fn ej10() {
    let array1: [u32; 5] = [1, 2, 3, 4, 5];
    let array2: [u32; 5] = [6, 7, 8, 9, 10];
    let mut array_suma: [u32; 5] = [0; 5];

    for i in 0..5 {
        array_suma[i] = array1[i] + array2[i];
    }
    println!("Arreglo 1: {:?}", array1);
    println!("Arreglo 2: {:?}", array2);
    println!("Arreglo suma: {:?}", array_suma);
}
