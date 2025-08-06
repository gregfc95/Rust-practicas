/*
3- Definir la función llamada suma_pares que recibe como parámetro un arreglo de
números enteros y retorna la suma de los números pares.

4- Definir la función llamada cantidad_impares que recibe como parámetro un arreglo de
números enteros y retorna la cantidad de números impares.

*/

use std::io::stdin;
pub fn ej3() {
    let mut numeros: Vec<i32> = Vec::new();
    let mut numero: i32;
    loop {
        println!("Ingrese un número: ");
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada");
        numero = input.trim().parse().expect("Error al convertir el número");
        numeros.push(numero);
        if numero == 0 {
            break;
        }
    }
}

fn sum_pares(numeros: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for &num in numeros {
        if num % 2 == 0 {
            sum += num;
        }
    }
    sum
}

fn sum_impares(numeros: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for &num in numeros {
        if num % 2 != 0 {
            sum += num;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_pares() {
        let numeros = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_pares(&numeros), 6);
    }

    #[test]
    fn test_sum_impares() {
        let numeros = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_impares(&numeros), 9);
    }
}
