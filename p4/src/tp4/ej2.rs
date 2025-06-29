/* 2- Dado el siguiente struct:
a- Escriba una función que reciba un vector de personas y otro parámetro que indica un salario y retorna un listado de personas donde el salario es mayor al parámetro recibido.
b- Escriba una función que reciba un vector de personas, edad y el nombre de una ciudad,
y retorna las personas mayores al parámetro edad y que viven en el valor del parámetro
ciudad.
c- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si todas las personas viven en la ciudad pasada por parámetro, false caso
contrario.
d- Escriba una función que reciba un vector de personas y un nombre de una ciudad y
retorna true si al menos vive una persona en la ciudad pasada por parámetro,, false caso
contrario.
e- Escriba una función que reciba un arreglo de personas y una persona y retorna true si la
persona existe en el arreglo, false caso contrario
f -Escriba una función que reciba un arreglo de personas y retorna un arreglo con las
edades de las personas.
g - Escriba una función que reciba un arreglo de personas y retorna la persona con el menor
salario y la persona con el mayor salario, en caso de que haya más de una persona en cada
categoría desempatar por la edad más grande.

Nota: Implemente todos los métodos y traits que considere para resolver los ejercicios.
Todos los ejercicios deben resolverse con iterator y closure
*/

// 'a Eso se llama un "lifetime annotation" (anotación de tiempo de vida) en Rust.
//"todas estas referencias (&'a str) viven al menos tanto como 'a".

//#[derive(PartialEq)]
struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}
impl Persona<'_> {
    fn new<'a>(
        nombre: &'a str,
        apellido: &'a str,
        direccion: &'a str,
        ciudad: &'a str,
        salario: f64,
        edad: u8,
    ) -> Persona<'a> {
        Persona {
            nombre,
            apellido,
            direccion,
            ciudad,
            salario,
            edad,
        }
    }
}
pub trait Filtros {
    fn salario_mayor_que(&self, salario: f64) -> bool;
    fn mayor_edad_que(&self, edad: u8) -> bool;
    fn vive_en(&self, ciudad: &str) -> bool;
    fn match_persona(&self, nombre: &str, apellido: &str) -> bool;
    fn match_persona_partial(&self, nombre: &str, apellido: &str) -> bool;
    fn max_edad(&self, edad: &u8) -> u8;
    fn mejor_min<'a>(a: &'a Persona<'a>, b: &'a Persona<'a>) -> &'a Persona<'a>;
    fn mejor_max<'a>(a: &'a Persona<'a>, b: &'a Persona<'a>) -> &'a Persona<'a>;
}

impl Filtros for Persona<'_> {
    fn salario_mayor_que(&self, salario: f64) -> bool {
        self.salario > salario
    }

    fn mayor_edad_que(&self, edad: u8) -> bool {
        self.edad > edad
    }

    fn vive_en(&self, ciudad: &str) -> bool {
        self.ciudad == ciudad
    }

    fn match_persona(&self, nombre: &str, apellido: &str) -> bool {
        self.nombre == nombre && self.apellido == apellido
    }
    fn match_persona_partial(&self, nombre: &str, apellido: &str) -> bool {
        self.nombre.eq_ignore_ascii_case(nombre) && self.apellido.eq_ignore_ascii_case(apellido)
    }

    fn max_edad(&self, edad: &u8) -> u8 {
        if self.edad < *edad {
            return *edad;
        } else {
            return self.edad;
        }
    }

    fn mejor_min<'a>(a: &'a Persona<'a>, b: &'a Persona<'a>) -> &'a Persona<'a> {
        if a.salario < b.salario {
            return a;
        } else if b.salario < a.salario {
            return b;
        } else {
            if a.edad >= b.edad {
                return a;
            } else {
                return b;
            }
        }
    }
    fn mejor_max<'a>(a: &'a Persona<'a>, b: &'a Persona<'a>) -> &'a Persona<'a> {
        if a.salario > b.salario {
            return a;
        } else if b.salario > a.salario {
            return b;
        } else {
            if a.edad >= b.edad {
                return a;
            } else {
                return b;
            }
        }
    }
}

/*
vec contiene datos que viven 'a
devolvés referencias que viven 'a
no generás nuevas referencias, solo devolvés algunas de las existentes
*/
fn personas_mayor_salario<'a>(vec: &'a Vec<Persona<'a>>, salario: f64) -> Vec<&'a Persona<'a>> {
    vec.iter()
        .filter(|&p| p.salario_mayor_que(salario))
        .collect()
}

fn personas_mayor_edad_y_viven_en_ciudad<'a>(
    vec: &'a Vec<Persona<'a>>,
    edad: u8,
    ciudad: &str,
) -> Vec<&'a Persona<'a>> {
    vec.iter()
        .filter(|&p| p.mayor_edad_que(edad) && p.vive_en(ciudad))
        .collect()
}

fn all_viven_en_ciudad<'a>(vec: &'a Vec<Persona<'a>>, ciudad: &str) -> bool {
    vec.iter().filter(|&p| p.vive_en(ciudad)).count() == vec.len()
}

fn at_least_viven_en_ciudad<'a>(vec: &'a Vec<Persona<'a>>, ciudad: &str) -> bool {
    vec.iter().any(|p| p.vive_en(ciudad))
}

fn existe_persona<'a>(vec: &'a Vec<Persona<'a>>, persona: &Persona<'a>) -> bool {
    vec.iter()
        .any(|p| p.match_persona(persona.nombre, persona.apellido))
}

/* fn ver_persona_existe<'a>(vec: &'a Vec<Persona<'a>>, persona: &Persona<'a>) -> bool {
    vec.iter().any(|p| p == persona)
} */

fn return_edades_vec<'a>(vec: &'a Vec<Persona<'a>>) -> Vec<&'a u8> {
    vec.iter().map(|p| &p.edad).collect()
}

fn min_max_salario<'a>(vec: &'a Vec<Persona<'a>>) -> (&'a Persona<'a>, &'a Persona<'a>) {
    let mut min = &vec[0];
    let mut max = &vec[0];
    vec.iter().for_each(|p| {
        min = Persona::mejor_min(min, p);
        max = Persona::mejor_max(max, p);
    });
    (min, max)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    //contructores builders
    fn persona1<'a>() -> Persona<'a> {
        Persona::new("Juan", "Perez", "Calle Falsa 123", "Ciudad A", 50000.0, 30)
    }
    fn persona2<'a>() -> Persona<'a> {
        Persona::new("Maria", "Lopez", "Calle Falsa 123", "Ciudad B", 60000.0, 25)
    }
    fn persona3<'a>() -> Persona<'a> {
        Persona::new("Pedro", "Gomez", "Calle Falsa 123", "Ciudad C", 55000.0, 35)
    }
    fn vec_personas<'a>() -> Vec<Persona<'a>> {
        vec![persona1(), persona2(), persona3()]
    }
    //Test de traits
    #[test]
    fn trait_salario_mayor_que_positivo() {
        let p = persona1();
        assert!(p.salario_mayor_que(40000.0));
    }
    #[test]
    fn trait_salario_mayor_que_negativo() {
        let p = persona1();
        assert!(!p.salario_mayor_que(60000.0));
    }
    #[test]
    fn trait_mayor_edad_que_positivo() {
        let p = persona1();
        assert!(p.mayor_edad_que(25));
    }
    #[test]
    fn trait_mayor_edad_que_negativo() {
        let p = persona1();
        assert!(!p.mayor_edad_que(30))
    }
    #[test]
    fn trait_vive_en_ciudad_positivo() {
        let p = persona1();
        assert!(p.vive_en("Ciudad A"));
    }
    #[test]
    fn trait_vive_en_ciudad_negativo() {
        let p = persona1();
        assert!(!p.vive_en("Ciudad B"));
    }
    #[test]
    fn trait_match_persona_positivo() {
        let p = persona1();
        assert!(p.match_persona("Juan", "Perez"));
    }
    #[test]
    fn trait_match_persona_negativo() {
        let p = persona1();
        assert!(!p.match_persona("juan", "perez"));
    }
    #[test]
    fn trait_match_persona_partial_positivo() {
        let p = persona1();
        assert!(p.match_persona_partial("juan", "perez"));
    }
    #[test]
    fn trait_match_persona_partial_negativo() {
        let p = persona1();
        assert!(!p.match_persona_partial("maria", "perez"));
    }
    #[test]
    fn trait_max_edad() {
        let p = persona1();
        let edad: &u8 = &25;
        assert_eq!(p.max_edad(edad), 30);
    }
    #[test]
    fn trait_mejor_min() {
        let p1 = persona1();
        let p2 = persona2();
        let mejor = Persona::mejor_min(&p1, &p2);
        assert_eq!(mejor.nombre, "Juan");
    }
    #[test]
    fn trait_mejor_max() {
        let p1 = persona1();
        let p2 = persona2();
        let mejor = Persona::mejor_max(&p1, &p2);
        assert_eq!(mejor.nombre, "Maria");
    }

    //Test de Vec
    #[test]
    fn test_personas_mayor_salario() {
        let vec = vec_personas();
        let resultado = personas_mayor_salario(&vec, 55000.0);
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0].nombre, "Maria");
    }
    #[test]
    fn test_personas_mayor_edad_y_viven_en_ciudad() {
        let vec = vec_personas();
        let resultado = personas_mayor_edad_y_viven_en_ciudad(&vec, 30, "Ciudad C");
        assert_eq!(resultado.len(), 1);
        assert_eq!(resultado[0].nombre, "Pedro");
    }
    #[test]
    fn test_personas_mayor_edad_y_viven_en_ciudad_vacio() {
        let vec = vec_personas();
        let resultado_vacio = personas_mayor_edad_y_viven_en_ciudad(&vec, 40, "Ciudad C");
        assert!(resultado_vacio.is_empty());
    }

    #[test]
    fn test_all_viven_en_ciudad_positivo() {
        let p4 = Persona::new("Ana", "Diaz", "Calle Falsa 456", "Ciudad A", 70000.0, 40);
        let vec2 = vec![persona1(), p4];
        assert!(all_viven_en_ciudad(&vec2, "Ciudad A"));
    }
    #[test]
    fn test_all_viven_en_ciudad_negativo() {
        let vec = vec_personas();
        assert!(!all_viven_en_ciudad(&vec, "Ciudad A"));
    }

    #[test]
    fn test_at_least_viven_en_ciudad() {
        let vec = vec_personas();
        assert!(at_least_viven_en_ciudad(&vec, "Ciudad A"));
        assert!(!at_least_viven_en_ciudad(&vec, "Ciudad Z"));
    }

    #[test]
    fn test_existe_persona() {
        let vec = vec_personas();
        let p = persona1();
        assert!(existe_persona(&vec, &p));
        let p_fake = Persona::new("No", "Existe", "X", "Ciudad X", 0.0, 0);
        assert!(!existe_persona(&vec, &p_fake));
    }

    #[test]
    fn test_return_edades_vec() {
        let vec = vec_personas();
        let edades = return_edades_vec(&vec);
        assert_eq!(edades.len(), 3);
        assert!(edades.contains(&&30));
        assert!(edades.contains(&&25));
        assert!(edades.contains(&&35));
    }

    #[test]
    fn test_min_max_salario() {
        let vec = vec_personas();
        let (min, max) = min_max_salario(&vec);
        assert_eq!(min.nombre, "Juan");
        assert_eq!(max.nombre, "Maria");
    }
}
