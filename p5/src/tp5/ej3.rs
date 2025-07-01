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

3- En base al ejercicio 9 del tp#3 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b - Ahora el registro de atenciones debe persistir en un archivo en formato JSON, es
decir todas la operaciones que lectura, agregar y modificación de atenciones se realizan
sobre un archivo.No debe modificar los tests hechos en el punto a. Si puede agregar más
en caso de que haga métodos nuevos para cumplir con este punto. Recuerde también que
se debe seguir manteniendo un coverage de al menos 90%,

*/
use super::Fecha;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Clone, Serialize, Deserialize)]

pub enum Animal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ResultadoOperacion {
    /// La operación se realizó con éxito.
    Exito,
    /// La operación falló por una razón lógica
    Fallo(String),
    /// Ocurrió un error relacionado con archivos (por ejemplo, al guardar/cargar).
    ErrorArchivo(String),
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
#[derive(Clone, Serialize, Deserialize)]

struct Dueño {
    nombre: String,
    direccion: String,
    telefono: String,
}
#[derive(Clone, Serialize, Deserialize)]

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
    // fn de JSON
    pub fn guardar_atenciones_en_archivo(&self, path: &str) -> Result<(), ResultadoOperacion> {
        let atenciones: Vec<Atencion> = self
            .atenciones
            .iter()
            .map(|a| Atencion {
                mascota: a.mascota.clone(),
                diagnóstico: a.diagnóstico.clone(),
                tratamiento: a.tratamiento.clone(),
                fecha_proxima_visita: a.fecha_proxima_visita.clone(),
            })
            .collect();
        let file =
            File::create(path).map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?;
        serde_json::to_writer_pretty(file, &atenciones)
            .map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?;
        Ok(())
    }
    pub fn cargar_atenciones_de_archivo(&mut self, path: &str) -> Result<(), ResultadoOperacion> {
        let mut file =
            File::open(path).map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?;
        let mut contenido = String::new();
        file.read_to_string(&mut contenido)
            .map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?;
        let atenciones: Vec<Atencion> = serde_json::from_str(&contenido)
            .map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?;
        self.atenciones = atenciones
            .into_iter()
            .map(|a| Atencion {
                mascota: a.mascota,
                diagnóstico: a.diagnóstico,
                tratamiento: a.tratamiento,
                fecha_proxima_visita: a.fecha_proxima_visita,
            })
            .collect();
        Ok(())
    }
    // Registrar una atencion y guardar en archivo
    pub fn registrar_atencion_y_guardar(
        &mut self,
        atencion: Atencion,
        path: &str,
    ) -> Result<(), ResultadoOperacion> {
        self.cargar_atenciones_de_archivo(path).ok(); // Ignora error si el archivo no existe
        self.atenciones.push(atencion);
        self.guardar_atenciones_en_archivo(path) //delegamos la persistencia al método ya implementado
    }
    // Modificar diagnostico y guardar en archivo
    pub fn modificar_diagnostico_y_guardar(
        &mut self,
        mascota: Mascota,
        nuevo_diagnostico: String,
        path: &str,
    ) -> Result<(), ResultadoOperacion> {
        self.cargar_atenciones_de_archivo(path).ok();
        if let Some(index) = self
            .atenciones
            .iter()
            .position(|a| a.mascota.igual(&mascota))
        {
            self.atenciones[index].diagnóstico = nuevo_diagnostico;
            self.guardar_atenciones_en_archivo(path)
        } else {
            Err(ResultadoOperacion::Fallo(
                "Mascota no encontrada".to_string(),
            ))
        }
    }
    // Modificar fecha próxima visita y guardar en archivo
    pub fn modificar_fecha_proxima_visita_y_guardar(
        &mut self,
        mascota: Mascota,
        nueva_fecha: Option<Fecha>,
        path: &str,
    ) -> Result<(), ResultadoOperacion> {
        self.cargar_atenciones_de_archivo(path).ok();
        if let Some(index) = self
            .atenciones
            .iter()
            .position(|a| a.mascota.igual(&mascota))
        {
            self.atenciones[index].fecha_proxima_visita = nueva_fecha;
            self.guardar_atenciones_en_archivo(path)
        } else {
            Err(ResultadoOperacion::Fallo(
                "Mascota no encontrada".to_string(),
            ))
        }
    }
    // Eliminar atención y guardar en archivo
    pub fn eliminar_atencion_y_guardar(
        &mut self,
        mascota: Mascota,
        path: &str,
    ) -> Result<(), ResultadoOperacion> {
        self.cargar_atenciones_de_archivo(path).ok();
        if let Some(index) = self
            .atenciones
            .iter()
            .position(|a| a.mascota.igual(&mascota))
        {
            self.atenciones.remove(index);
            self.guardar_atenciones_en_archivo(path)
        } else {
            Err(ResultadoOperacion::Fallo(
                "Atención no encontrada".to_string(),
            ))
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
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
    fn crear_atencion() -> Atencion {
        Atencion::new(
            crear_mascota(),
            "Resfriado".to_string(),
            "Reposo".to_string(),
            None,
        )
    }

    fn crear_vet() -> Veterinaria {
        Veterinaria::new(
            "Mi Vet".to_string(),
            "Av. Siempre Viva".to_string(),
            1,
            VecDeque::new(),
            vec![],
        )
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

    //test JSON
    #[test]
    fn test_guardar_atenciones_en_archivo_exito() {
        use std::fs;
        let atencion = crear_atencion();
        let vet = Veterinaria::new(
            "Mi Vet".to_string(),
            "Av. Siempre Viva".to_string(),
            1,
            VecDeque::new(),
            vec![atencion.clone()],
        );
        let path = "test_atenciones.json";
        let res = vet.guardar_atenciones_en_archivo(path);
        assert!(res.is_ok());
        let contenido = fs::read_to_string(path).unwrap();
        assert!(contenido.contains("Firulais"));
        let _ = fs::remove_file(path);
    }
    #[test]
    fn test_guardar_atenciones_en_archivo_error() {
        let vet = Veterinaria::new(
            "Mi Vet".to_string(),
            "Av. Siempre Viva".to_string(),
            1,
            VecDeque::new(),
            vec![],
        );
        // Usamos un path inválido para forzar el error
        let res = vet.guardar_atenciones_en_archivo("/no_existe/test_atenciones.json");
        match res {
            Err(ResultadoOperacion::ErrorArchivo(_)) => assert!(true),
            _ => panic!("Se esperaba un error de archivo"),
        }
    }
    #[test]
    fn test_cargar_atenciones_de_archivo_exito() {
        use std::fs;
        let atencion = crear_atencion();
        let path = "test_atenciones_cargar.json";
        // Guardar primero el archivo con una atención
        {
            let vet = Veterinaria::new(
                "Mi Vet".to_string(),
                "Av. Siempre Viva".to_string(),
                1,
                VecDeque::new(),
                vec![atencion.clone()],
            );
            vet.guardar_atenciones_en_archivo(path).unwrap();
        }
        // Ahora cargarlo en una veterinaria vacía
        let mut vet2 = Veterinaria::new(
            "Mi Vet".to_string(),
            "Av. Siempre Viva".to_string(),
            1,
            VecDeque::new(),
            vec![],
        );
        let res = vet2.cargar_atenciones_de_archivo(path);
        assert!(res.is_ok());
        assert_eq!(vet2.atenciones.len(), 1);
        assert_eq!(vet2.atenciones[0].mascota.nombre, "Firulais");
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_cargar_atenciones_de_archivo_error() {
        let mut vet = Veterinaria::new(
            "Mi Vet".to_string(),
            "Av. Siempre Viva".to_string(),
            1,
            VecDeque::new(),
            vec![],
        );
        // Usamos un path inválido para forzar el error
        let res = vet.cargar_atenciones_de_archivo("/no_existe/test_atenciones.json");
        match res {
            Err(ResultadoOperacion::ErrorArchivo(_)) => assert!(true),
            _ => panic!("Se esperaba un error de archivo"),
        }
    }

    #[test]
    fn test_registrar_atencion_y_guardar_exito() {
        use std::fs;
        let path = "test_registrar_atencion.json";
        let mut vet = Veterinaria::new(
            "Mi Vet".to_string(),
            "Av. Siempre Viva".to_string(),
            1,
            VecDeque::new(),
            vec![],
        );
        let atencion = crear_atencion();
        // Debe registrar y guardar correctamente
        let res = vet.registrar_atencion_y_guardar(atencion.clone(), path);
        assert!(res.is_ok());
        // El archivo debe existir y contener la atención
        let contenido = fs::read_to_string(path).unwrap();
        assert!(contenido.contains("Firulais"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_registrar_atencion_y_guardar_archivo_no_existe() {
        use std::fs;
        let path = "test_registrar_atencion_no_existe.json";
        // Nos aseguramos de que el archivo no exista
        let _ = fs::remove_file(path);
        let mut vet = Veterinaria::new(
            "Mi Vet".to_string(),
            "Av. Siempre Viva".to_string(),
            1,
            VecDeque::new(),
            vec![],
        );
        let atencion = crear_atencion();
        // Debe crear el archivo y guardar la atención
        let res = vet.registrar_atencion_y_guardar(atencion.clone(), path);
        assert!(res.is_ok());
        let contenido = fs::read_to_string(path).unwrap();
        assert!(contenido.contains("Firulais"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_registrar_atencion_y_guardar_error_archivo() {
        let mut vet = Veterinaria::new(
            "Mi Vet".to_string(),
            "Av. Siempre Viva".to_string(),
            1,
            VecDeque::new(),
            vec![],
        );
        let atencion = crear_atencion();
        // Usamos un path inválido para forzar el error
        let res =
            vet.registrar_atencion_y_guardar(atencion, "/no_existe/test_registrar_atencion.json");
        match res {
            Err(ResultadoOperacion::ErrorArchivo(_)) => assert!(true),
            _ => panic!("Se esperaba un error de archivo"),
        }
    }
    #[test]
    fn test_modificar_diagnostico_y_guardar_exito() {
        use std::fs;
        let path = "test_modificar_diagnostico.json";
        let mascota = crear_mascota();
        let atencion = Atencion::new(
            mascota.clone(),
            "Fiebre".to_string(),
            "Reposo".to_string(),
            None,
        );
        // Guardamos una atención con diagnóstico "Fiebre"
        {
            let vet = Veterinaria::new(
                "Vet".to_string(),
                "Calle".to_string(),
                1,
                VecDeque::new(),
                vec![atencion],
            );
            vet.guardar_atenciones_en_archivo(path).unwrap();
        }
        // Modificamos el diagnóstico
        let mut vet2 = Veterinaria::new(
            "Vet".to_string(),
            "Calle".to_string(),
            1,
            VecDeque::new(),
            vec![],
        );
        let res = vet2.modificar_diagnostico_y_guardar(mascota.clone(), "Gripe".to_string(), path);
        assert!(res.is_ok());
        // Verificamos que el archivo contiene el nuevo diagnóstico
        let contenido = fs::read_to_string(path).unwrap();
        assert!(contenido.contains("Gripe"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_modificar_diagnostico_y_guardar_mascota_no_encontrada() {
        use std::fs;
        let path = "test_modificar_diagnostico_no_encontrada.json";
        // Guardamos una atención con otra mascota
        {
            let otra_mascota = Mascota {
                nombre: "Otro".to_string(),
                edad: 2,
                tipo: Animal::Gato,
                dueño: Dueño {
                    nombre: "Juan".to_string(),
                    direccion: "Calle 2".to_string(),
                    telefono: "987654321".to_string(),
                },
            };
            let atencion =
                Atencion::new(otra_mascota, "Sano".to_string(), "Nada".to_string(), None);
            let vet = Veterinaria::new(
                "Vet".to_string(),
                "Calle".to_string(),
                1,
                VecDeque::new(),
                vec![atencion],
            );
            vet.guardar_atenciones_en_archivo(path).unwrap();
        }
        // Intentamos modificar el diagnóstico de una mascota que no existe en el archivo
        let mut vet2 = Veterinaria::new(
            "Vet".to_string(),
            "Calle".to_string(),
            1,
            VecDeque::new(),
            vec![],
        );
        let mascota = crear_mascota();
        let res = vet2.modificar_diagnostico_y_guardar(mascota, "Gripe".to_string(), path);
        match res {
            Err(ResultadoOperacion::Fallo(msg)) => assert!(msg.contains("Mascota no encontrada")),
            _ => panic!("Se esperaba error Mascota no encontrada"),
        }
        let _ = fs::remove_file(path);
    }
    #[test]
    fn test_modificar_fecha_proxima_visita_y_guardar_exito() {
        use std::fs;
        let path = "test_modificar_fecha_visita.json";
        let mascota = crear_mascota();
        let atencion = Atencion::new(
            mascota.clone(),
            "Chequeo".to_string(),
            "Ninguno".to_string(),
            None,
        );
        // Guardamos una atención sin fecha de próxima visita
        let mut vet = crear_vet();
        vet.registrar_atencion(atencion);
        vet.guardar_atenciones_en_archivo(path).unwrap();
        // Modificamos la fecha de próxima visita
        let nueva_fecha = crear_fecha();
        let res = vet.modificar_fecha_proxima_visita_y_guardar(
            mascota.clone(),
            Some(nueva_fecha.clone()),
            path,
        );
        assert!(res.is_ok());
        // Verificamos que el archivo contiene la nueva fecha
        let contenido = fs::read_to_string(path).unwrap();
        let atenciones: Vec<Atencion> = serde_json::from_str(&contenido).unwrap();
        assert!(atenciones[0].fecha_proxima_visita.is_some());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_modificar_fecha_proxima_visita_y_guardar_mascota_no_encontrada() {
        use std::fs;
        let path = "test_modificar_fecha_visita_no_encontrada.json";
        // Guardamos una atención con otra mascota

        let otra_mascota = Mascota {
            nombre: "Otro".to_string(),
            edad: 2,
            tipo: Animal::Gato,
            dueño: Dueño {
                nombre: "Juan".to_string(),
                direccion: "Calle 2".to_string(),
                telefono: "987654321".to_string(),
            },
        };
        let atencion = Atencion::new(otra_mascota, "Sano".to_string(), "Nada".to_string(), None);
        let mut vet = crear_vet();
        vet.registrar_atencion(atencion);

        vet.guardar_atenciones_en_archivo(path).unwrap();
        // Intentamos modificar la fecha de una mascota que no existe en el archivo
        let mascota = crear_mascota();
        let nueva_fecha = crear_fecha();
        let res =
            vet.modificar_fecha_proxima_visita_y_guardar(mascota, Some(nueva_fecha.clone()), path);
        match res {
            Err(ResultadoOperacion::Fallo(msg)) => assert!(msg.contains("Mascota no encontrada")),
            _ => panic!("Se esperaba error Mascota no encontrada"),
        }
        let _ = fs::remove_file(path);
    }
    #[test]
    fn test_eliminar_atencion_y_guardar_exito() {
        use std::fs;
        let path = "test_eliminar_atencion.json";
        let mascota = crear_mascota();
        let atencion = crear_atencion();
        // Guardamos una atención
        {
            let mut vet = crear_vet();
            vet.registrar_atencion(atencion);
            vet.guardar_atenciones_en_archivo(path).unwrap();
        }
        // Eliminamos la atención y guardamos
        let mut vet2 = crear_vet();
        let res = vet2.eliminar_atencion_y_guardar(mascota.clone(), path);
        assert!(res.is_ok());
        // Verificamos que el archivo quedó vacío
        let contenido = fs::read_to_string(path).unwrap();
        let atenciones: Vec<Atencion> = serde_json::from_str(&contenido).unwrap();
        assert!(atenciones.is_empty());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_eliminar_atencion_y_guardar_no_encontrada() {
        use std::fs;
        let path = "test_eliminar_atencion_no_encontrada.json";
        // Guardamos una atención con otra mascota
        let otra_mascota = Mascota {
            nombre: "Otro".to_string(),
            edad: 2,
            tipo: Animal::Gato,
            dueño: Dueño {
                nombre: "Juan".to_string(),
                direccion: "Calle 2".to_string(),
                telefono: "987654321".to_string(),
            },
        };
        let atencion = Atencion::new(otra_mascota, "Sano".to_string(), "Nada".to_string(), None);
        let mut vet = crear_vet();
        vet.registrar_atencion(atencion);
        vet.guardar_atenciones_en_archivo(path).unwrap();

        // Intentamos eliminar una atención de una mascota que no existe en el archivo
        let mascota = crear_mascota();
        let res = vet.eliminar_atencion_y_guardar(mascota, path);
        match res {
            Err(ResultadoOperacion::Fallo(msg)) => assert!(msg.contains("Atención no encontrada")),
            _ => panic!("Se esperaba error Atención no encontrada"),
        }
        let _ = fs::remove_file(path);
    }
}
