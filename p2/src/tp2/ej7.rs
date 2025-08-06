/*
7-Definir la función llamada cantidad_de_mayores que recibe como parámetro un arreglo
de números enteros y un número entero llamado límite. Esta función retorna la cantidad de
números mayores al límite que tiene el arreglo.
*/

pub fn ej7() {
    let arr = vec![1, 2, 3, 4, 5];
    let limite = 3;

    let cantidad = cantidad_de_mayores(arr, limite);

    println!("Cantidad de números mayores al límite: {}", cantidad);
}

fn cantidad_de_mayores(arr: Vec<i32>, limite: i32) -> i32 {
    let mut contador = 0;
    for &arr in arr.iter() {
        if arr > limite {
            contador += 1;
        }
    }
    contador
}
