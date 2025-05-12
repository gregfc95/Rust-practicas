/*
6- Escribir un programa que defina una estructura Estudiante que tenga campos para el
nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se
conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes
métodos:
❖ Examen:
➢ new: que pasando los parámetros correspondientes, crea un Examen y lo
retorna.
❖ Estudiante:
➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo
retorna.
➢ obtener_promedio: retorna el promedio de las notas.
➢ obtener_calificacion_mas_alta: retorna la nota más alta.
➢ obtener_calificacion_mas_baja: retorna la nota más baja.
*/

pub struct Estudiante {
    nombre: String,
    id: u32,
    calificaciones: Vec<Examen>,
}
pub struct Examen {
    nombre: String,
    nota: f32,
}

impl Estudiante {
    pub fn new(nombre: String, id: u32, calificaciones: Vec<Examen>) -> Self {
        Estudiante {
            nombre,
            id,
            calificaciones,
        }
    }
}

impl Examen {
    pub fn new(nombre: String, nota: f32) -> Self {
        Examen { nombre, nota }
    }

    pub fn get_nota(&self) -> f32 {
        self.nota
    }
}
