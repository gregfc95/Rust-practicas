/* 2- Definir la función llamada es_primo que recibe un número entero positivo mayor a 1 y
retorna true si es primo, false caso contrario.
*/

use std::io::stdin;

pub fn ej2() {
    let num: i32;
    println!("Ingrese un número: ");
    let mut input: String = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    num = input.trim().parse().expect("Error al convertir el número");
    let es_primo = es_primo(num);
}

fn es_primo(num: i32) -> bool {
    if num <= 1 {
        return false;
    }
    // Optimización: solo verificar hasta la raíz cuadrada
    //let limite = (num as f64).sqrt() as i32 + 1;
    // for i in 2.. limite{}
    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_es_primo() {
        assert_eq!(es_primo(2), true);
        assert_eq!(es_primo(3), true);
        assert_eq!(es_primo(4), false);
    }

    #[test]
    fn test_no_es_primo() {
        assert_eq!(es_primo(1), false);
    }
}
