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
#[derive(Debug, PartialEq, Clone)]
pub enum Animal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

struct Dueño {
    nombre: String,
    direccion: String,
    telefono: String,
}
/*mascota nombre, la edad, el tipo de animal(perro, gato, caballo, otros) y su dueño. */
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: Animal,
    dueño: Dueño,
}
/*veterinaria se conoce el nombre, la dirección y un id */
struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    cola: Vec<Mascota>,
    atenciones: Vec<Atencion>,
}
struct Atencion {
    mascota: Mascota,
    diagnóstico: String,
    tratamiento: String,
    fecha_proxima_visita: Option<Fecha>,
}
