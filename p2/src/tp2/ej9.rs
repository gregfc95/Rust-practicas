/*

9-Definir la función llamada cantidad_en_rango que recibe 3 parámetros: 1 arreglo de
enteros, un número entero llamado inferior y otro número entero llamado superior. Esta
función retorna la cantidad de números del arreglo que están entre el rango de los
parámetros inferior y superior inclusive.

*/

pub fn ej9() {
    let arr = vec![1, 3, 5, 7, 9];

    let inf = 3;
    let sup = 7;

    let cantidad = cantidad_en_rango(arr, inf, sup);
    println!(
        "Cantidad de números entre {} y {} es: {}",
        inf, sup, cantidad
    );
}

fn cantidad_en_rango(arr: Vec<i32>, inf: i32, sup: i32) -> i32 {
    let mut cont = 0;

    for &num in arr.iter() {
        if num > inf && num < sup {
            cont += 1;
        }
    }
    cont
}
