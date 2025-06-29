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
pub trait Filtros {
    fn salario_mayor_que(&self, salario: f64) -> bool;
    fn mayor_edad_que(&self, edad: u8) -> bool;
    fn vive_en(&self, ciudad: &str) -> bool;
    fn match_persona(&self, nombre: &str, apellido: &str) -> bool;
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
