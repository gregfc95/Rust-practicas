/*
11-Definir la función llamada multiplicar_valores que recibe como parámetro un arreglo de
enteros y otro número entero llamado factor. Esta función multiplica los valores del arreglo
por el parámetro factor modificándolo.
*/

pub fn ej11() {
    let arr = vec![1, 2, 3, 4, 5];
    let factor = 2;

    let arr = multiplicar_valores(arr, &factor);
    println!("{:?}", arr);
}

fn multiplicar_valores(mut arr: Vec<i32>, factor: &i32) -> Vec<i32> {
    for i in 0..arr.len() {
        arr[i] *= factor;
    }
    arr
}
