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
    pub fn obtener_promedio(&self) -> Option<f32> {
        if self.calificaciones.is_empty() {
            return None;
        }
        let mut suma: f32 = 0.0;
        for calificacion in self.calificaciones.iter() {
            suma += calificacion.get_nota();
        }
        Some(suma / self.calificaciones.len() as f32)
    }
    //➢ obtener_calificacion_mas_alta: retorna la nota más alta.
    pub fn obtener_calificacion_mas_alta(&self) -> Option<f32> {
        if self.calificaciones.is_empty() {
            return None;
        }
        let mut max: f32 = -1.0;
        for calificacion in self.calificaciones.iter() {
            max = Self::get_max(calificacion.get_nota(), max);
        }
        Some(max)
        //act.max(max) // Usa el método nativo de f32
    }

    pub fn obtener_calificacion_mas_baja(&self) -> Option<f32> {
        if self.calificaciones.is_empty() {
            return None;
        }
        let mut min: f32 = 999.0;
        for calificacion in self.calificaciones.iter() {
            min = Self::get_min(calificacion.get_nota(), min);
        }
        Some(min)
        //act.min(min) // Usa el método nativo de f32
    }
    //Helper privado
    fn get_max(act: f32, max: f32) -> f32 {
        if act > max { act } else { max }
    }
    fn get_min(act: f32, min: f32) -> f32 {
        if act < min { act } else { min }
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

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_new() {
        let e = Examen::new("e".to_string(), 5.0);
        assert_eq!(e.nombre, "e".to_string());
        assert_eq!(e.get_nota(), 5.0);

        let e1 = Examen::new("e1".to_string(), 0.0);
        assert_eq!(e1.nombre, "e1".to_string());
        assert_eq!(e1.get_nota(), 0.0);
    }

    #[test]
    fn test_estudiante() {
        let e1 = Examen::new("e1".to_string(), 0.0);
        let e2 = Examen::new("e2".to_string(), 1.0);
        let e3 = Examen::new("e3".to_string(), 2.0);
        let e4 = Examen::new("e4".to_string(), 3.0);
        let e5 = Examen::new("e5".to_string(), 4.0);
        let e6 = Examen::new("e6".to_string(), 5.0);

        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, vec![e1, e2, e3, e4, e5, e6]);

        assert_eq!(estudiante1.obtener_promedio(), Some(2.5));
        assert_eq!(estudiante1.obtener_calificacion_mas_alta(), Some(5.0));
        assert_eq!(estudiante1.obtener_calificacion_mas_baja(), Some(0.0));
        assert_eq!(estudiante1.nombre, "Juan".to_string());
        assert_eq!(estudiante1.id, 196394);
    }

    #[test]
    fn test_estudiante_vacio() {
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, Vec::new());
        assert_eq!(estudiante1.obtener_promedio(), None);
        assert_eq!(estudiante1.obtener_calificacion_mas_alta(), None);
        assert_eq!(estudiante1.obtener_calificacion_mas_baja(), None);
    }
}
