/*
9.-Dada una cadena de veterinarias se desea implementar un sistema de atención de
pacientes para cada veterinaria, de la veterinaria se conoce el nombre, la dirección y un id.
Para la atención de mascotas se requiere administrar una cola de atención. De la mascota
se conoce el nombre, la edad, el tipo de animal(perro, gato, caballo, otros) y su dueño. Del
dueño se conoce el nombre, la dirección y un teléfono de contacto. Luego de la atención se
desea tener un registro de las atenciones realizadas guardando los datos de la mascota, el
diagnóstico final, tratamiento y fecha de la próxima visita si es que se requiere.
Dado todo lo mencionado anteriormente implemente los métodos para realizar las
siguientes acciones:
➔ crear una veterinaria.
➔ agregar una nueva mascota a la cola de atención de la veterinaria.
➔ agregar una nueva mascota a la cola de atención pero que sea la siguiente
en atender porque tiene la máxima prioridad.
➔ atender la próxima mascota de la cola.
➔ eliminar una mascota específica de la cola de atención dado que se retira.
➔ registrar una atención.
➔ buscar una atención dado el nombre de la mascota, el nombre del dueño y el
teléfono.
➔ modificar el diagnóstico de una determinada atención.
➔ modificar la fecha de la próxima visita de una determinada atención.
➔ eliminar una determinada atención.
Nota: para la fecha utilice lo implementado en el punto 3.
*/
use crate::tp3::ej3::Fecha;
use std::collections::VecDeque;
#[derive(Debug, Clone)]
pub enum Animal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

impl Animal {
    pub fn igual(&self, otro: &Animal) -> bool {
        match (self, otro) {
            (Animal::Perro, Animal::Perro)
            | (Animal::Gato, Animal::Gato)
            | (Animal::Caballo, Animal::Caballo)
            | (Animal::Otros, Animal::Otros) => true,
            _ => false,
        }
    }
}
#[derive(Debug, Clone)]
struct Dueño {
    nombre: String,
    direccion: String,
    telefono: String,
}
#[derive(Debug, Clone)]
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: Animal,
    dueño: Dueño,
}
impl Mascota {
    pub fn igual(&self, otra: &Mascota) -> bool {
        self.nombre == otra.nombre
            && self.edad == otra.edad
            && self.tipo.igual(&otra.tipo)
            && self.dueño.nombre == otra.dueño.nombre
            && self.dueño.direccion == otra.dueño.direccion
            && self.dueño.telefono == otra.dueño.telefono
    }
    pub fn encontrar(&self, nombre: &str, nombre_dueño: &str, telefono: &str) -> bool {
        self.nombre == nombre
            && self.dueño.nombre == nombre_dueño
            && self.dueño.telefono == telefono
    }
}
struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    cola: VecDeque<Mascota>,
    atenciones: Vec<Atencion>,
}
impl Veterinaria {
    pub fn new(
        nombre: String,
        direccion: String,
        id: u32,
        cola: VecDeque<Mascota>,
        atenciones: Vec<Atencion>,
    ) -> Self {
        Veterinaria {
            nombre,
            direccion,
            id,
            cola,
            atenciones,
        }
    }
    pub fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola.push_back(mascota);
    }
    pub fn agregar_mascota_prioritaria(&mut self, mascota: Mascota) {
        self.cola.push_front(mascota);
    }
    pub fn atender_mascota(&mut self) -> Option<Mascota> {
        self.cola.pop_front()
    }
    pub fn eliminar_mascota_especifica(&mut self, mascota: Mascota) {
        if !self.cola.is_empty() {
            if let Some(index) = self.cola.iter().position(|m| m.igual(&mascota)) {
                self.cola.remove(index);
            }
        }
    }
    pub fn registrar_atencion(&mut self, atencion: Atencion) {
        self.atenciones.push(atencion);
    }
    pub fn buscar_atencion(
        &self,
        nombre_mascota: String,
        nombre_dueño: String,
        telefono_dueño: String,
    ) -> Option<&Atencion> {
        self.atenciones.iter().find(|a| {
            a.mascota
                .encontrar(&nombre_mascota, &nombre_dueño, &telefono_dueño)
        })
    }
    pub fn modificar_diagnostico(&mut self, mascota: Mascota, nuevo_diagnostico: String) {
        if let Some(index) = self
            .atenciones
            .iter()
            .position(|a| a.mascota.igual(&mascota))
        {
            self.atenciones[index].diagnóstico = nuevo_diagnostico;
        }
    }

    pub fn modificar_fecha_proxima_visita(&mut self, mascota: Mascota, nueva_fecha: Option<Fecha>) {
        if let Some(index) = self
            .atenciones
            .iter()
            .position(|a| a.mascota.igual(&mascota))
        {
            self.atenciones[index].fecha_proxima_visita = nueva_fecha;
        }
    }
}

struct Atencion {
    mascota: Mascota,
    diagnóstico: String,
    tratamiento: String,
    fecha_proxima_visita: Option<Fecha>,
}

impl Atencion {
    pub fn new(
        mascota: Mascota,
        diagnóstico: String,
        tratamiento: String,
        fecha_proxima_visita: Option<Fecha>,
    ) -> Self {
        Atencion {
            mascota,
            diagnóstico,
            tratamiento,
            fecha_proxima_visita,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_mascota() -> Mascota {
        Mascota {
            nombre: "Firulais".to_string(),
            edad: 5,
            tipo: Animal::Perro,
            dueño: Dueño {
                nombre: "Carlos".to_string(),
                direccion: "Calle Falsa 123".to_string(),
                telefono: "123456789".to_string(),
            },
        }
    }

    fn crear_fecha() -> Fecha {
        Fecha::new(14, 5, 2025).unwrap()
    }

    #[test]
    fn test_agregar() {
        let mut vet = Veterinaria::new(
            "Mi Vet".to_string(),
            "Av. Siempre Viva".to_string(),
            1,
            VecDeque::new(),
            vec![],
        );
        let mascota = crear_mascota();

        vet.agregar_mascota(mascota.clone());
        assert_eq!(vet.cola.len(), 1);

        vet.eliminar_mascota_especifica(mascota);
        assert_eq!(vet.cola.len(), 0);
    }
    #[test]
    fn test_atender_mascota() {
        let mut vet = Veterinaria::new(
            "Vet".to_string(),
            "Calle".to_string(),
            1,
            VecDeque::new(),
            vec![],
        );
        let mascota = crear_mascota();

        vet.agregar_mascota(mascota.clone());
        let atendida = vet.atender_mascota();
        assert!(atendida.is_some());
        assert!(vet.cola.is_empty());
    }

    #[test]
    fn test_registrar_y_buscar_atencion() {
        let mut vet = Veterinaria::new(
            "Vet".to_string(),
            "Calle".to_string(),
            1,
            VecDeque::new(),
            vec![],
        );
        let mascota = crear_mascota();
        let fecha = Some(crear_fecha());

        let atencion = Atencion::new(
            mascota.clone(),
            "Resfriado".to_string(),
            "Reposo".to_string(),
            fecha.clone(),
        );

        vet.registrar_atencion(atencion);

        let resultado = vet.buscar_atencion(
            mascota.nombre.clone(),
            mascota.dueño.nombre.clone(),
            mascota.dueño.telefono.clone(),
        );

        assert!(resultado.is_some());
        assert_eq!(resultado.unwrap().diagnóstico, "Resfriado");
    }

    #[test]
    fn test_modificar_diagnostico() {
        let mut vet = Veterinaria::new(
            "Vet".to_string(),
            "Calle".to_string(),
            1,
            VecDeque::new(),
            vec![],
        );
        let mascota = crear_mascota();
        let fecha = Some(crear_fecha());

        vet.registrar_atencion(Atencion::new(
            mascota.clone(),
            "Fiebre".to_string(),
            "Reposo".to_string(),
            fecha,
        ));

        vet.modificar_diagnostico(mascota.clone(), "Gripe canina".to_string());

        let atencion = vet.buscar_atencion(
            mascota.nombre.clone(),
            mascota.dueño.nombre.clone(),
            mascota.dueño.telefono.clone(),
        );

        assert_eq!(atencion.unwrap().diagnóstico, "Gripe canina");
    }

    #[test]
    fn test_modificar_fecha_proxima_visita() {
        let mut vet = Veterinaria::new(
            "Vet".to_string(),
            "Calle".to_string(),
            1,
            VecDeque::new(),
            vec![],
        );
        let mascota = crear_mascota();

        vet.registrar_atencion(Atencion::new(
            mascota.clone(),
            "Chequeo".to_string(),
            "Ninguno".to_string(),
            None,
        ));

        let nueva_fecha = Some(Fecha::new(20, 6, 2025).unwrap());
        vet.modificar_fecha_proxima_visita(mascota.clone(), nueva_fecha.clone());

        let atencion = vet.buscar_atencion(
            mascota.nombre.clone(),
            mascota.dueño.nombre.clone(),
            mascota.dueño.telefono.clone(),
        );

        let fecha_obtenida = atencion.unwrap().fecha_proxima_visita.as_ref().unwrap();
        let fecha_esperada = nueva_fecha.as_ref().unwrap();
        assert!(fecha_obtenida.igual(fecha_esperada));
    }
}
