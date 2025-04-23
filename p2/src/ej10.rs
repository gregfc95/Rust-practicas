/*
10-Definir la función llamada cantidad_de_cadenas_mayor_a que recibe como parámetros
un arreglo de String y un entero llamado límite. Esta función retorna la cantidad de Strings
del arreglo que son de longitud mayor al parámetro límite.
*/

pub fn ej10() {
    let arr: Vec<&str> = vec!["uno", "dos", "tres", "cuatro", "cinco"];
    let limite = 4;

    let cantidad = cantidad_de_cadenas_mayor_a(&arr, limite);
    println!("Cantidad de cadenas mayor a {}: {}", limite, cantidad);
}
/*
 Usa usize cuando trabajes con longitudes, capacidades o índices de colecciones. Si tienes un valor de otro tipo entero (como i32) que necesitas usar como índice o comparar con una longitud, conviértelo explícitamente a usize usando as usize
*/
fn cantidad_de_cadenas_mayor_a(arr: &Vec<&str>, limite: usize) -> usize {
    let mut contador = 0;
    for s in arr.iter() {
        if s.len() > limite {
            contador += 1;
        }
    }
    contador
}
