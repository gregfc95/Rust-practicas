/*
12-Definir una función llamada reemplazar_pares que recibe un arreglo de enteros y
reemplaza todos los números pares por -1.
*/

pub fn ej12() {
    let arr = vec![1, 2, 3, 4, 5];

    let arr = reemplazar_pares(arr);
    println!("{:?}", arr);
}

fn reemplazar_pares(mut arr: Vec<i32>) -> Vec<i32> {
    for i in 0..arr.len() {
        if arr[i] % 2 == 0 {
            arr[i] = -1;
        }
    }
    arr
}
