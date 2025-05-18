/* 1-Definir la función llamada es_par que recibe como parámetro un número entero y retorna
true si el número es par, false caso contrario.
*/

use std::io::stdin;

pub fn ej1() {
    let mut input: String = String::new();
    println!("Ingrese un numero: ");

    stdin()
        .read_line(&mut input)
        .expect("Error al leer el numero");
    let numero: i32 = input.trim().parse().expect("Error al convertir a numero");
}
pub fn es_par(numero: i32) -> bool {
    numero % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_par() {
        //Numero pares
        assert_eq!(es_par(2), true);
        assert_eq!(es_par(0), true);
    }

    #[test]
    fn test_no_es_par() {
        //Numero impares
        assert_eq!(es_par(1), false);
        assert_eq!(es_par(3), false);
    }
}
