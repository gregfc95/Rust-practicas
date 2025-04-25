/*
5-Defina la función llamada duplicar_valores que recibe un arreglo de números flotantes y
retorna un arreglo nuevo con los valores duplicados del parámetro.
*/

pub fn ej5() {
    let arr = vec![1.0, 2.0, 3.0];
    let arr_duplicado = duplicar_valores(arr.clone());

    println!("Arreglo original: {:?}", arr);

    println!("Arreglo duplicado: {:?}", arr_duplicado);
}

fn duplicar_valores(arr: Vec<f64>) -> Vec<f64> {
    //Creamos el vector
    let mut arr_duplicado = Vec::new();
    //Iteramos sobre el vector y duplicamos cada elemento
    for &num in arr.iter() {
        arr_duplicado.push(num * 2.0);
    }
    // 4. Retornamos el nuevo vector (sin ';' para que sea la expresión de retorno)
    arr_duplicado
}
/*
fn duplicar_valores_v2(arr: Vec<f64>) -> Vec<f64> {
    arr.into_iter() // Convertimos el vector en un iterador consumible
    .map(|x| x * 2.0) // Aplicamos la multiplicación a cada elemento
    .collect() // Convertimos el iterador en un nuevo Vec<f64>
}
*/
