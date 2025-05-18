/*
13-Definir una función llamada ordenar_nombres que recibe un arreglo de String y los
ordena en orden alfabético.
*/

pub fn ej13() {
    let arr1: Vec<String> = vec!["z".to_string(), "b".to_string(), "c".to_string()];

    let arr1 = ordenar_nombres(arr1);
    println!("{:?}", arr1);
}

fn ordenar_nombres(mut arr: Vec<String>) -> Vec<String> {
    arr.sort();
    arr
}
