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

struct Dueño {
    nombre: String,
    direccion: String,
    telefono: String,
}
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
