/*
Nota: Para todos los ejercicios realizar los tests de unidad correspondientes.
1- Escribir un programa que defina una estructura Persona que tenga campos para el
nombre, la edad y la dirección(que puede ser nulo al momento de la creación de una
persona). Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea una Persona y la retorna.
➢ to_string: que retorna un string con los datos de la persona concatenados sobre el
mensaje ejecutado por ej:
person.to_string() , donde person es una variable del tipo Persona.
➢ obtener_edad: retorna la edad de la persona.
➢ actualizar_direccion(nueva_direccion)
*/
pub struct Persona {
    nombre: String,
    edad: u16,
    direccion: Option<String>,
}
impl Persona {
    pub fn new(nombre: String, edad: u16, direccion: Option<String>) -> Self {
        Persona {
            nombre,
            edad,
            direccion,
        }
    }

    pub fn to_string(&self) -> String {
        //.as_deref() Convierte Option<String> a Option<&str>, trabajamos con la referencia &str
        //.unwrap_or("Sin direccion") si Option es Some, devuelve valor &str, sino devuelve "Sin direccion"
        let direccion_str = self.direccion.as_deref().unwrap_or("Sin direccion");
        format!(
            "Nombre: {}, Edad: {}, Direccion: {:?}",
            self.nombre, self.edad, direccion_str
        )
    }

    pub fn obtener_edad(&self) -> u16 {
        self.edad
    }

    pub fn actualizar_direccion(&mut self, new_dir: &str) {
        self.direccion = Some(new_dir.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crear_persona() {
        let persona = Persona::new("Carla".to_string(), 30, None);
        assert_eq!(persona.nombre, "Carla");
        assert_eq!(persona.edad, 30);
        assert_eq!(persona.direccion, None);
    }
    #[test]
    fn persona_to_string() {
        let persona: Persona = Persona::new("Juan".to_string(), 99, Some("calle 123".to_string()));
        assert_eq!(
            persona.to_string(),
            "Nombre: Juan, Edad: 99, Direccion: \"calle 123\""
        );
    }

    #[test]
    fn persona_obtener_edad() {
        let persona: Persona = Persona::new("Juan".to_string(), 99, Some("calle 123".to_string()));
        assert_eq!(persona.obtener_edad(), 99);
    }

    #[test]
    fn persona_actualizar_direccion() {
        let nueva_direccion: String = "Calle 724".to_string();
        let mut persona: Persona = Persona::new("Juan".to_string(), 99, None);
        persona.actualizar_direccion(&nueva_direccion);
        assert_eq!(persona.direccion, Some(nueva_direccion));
        assert_ne!(persona.direccion, None);
    }
}
