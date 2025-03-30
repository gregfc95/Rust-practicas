/*
7- Escribir un programa que defina una variable de tipo arreglo que contenga seis números
enteros, y luego multiplique cada valor del arreglo por un valor constante definido,
modificando el contenido del arreglo.
*/

pub fn ej7() {
    let mut arreglo: [u32; 6] = [1, 2, 3, 4, 5, 6];
    const NUM_CONST: u32 = 3;

    println!("Arreglo original: {:?} ", arreglo);
    println!("Multiplicar el arreglo por {} ", NUM_CONST);
    for arreglo in arreglo.iter_mut() {
        *arreglo *= NUM_CONST;
    }

    println!("Arreglo modificado: {:?} ", arreglo);
}
