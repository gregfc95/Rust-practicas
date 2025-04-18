/*
6-Definir la función llamada longitud_de_cadenas que recibe un arreglo de String y retorna
un arreglo con la longitud de las cadenas del parámetro, correspondiéndose en posición del
arreglo.
*/

pub fn ej6() {
    let palabras = vec![
        "hola".to_string(),
        "como estas".to_string(),
        "🦀".to_string(),
    ];
    //SIN & Necesites modificar el valor original dentro de la función
    //SIN & Quieras transferir responsabilidad de la memoria
    // & Necesites leer datos sin modificarlos
    // & Quieras mantener el ownership del valor original
    let longitudes = longitud_de_cadenas(&palabras);
}

fn longitud_de_cadenas(arr: &Vec<String>) -> Vec<usize> {
    // Creamos un vector mutable para almacenar las longitudes
    let mut longitud = Vec::new();
    // Iteramos por referencia sobre cada String del vector
    for s in arr.iter() {
        //Obtenemos la longitud y la agregamos al vector
        longitud.push(s.len())
    }

    longitud
}
