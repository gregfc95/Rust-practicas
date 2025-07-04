/*
1- Escriba una función que reciba un vector de números enteros y retorna la cantidad de
números primos.
Cree un trait para la determinación del número primo e impleméntelo según corresponda.
Utilice la función iter sobre el vector y
aplique un closure para resolverlo.

Trait:es como una interfaz en otros lenguajes (Java, C#). Define un comportamiento común que distintos tipos pueden implementar.
.iter() es un método de los vectores (y otras colecciones) que te permite iterar sin consumir los elementos. Retorna un iterador sobre referencias.
Closure:Un closure es una función anónima, es decir, una función que podés definir en línea. Se usa con mucha frecuencia en combinaciones con .iter(), .map(), .filter(), etc.
*/

//Trait:es como una interfaz en otros lenguajes (Java, C#). Define un comportamiento común que distintos tipos pueden implementar.
pub trait EsPrimo {
    fn es_primo(&self) -> bool;
}

impl EsPrimo for i32 {
    fn es_primo(&self) -> bool {
        if *self <= 1 {
            return false;
        }
        for i in 2..*self {
            if *self % i == 0 {
                return false;
            }
        }
        true
    }
}
//usize: es un entero sin signo de 32 bits (0 a 4,294,967,295) es el tipo más seguro y natural para representar "cantidad de elementos" en colecciones.
pub fn primos(vec: &Vec<i32>) -> usize {
    vec.iter().filter(|n| n.es_primo()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_negativo() {
        let vec: Vec<i32> = vec![-2, -4, -5, -6, -7, -8, -9, -10, -11, -12, -13, -14];
        assert_eq!(primos(&vec), 0);
    }

    #[test]
    fn test_num_cero() {
        let vec: Vec<i32> = vec![0];
        assert_eq!(primos(&vec), 0);
    }

    #[test]
    fn test_num_primos() {
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(primos(&vec), 3);
    }

    #[test]
    fn test_true_primo() {
        let num: i32 = 2;
        assert!(num.es_primo())
    }

    #[test]
    fn test_false_primo() {
        let num: i32 = 0;
        assert!(!num.es_primo())
    }
}
