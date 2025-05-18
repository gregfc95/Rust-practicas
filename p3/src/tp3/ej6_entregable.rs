/*
Jose gregorio fernandez campos

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

//Entregable: Implementar una funcion
minimo 2 tests

Deberán agregar una funcionalidad al ejercicio que permita retornar un informe detallado del rendimiento académico de un estudiante.

Este informe debe incluir:
Nombre e identificación del estudiante.,
Cantidad total de exámenes rendidos.,
Promedio general de notas.,
Nota más alta y la materia correspondiente.,
Nota más baja y la materia correspondiente.,

🔧 La funcionalidad deberá implementarse como un método asociado del estudiante llamado generar_informe.
En caso de que el estudiante no haya rendido ningún examen, no debe retornarse ningún informe.

📌 Requisitos:
La funcionalidad debe integrarse naturalmente con las estructuras ya definidas.,
Se espera una solución robusta ante distintas situaciones, incluyendo estudiantes sin exámenes.,
Se debe acompañar con al menos dos tests unitarios que validen su correcto funcionamiento.,

🛠️ Durante la evaluación:
Deberán presentar una estrategia clara para encarar esta funcionalidad, explicando cómo organizarán los datos y qué métodos planean utilizar.
La versión final (V2) deberá ajustarse fielmente a la estrategia presentada.

*/
#[derive(Debug, Clone)]
pub struct Estudiante {
    nombre: String,
    id: u32,
    calificaciones: Vec<Examen>,
}
#[derive(Debug, Clone)]
pub struct Examen {
    nombre: String,
    nota: f32,
}
#[derive(Debug, Clone)]
pub struct Informe {
    estudiante: Estudiante,
    cant_examenes: u32,
    promedio: Option<f32>,
    tupla_calf_mas_alta: Option<(f32, String)>,
    tupla_calf_mas_baja: Option<(f32, String)>,
}
impl Informe {
    pub fn new(
        estudiante: Estudiante,
        cant_examenes: u32,
        promedio: Option<f32>,
        tupla_calf_mas_alta: Option<(f32, String)>,
        tupla_calf_mas_baja: Option<(f32, String)>,
    ) -> Self {
        Informe {
            estudiante,
            cant_examenes,
            promedio,
            tupla_calf_mas_alta,
            tupla_calf_mas_baja,
        }
    }
}

impl Estudiante {
    pub fn new(nombre: String, id: u32, calificaciones: Vec<Examen>) -> Self {
        Estudiante {
            nombre,
            id,
            calificaciones,
        }
    }

    pub fn generar_informe(&self) -> Option<Informe> {
        if self.calificaciones.is_empty() {
            return None;
        } else {
            //Llamamos al metodo new de informe
            Some(Informe::new(
                self.clone(),
                self.get_calificaciones_validas(),
                self.obtener_promedio(),
                self.get_tupla_mas_alta(),
                self.get_tupla_mas_baja(),
            ))
        }
    }

    fn get_nombre_mas_alto(&self) -> Option<String> {
        if self.calificaciones.is_empty() {
            return None;
        }
        let max_nota = self.obtener_calificacion_mas_alta()?;
        for calificacion in self.calificaciones.iter() {
            if calificacion.get_nota() == max_nota {
                return Some(calificacion.get_nombre().to_string());
            }
        }
        None
    }

    fn get_nombre_mas_bajo(&self) -> Option<String> {
        if self.calificaciones.is_empty() {
            return None;
        }
        let min_nota = self.obtener_calificacion_mas_baja()?;
        for calificacion in self.calificaciones.iter() {
            if calificacion.get_nota() == min_nota {
                return Some(calificacion.get_nombre().to_string());
            }
        }
        None
    }
    // Iteramos buscando solo calificaciones sin negativos, ya estamos protegidos si es None en generar_informe
    fn get_calificaciones_validas(&self) -> u32 {
        let mut contador_valido: u32 = 0;
        for calificacion in self.calificaciones.iter() {
            let nota = calificacion.get_nota();
            if nota >= 0.0 {
                contador_valido += 1;
            }
        }
        contador_valido
    }

    //helper tupla_alta llamamos al nombre y a la calificacion mas alta, si no existe devuelve none
    fn get_tupla_mas_alta(&self) -> Option<(f32, String)> {
        let max_nota = self.obtener_calificacion_mas_alta()?;
        let nombre = self.get_nombre_mas_alto()?;
        Some((max_nota, nombre))
    }
    //helper tupla_baja llamamos al nombre y a la calificacion mas baja, si no existe devuelve none
    fn get_tupla_mas_baja(&self) -> Option<(f32, String)> {
        let min_nota = self.obtener_calificacion_mas_baja()?;
        let nombre = self.get_nombre_mas_bajo()?;
        Some((min_nota, nombre))
    }

    //Agregar proteccion contra promedio negativo, mientras hacia tests me di cuenta que negativos era valido
    pub fn obtener_promedio(&self) -> Option<f32> {
        if self.calificaciones.is_empty() {
            return None;
        }
        let mut suma: f32 = 0.0;
        let mut contador_valido: u32 = 0;
        for calificacion in self.calificaciones.iter() {
            let nota = calificacion.get_nota();
            if nota >= 0.0 {
                suma += nota;
                contador_valido += 1;
            }
        }
        if contador_valido == 0 {
            return None;
        }
        Some(suma / contador_valido as f32)
    }

    //Agregar proteccion contra notas negativas, mientras hacia tests me di cuenta que negativos era valido
    pub fn obtener_calificacion_mas_alta(&self) -> Option<f32> {
        if self.calificaciones.is_empty() {
            return None;
        }
        let mut max: f32 = -1.0;
        for calificacion in self.calificaciones.iter() {
            max = Self::get_max(calificacion.get_nota(), max);
        }
        if max < 0.0 {
            return None;
        }
        Some(max)
    }
    //Agregar proteccion contra notas negativas, mientras hacia tests me di cuenta que negativos era valido
    pub fn obtener_calificacion_mas_baja(&self) -> Option<f32> {
        if self.calificaciones.is_empty() {
            return None;
        }
        let mut min: f32 = 999.0;
        for calificacion in self.calificaciones.iter() {
            if calificacion.get_nota() < 0.0 {
                continue;
            }
            min = Self::get_min(calificacion.get_nota(), min);
        }
        if min >= 999.0 {
            return None;
        }
        Some(min)
    }

    //Helper privado
    fn get_max(act: f32, max: f32) -> f32 {
        if act > max { act } else { max }
    }
    fn get_min(act: f32, min: f32) -> f32 {
        if act < min { act } else { min }
    }
    //helper para matchear estudiante
    pub fn es_igual(&self, otro: &Self) -> bool {
        self.nombre == otro.nombre && self.id == otro.id
    }
}

impl Examen {
    pub fn new(nombre: String, nota: f32) -> Self {
        Examen { nombre, nota }
    }

    pub fn get_nota(&self) -> f32 {
        self.nota
    }
    pub fn get_nombre(&self) -> String {
        self.nombre.clone()
    }
}

#[cfg(test)]
mod tests {
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
    fn test_promedio_vacio() {
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, Vec::new());
        assert_eq!(estudiante1.obtener_promedio(), None);
    }
    #[test]
    fn test_calificacion_alta_vacio() {
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, Vec::new());
        assert_eq!(estudiante1.obtener_calificacion_mas_alta(), None);
    }
    #[test]
    fn test_calificacion_baja_vacio() {
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, Vec::new());
        assert_eq!(estudiante1.obtener_calificacion_mas_baja(), None);
    }

    #[test]
    fn test_calificacion_alta_negativa() {
        let e1 = Examen::new("e1".to_string(), 0.0);
        let e2 = Examen::new("e2".to_string(), -1.0);
        let e3 = Examen::new("e3".to_string(), -2.0);
        let e4 = Examen::new("e4".to_string(), -3.0);
        let e5 = Examen::new("e5".to_string(), -4.0);
        let e6 = Examen::new("e6".to_string(), -5.0);

        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, vec![e1, e2, e3, e4, e5, e6]);

        assert_eq!(estudiante1.obtener_calificacion_mas_alta(), Some(0.0));
    }

    #[test]
    fn test_calificacion_baja_negativa() {
        let e1 = Examen::new("e1".to_string(), 0.0);
        let e2 = Examen::new("e2".to_string(), -1.0);
        let e3 = Examen::new("e3".to_string(), -2.0);
        let e4 = Examen::new("e4".to_string(), -3.0);
        let e5 = Examen::new("e5".to_string(), -4.0);
        let e6 = Examen::new("e6".to_string(), -5.0);
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, vec![e1, e2, e3, e4, e5, e6]);

        assert_eq!(estudiante1.obtener_calificacion_mas_baja(), Some(0.0));
    }

    #[test]
    fn test_promedio_negativo_con_cero() {
        let e1 = Examen::new("e1".to_string(), 0.0);
        let e2 = Examen::new("e2".to_string(), -1.0);
        let e3 = Examen::new("e3".to_string(), -2.0);
        let e4 = Examen::new("e4".to_string(), -3.0);
        let e5 = Examen::new("e5".to_string(), -4.0);
        let e6 = Examen::new("e6".to_string(), -5.0);
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, vec![e1, e2, e3, e4, e5, e6]);

        assert_eq!(estudiante1.obtener_promedio(), Some(0.0));
    }
    #[test]
    fn test_promedio_negativo() {
        let e1 = Examen::new("e1".to_string(), -1.0);
        let e2 = Examen::new("e2".to_string(), -2.0);
        let e3 = Examen::new("e3".to_string(), -3.0);
        let e4 = Examen::new("e4".to_string(), -4.0);
        let e5 = Examen::new("e5".to_string(), -5.0);
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, vec![e1, e2, e3, e4, e5]);

        assert_eq!(estudiante1.obtener_promedio(), None);
    }

    //Informe
    #[test]
    fn test_informe_creacion() {
        //Constructor
        let e1 = Examen::new("e1".to_string(), 0.0);
        let e2 = Examen::new("e2".to_string(), 1.0);
        let e3 = Examen::new("e3".to_string(), 2.0);
        let e4 = Examen::new("e4".to_string(), 3.0);
        let e5 = Examen::new("e5".to_string(), 4.0);

        let vec_examen = vec![e1, e2, e3, e4, e5];
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, vec_examen);
        let informe = estudiante1.generar_informe();

        //asserts
        assert_eq!(informe.is_none(), false);
        let informe = informe.unwrap();

        assert!(informe.estudiante.es_igual(&estudiante1));
        assert_eq!(informe.cant_examenes, 5);
        assert_eq!(informe.promedio, Some(2.0));
        assert_eq!(informe.tupla_calf_mas_alta, Some((4.0, "e5".to_string())));
        assert_eq!(informe.tupla_calf_mas_baja, Some((0.0, "e1".to_string())));
    }

    #[test]
    fn test_informe_creacion_negativo() {
        //Constructor
        let e1 = Examen::new("e1".to_string(), 0.0);
        let e2 = Examen::new("e2".to_string(), -1.0);
        let e3 = Examen::new("e3".to_string(), -2.0);
        let e4 = Examen::new("e4".to_string(), -3.0);
        let e5 = Examen::new("e5".to_string(), -4.0);

        let vec_examen = vec![e1, e2, e3, e4, e5];
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, vec_examen);
        let informe = estudiante1.generar_informe();

        //asserts
        assert_eq!(informe.is_none(), false);
        let informe = informe.unwrap();

        assert!(informe.estudiante.es_igual(&estudiante1));
        assert_eq!(informe.cant_examenes, 1);
        assert_eq!(informe.promedio, Some(0.0));
        assert_eq!(informe.tupla_calf_mas_alta, Some((0.0, "e1".to_string())));
        assert_eq!(informe.tupla_calf_mas_baja, Some((0.0, "e1".to_string())));
    }

    #[test]
    fn test_informe_creacion_negativo_sin_cero() {
        //Constructor
        let e1 = Examen::new("e1".to_string(), -1.0);
        let e2 = Examen::new("e2".to_string(), -2.0);
        let e3 = Examen::new("e3".to_string(), -3.0);
        let e4 = Examen::new("e4".to_string(), -4.0);
        let e5 = Examen::new("e5".to_string(), -5.0);

        let vec_examen = vec![e1, e2, e3, e4, e5];
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, vec_examen);
        let informe = estudiante1.generar_informe();

        //asserts
        assert_eq!(informe.is_none(), false);
        let informe = informe.unwrap();

        assert!(informe.estudiante.es_igual(&estudiante1));
        assert_eq!(informe.cant_examenes, 0);
        assert_eq!(informe.promedio, None);
        assert_eq!(informe.tupla_calf_mas_alta, None);
        assert_eq!(informe.tupla_calf_mas_baja, None);
    }

    #[test]
    fn test_calificaciones_baja_iguales() {
        //Constructor
        let e1 = Examen::new("e1".to_string(), 5.0);
        let e2 = Examen::new("e2".to_string(), 5.0);
        let e3 = Examen::new("e3".to_string(), 9.0);
        let e4 = Examen::new("e4".to_string(), 9.0);
        let e5 = Examen::new("e5".to_string(), 9.0);

        let vec_examen = vec![e1, e2, e3, e4, e5];
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, vec_examen);

        let nombre_mas_bajo = estudiante1.get_nombre_mas_bajo();
        assert_eq!(nombre_mas_bajo.unwrap(), "e1".to_string());
        assert_eq!(estudiante1.obtener_calificacion_mas_baja(), Some(5.0));
    }

    #[test]
    fn test_calificaciones_alta_iguales() {
        //Constructor
        let e1 = Examen::new("e1".to_string(), 5.0);
        let e2 = Examen::new("e2".to_string(), 5.0);
        let e3 = Examen::new("e3".to_string(), 9.0);
        let e4 = Examen::new("e4".to_string(), 9.0);
        let e5 = Examen::new("e5".to_string(), 9.0);

        let vec_examen = vec![e1, e2, e3, e4, e5];
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, vec_examen);

        let nombre_mas_alto = estudiante1.get_nombre_mas_alto();
        assert_eq!(nombre_mas_alto.unwrap(), "e3".to_string());
        assert_eq!(estudiante1.obtener_calificacion_mas_alta(), Some(9.0));
    }

    #[test]
    fn test_informe_vacio() {
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, Vec::new());
        let informe = estudiante1.generar_informe();
        assert_eq!(informe.is_none(), true);
    }

    #[test]
    fn test_nombre_mas_alto_vacio() {
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, Vec::new());
        let nombre_mas_alto = estudiante1.get_nombre_mas_alto();
        assert_eq!(nombre_mas_alto.is_none(), true);
    }

    #[test]
    fn test_nombre_mas_bajo_vacio() {
        let estudiante1 = Estudiante::new("Juan".to_string(), 196394, Vec::new());
        let nombre_mas_bajo = estudiante1.get_nombre_mas_bajo();
        assert_eq!(nombre_mas_bajo.is_none(), true);
    }
}
