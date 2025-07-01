/*
10-Para una biblioteca se desea implementar un sistema de préstamos de libros. De la
biblioteca se conoce el nombre y la dirección, las copias de los libros a disposición para
prestar y los préstamos efectuados. Los libros a disposición es un registro donde se indica
la cantidad de ejemplares que tiene a disposición para prestar de determinado libro. De
cada libro se conoce el isbn, el título, autor, número de páginas, género(novela, infantil,
técnico, otros).
 Para registrar un préstamo se requiere el libro, el cliente, la fecha de
vencimiento del préstamo, la fecha de devolución y el estado que puede ser devuelto o en
préstamo. Del cliente se conoce el nombre, teléfono y dirección de correo electrónico.
Implemente los métodos necesarios para realizar las siguientes acciones:

➔ obtener cantidad de copias: dado un determinado libro retorna la cantidad de
copias a disposición que hay para prestar de dicho libro.
➔ decrementar cantidad de copias a disposición; dado un libro decrementa en 1
la cantidad de copias de libros a disposición para prestar.
➔ incrementar cantidad de copias a disposición: dado un libro incrementa en 1
la cantidad de copias del libro a disposición para ser prestado.
➔ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado
“en préstamo” de un determinado cliente.
➔ realizar un préstamo de un libro para un cliente: crea un préstamo de un libro
para un determinado cliente cumpliendo con lo siguiente
◆ el cliente no tenga más de 5 préstamos en el estado “en préstamo”
◆ haya al menos una copia disponible en el registro de copias a
disposición.
De ser así descuenta 1 en el registro de “copias a disposición” y
retorna true, si no cumple con alguna de las condiciones retorna false.
➔ ver préstamos a vencer el los próximos días: retorna una lista de préstamos a
vencer el los próximos días, el valor de días es pasado por parámetro.
➔ ver los préstamos vencidos: retorna una lista de préstamos en el estado “en
préstamos” donde la fecha de vencimiento es menor a la fecha actual.
➔ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si
existe.
➔ devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al
estado “devuelto”, se registra la fecha de devolución y se incrementa la
cantidad de libros en 1 del libro devuelto en el registro de copias a
disposición.
Nota: para la fecha utilice lo implementado en el punto 3.

4- En base al ejercicio 10 del tp#3 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Tanto los libros con sus copias como la administración de préstamos se realizan
sobre archivos en formato JSON. Realice las modificaciones pertinentes para poder hacerlo
así. No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que
haga métodos nuevos para cumplir con este punto . Recuerde también que se debe seguir
manteniendo un coverage de al menos 90%.
*/
use super::Fecha;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write};

//Enums
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Estado {
    EnPrestamo,
    Devuelto,
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
//Structs
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: Estado,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Libro {
    isbn: String,
    titulo: String,
    autor: String,
    num_paginas: u32,
    genero: Genero,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Cliente {
    nombre: String,
    telefono: String,
    email: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]

struct Biblioteca {
    nombre: String,
    direccion: String,
    libros_a_disposicion: Vec<Libros>,
    prestamos: Vec<Prestamo>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Libros {
    tupla_libro: (Libro, u32),
}

impl Estado {
    pub fn devuelto(&self) -> bool {
        match self {
            Estado::Devuelto => true,
            _ => false,
        }
    }
    pub fn en_prestamo(&self) -> bool {
        match self {
            Estado::EnPrestamo => true,
            _ => false,
        }
    }
}

impl Cliente {
    pub fn new(nombre: String, telefono: String, email: String) -> Self {
        Cliente {
            nombre,
            telefono,
            email,
        }
    }

    pub fn igual(&self, otro: &Cliente) -> bool {
        self.nombre == otro.nombre && self.telefono == otro.telefono && self.email == otro.email
    }
}

impl Prestamo {
    pub fn new(
        libro: Libro,
        cliente: Cliente,
        fecha_vencimiento: Fecha,
        fecha_devolucion: Option<Fecha>,
        estado: Estado,
    ) -> Self {
        Prestamo {
            libro,
            cliente,
            fecha_vencimiento,
            fecha_devolucion,
            estado,
        }
    }
    pub fn contar_prestado_cliente(&self, cliente: &Cliente) -> u32 {
        if self.cliente.igual(cliente) && self.estado.en_prestamo() {
            1
        } else {
            0
        }
    }
}

impl Biblioteca {
    pub fn new(
        nombre: String,
        direccion: String,
        libros_a_disposicion: Vec<Libros>,
        prestamos: Vec<Prestamo>,
    ) -> Self {
        Biblioteca {
            nombre,
            direccion,
            libros_a_disposicion,
            prestamos,
        }
    }
    /// Agrega un libro a disposición. Si ya existe, suma la cantidad.
    pub fn agregar_libro_a_disposicion(&mut self, libro: Libro, cantidad: u32) {
        if let Some(l) = self
            .libros_a_disposicion
            .iter_mut()
            .find(|l| l.tupla_libro.0.igual_isbn(&libro))
        {
            l.tupla_libro.1 += cantidad;
        } else {
            self.libros_a_disposicion.push(Libros::new(libro, cantidad));
        }
    }

    pub fn ver_prestamo_a_vencer(&self, fecha_hoy: &Fecha, dias: u32) -> Vec<Prestamo> {
        let mut a_vencer = Vec::new();
        if let Some(fecha_limite) = fecha_hoy.sumar_dias(dias) {
            for prestamo in &self.prestamos {
                let fecha_vencimiento = &prestamo.fecha_vencimiento;
                if fecha_limite.es_mayor(fecha_vencimiento) && prestamo.estado.en_prestamo() {
                    a_vencer.push(prestamo.clone());
                }
            }
        }
        a_vencer
    }
    pub fn ver_prestamos_vencidos(&self, fecha_hoy: &Fecha) -> Vec<Prestamo> {
        let mut vencidos = Vec::new();
        for prestamo in &self.prestamos {
            if fecha_hoy.es_mayor(&prestamo.fecha_vencimiento) && prestamo.estado.en_prestamo() {
                vencidos.push(prestamo.clone());
            }
        }
        vencidos
    }

    pub fn realizar_prestamo(
        &mut self,
        libro: &Libro,
        cliente: &Cliente,
        fecha_vencimiento: Fecha,
    ) -> bool {
        if self.contar_prestamos_cliente(&cliente) < 5 && self.obtener_cant_copias(&libro) > 0 {
            let prestamo = Prestamo::new(
                libro.clone(),
                cliente.clone(),
                fecha_vencimiento,
                None,
                Estado::EnPrestamo,
            );
            self.prestamos.push(prestamo);
            self.decrementar_copia_libro(libro);
            true
        } else {
            false
        }
    }

    pub fn buscar_prestamo(&mut self, libro: &Libro, cliente: &Cliente) -> Option<&mut Prestamo> {
        self.prestamos.iter_mut().find(|p| {
            p.libro.igual_isbn(libro) && p.cliente.igual(cliente) && p.estado.en_prestamo()
        })
    }

    pub fn devolver_libro(
        &mut self,
        libro: &Libro,
        cliente: &Cliente,
        fecha_devolucion: Fecha,
    ) -> bool {
        if let Some(prestamo) = self.buscar_prestamo(libro, cliente) {
            prestamo.estado = Estado::Devuelto;
            prestamo.fecha_devolucion = Some(fecha_devolucion);
            self.incrementar_copia_libro(libro);
            true
        } else {
            false
        }
    }

    pub fn contar_prestamos_cliente(&self, cliente: &Cliente) -> u32 {
        let mut contador = 0;
        for prestamo in &self.prestamos {
            if prestamo.cliente.igual(cliente) && prestamo.estado.en_prestamo() {
                contador += 1;
            }
        }
        contador
    }
    pub fn incrementar_copia_libro(&mut self, libro: &Libro) {
        if let Some(index) = self
            .libros_a_disposicion
            .iter()
            .position(|l| l.tupla_libro.0.igual_isbn(&libro))
        {
            self.libros_a_disposicion[index].incrementar();
        }
    }

    pub fn decrementar_copia_libro(&mut self, libro: &Libro) {
        if let Some(index) = self
            .libros_a_disposicion
            .iter()
            .position(|l| l.tupla_libro.0.igual_isbn(&libro))
        {
            self.libros_a_disposicion[index].decrementar();
        }
    }

    pub fn obtener_cant_copias(&self, libro: &Libro) -> u32 {
        if let Some(index) = self
            .libros_a_disposicion
            .iter()
            .position(|l| l.tupla_libro.0.igual_isbn(&libro))
        {
            self.libros_a_disposicion[index].tupla_libro.1
        } else {
            0
        }
    }
    //JSON
    pub fn guardar_en_archivo(&self, path: &str) -> Result<(), ResultadoOperacion> {
        let file =
            File::create(path).map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?;
        serde_json::to_writer_pretty(file, &self)
            .map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))
    }
    /// Carga el estado completo de la biblioteca (libros y préstamos) desde un archivo JSON.
    pub fn cargar_de_archivo(&mut self, path: &str) -> Result<(), ResultadoOperacion> {
        let mut file =
            File::open(path).map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?;
        let mut contenido = String::new();
        file.read_to_string(&mut contenido)
            .map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?;
        let biblioteca: Biblioteca = serde_json::from_str(&contenido)
            .map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?;
        *self = biblioteca;
        Ok(())
    }
    pub fn realizar_prestamo_y_guardar(
        &mut self,
        libro: &Libro,
        cliente: &Cliente,
        fecha_vencimiento: Fecha,
        path: &str,
    ) -> Result<ResultadoOperacion, ResultadoOperacion> {
        self.cargar_de_archivo(path).ok(); // Ignora error si el archivo no existe
        let exito = self.realizar_prestamo(libro, cliente, fecha_vencimiento);
        self.guardar_en_archivo(path)?;
        if exito {
            Ok(ResultadoOperacion::Exito)
        } else {
            Err(ResultadoOperacion::Fallo(
                "No se pudo realizar el préstamo".to_string(),
            ))
        }
    }
    /// Ejemplo: devolver libro y guardar automáticamente
    pub fn devolver_libro_y_guardar(
        &mut self,
        libro: &Libro,
        cliente: &Cliente,
        fecha_devolucion: Fecha,
        path: &str,
    ) -> Result<ResultadoOperacion, ResultadoOperacion> {
        self.cargar_de_archivo(path).ok();
        let exito = self.devolver_libro(libro, cliente, fecha_devolucion);
        self.guardar_en_archivo(path)?;
        if exito {
            Ok(ResultadoOperacion::Exito)
        } else {
            Err(ResultadoOperacion::Fallo(
                "No se pudo devolver el libro".to_string(),
            ))
        }
    }
}

impl Libro {
    pub fn new(
        isbn: String,
        titulo: String,
        autor: String,
        num_paginas: u32,
        genero: Genero,
    ) -> Self {
        Libro {
            isbn,
            titulo,
            autor,
            num_paginas,
            genero,
        }
    }

    pub fn igual_isbn(&self, otro: &Libro) -> bool {
        self.isbn == otro.isbn
    }
}

impl Libros {
    pub fn new(libro: Libro, cantidad: u32) -> Self {
        Libros {
            tupla_libro: (libro, cantidad),
        }
    }
    pub fn incrementar(&mut self) {
        self.tupla_libro.1 += 1;
    }
    pub fn decrementar(&mut self) {
        if self.tupla_libro.1 > 0 {
            self.tupla_libro.1 -= 1;
        }
    }
}
//Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn test_fecha() -> Fecha {
        Fecha::new(20, 5, 2025).expect("Fecha inválida")
    }
    fn test_libro() -> Libro {
        Libro::new(
            "123".to_string(),
            "Rust Book".to_string(),
            "Ferris".to_string(),
            300,
            Genero::Tecnico,
        )
    }
    fn test_cliente() -> Cliente {
        Cliente::new(
            "José".to_string(),
            "123456789".to_string(),
            "jose@correo.com".to_string(),
        )
    }
    fn test_biblioteca() -> Biblioteca {
        Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            vec![],
            vec![],
        )
    }

    #[test]
    fn test_obtener_cantidad_de_copias() {
        let libro = Libro::new(
            "123".to_string(),
            "Rust Book".to_string(),
            "Ferris".to_string(),
            300,
            Genero::Tecnico,
        );

        let libros = vec![Libros::new(libro.clone(), 3)];
        let biblioteca = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            libros,
            vec![],
        );

        let cantidad = biblioteca.obtener_cant_copias(&libro);
        assert_eq!(cantidad, 3);
    }

    #[test]
    fn test_decrementar_cantidad_de_copias() {
        let libro = Libro::new(
            "123".to_string(),
            "Rust Book".to_string(),
            "Ferris".to_string(),
            300,
            Genero::Tecnico,
        );
        let libros = vec![Libros::new(libro.clone(), 3)];
        let mut biblioteca = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            libros,
            vec![],
        );

        biblioteca.decrementar_copia_libro(&libro);
        let cantidad = biblioteca.obtener_cant_copias(&libro);
        assert_eq!(cantidad, 2);
    }

    #[test]
    fn test_incrementar_cantidad_de_copias() {
        let libro = Libro::new(
            "123".to_string(),
            "Rust Book".to_string(),
            "Ferris".to_string(),
            300,
            Genero::Tecnico,
        );
        let libros = vec![Libros::new(libro.clone(), 3)];
        let mut biblioteca = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            libros,
            vec![],
        );

        biblioteca.incrementar_copia_libro(&libro);
        let cantidad = biblioteca.obtener_cant_copias(&libro);
        assert_eq!(cantidad, 4);
    }

    #[test]
    fn test_contar_prestamos_cliente() {
        let libro = Libro::new(
            "123".to_string(),
            "Rust Book".to_string(),
            "Ferris".to_string(),
            300,
            Genero::Tecnico,
        );
        let cliente = Cliente::new(
            "José".to_string(),
            "123456789".to_string(),
            "jose@mail.com".to_string(),
        );
        let fecha = Fecha::new(20, 5, 2025).expect("Fecha inválida");
        let libros = vec![Libros::new(libro.clone(), 3)];
        let mut biblioteca = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            libros,
            vec![],
        );
        biblioteca.realizar_prestamo(&libro, &cliente, fecha.clone());
        biblioteca.realizar_prestamo(&libro, &cliente, fecha.clone());
        assert_eq!(biblioteca.contar_prestamos_cliente(&cliente), 2);
    }

    #[test]
    fn test_realizar_prestamo_exitoso() {
        let libro = Libro::new(
            "123".to_string(),
            "Rust Book".to_string(),
            "Ferris".to_string(),
            300,
            Genero::Tecnico,
        );
        let cliente = Cliente::new(
            "José".to_string(),
            "123456789".to_string(),
            "jose@mail.com".to_string(),
        );
        let fecha = Fecha::new(20, 5, 2025).expect("Fecha inválida");
        let libros = vec![Libros::new(libro.clone(), 3)];
        let mut biblioteca = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            libros,
            vec![],
        );
        let exito = biblioteca.realizar_prestamo(&libro, &cliente, fecha.clone());
        assert!(exito);
        assert_eq!(biblioteca.obtener_cant_copias(&libro), 2);
    }

    #[test]
    fn test_realizar_prestamo_falla_por_limite() {
        let libro = Libro::new(
            "123".to_string(),
            "Rust Book".to_string(),
            "Ferris".to_string(),
            300,
            Genero::Tecnico,
        );
        let cliente = Cliente::new(
            "José".to_string(),
            "123456789".to_string(),
            "jose@mail.com".to_string(),
        );
        let fecha = Fecha::new(20, 5, 2025).expect("Fecha inválida");
        let libros = vec![Libros::new(libro.clone(), 3)];
        let mut biblioteca = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            libros,
            vec![],
        );
        for _ in 0..5 {
            biblioteca.realizar_prestamo(&libro, &cliente, fecha.clone());
        }
        let exito = biblioteca.realizar_prestamo(&libro, &cliente, fecha);
        assert!(!exito);
    }

    #[test]
    fn test_ver_prestamos_a_vencer() {
        let libro = Libro::new(
            "123".to_string(),
            "Rust Book".to_string(),
            "Ferris".to_string(),
            300,
            Genero::Tecnico,
        );
        let cliente = Cliente::new(
            "José".to_string(),
            "123456789".to_string(),
            "jose@mail.com".to_string(),
        );
        let fecha = Fecha::new(20, 5, 2025).expect("Fecha inválida");
        let libros = vec![Libros::new(libro.clone(), 3)];
        let mut biblioteca = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            libros,
            vec![],
        );
        let fecha_vencimiento = fecha.sumar_dias(3).unwrap();
        biblioteca.realizar_prestamo(&libro, &cliente, fecha_vencimiento.clone());

        let proximos = biblioteca.ver_prestamo_a_vencer(&fecha, 5);
        assert_eq!(proximos.len(), 1);
    }
    #[test]
    fn test_ver_prestamos_vencidos() {
        let libro = Libro::new(
            "123".to_string(),
            "Rust Book".to_string(),
            "Ferris".to_string(),
            300,
            Genero::Tecnico,
        );
        let cliente = Cliente::new(
            "José".to_string(),
            "123456789".to_string(),
            "jose@mail.com".to_string(),
        );
        let libros = vec![Libros::new(libro.clone(), 3)];
        let mut biblioteca = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            libros,
            vec![],
        );
        let fecha_prestamo = Fecha::new(1, 4, 2025).expect("fecha invalida");
        let hoy = Fecha::new(15, 5, 2025).expect("fecha invalida");

        biblioteca.realizar_prestamo(&libro, &cliente, fecha_prestamo);

        let vencidos = biblioteca.ver_prestamos_vencidos(&hoy);
        assert_eq!(vencidos.len(), 1);
    }

    #[test]
    fn test_devolver_libro() {
        let libro = Libro::new(
            "123".to_string(),
            "Rust Book".to_string(),
            "Ferris".to_string(),
            300,
            Genero::Tecnico,
        );
        let cliente = Cliente::new(
            "José".to_string(),
            "123456789".to_string(),
            "jose@mail.com".to_string(),
        );
        let fecha = Fecha::new(20, 5, 2025).expect("Fecha inválida");
        let libros = vec![Libros::new(libro.clone(), 3)];
        let mut biblioteca = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            libros,
            vec![],
        );
        biblioteca.realizar_prestamo(&libro, &cliente, fecha.clone());

        let devuelto = biblioteca.devolver_libro(&libro, &cliente, fecha.clone());
        assert!(devuelto);
        assert_eq!(biblioteca.obtener_cant_copias(&libro), 3);
    }
    #[test]
    fn test_estado_devuelto_true() {
        let estado = Estado::Devuelto;
        assert!(estado.devuelto());
    }

    #[test]
    fn test_estado_devuelto_false() {
        let estado = Estado::EnPrestamo;
        assert!(!estado.devuelto());
    }
    #[test]
    fn test_contar_prestado_cliente_positivo() {
        let cliente = Cliente::new(
            "Ana".to_string(),
            "123".to_string(),
            "ana@mail.com".to_string(),
        );
        let libro = Libro::new(
            "1".to_string(),
            "Libro".to_string(),
            "Autor".to_string(),
            100,
            Genero::Novela,
        );
        let prestamo = Prestamo::new(
            libro.clone(),
            cliente.clone(),
            Fecha::new(1, 1, 2025).unwrap(),
            None,
            Estado::EnPrestamo,
        );
        assert_eq!(prestamo.contar_prestado_cliente(&cliente), 1);
    }

    #[test]
    fn test_contar_prestado_cliente_negativo() {
        let cliente = Cliente::new(
            "Ana".to_string(),
            "123".to_string(),
            "ana@mail.com".to_string(),
        );
        let otro_cliente = Cliente::new(
            "Juan".to_string(),
            "456".to_string(),
            "juan@mail.com".to_string(),
        );
        let libro = Libro::new(
            "1".to_string(),
            "Libro".to_string(),
            "Autor".to_string(),
            100,
            Genero::Novela,
        );
        let prestamo = Prestamo::new(
            libro.clone(),
            cliente.clone(),
            Fecha::new(1, 1, 2025).unwrap(),
            None,
            Estado::Devuelto,
        );
        // No está en préstamo
        assert_eq!(prestamo.contar_prestado_cliente(&cliente), 0);
        // Cliente distinto
        assert_eq!(prestamo.contar_prestado_cliente(&otro_cliente), 0);
    }

    #[test]
    fn test_agregar_libro() {
        let libro = test_libro();
        let libros = vec![Libros::new(libro.clone(), 3)];
        let mut biblioteca = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            libros,
            vec![],
        );

        assert_eq!(biblioteca.libros_a_disposicion.len(), 1);
        assert_eq!(biblioteca.obtener_cant_copias(&libro), 3);
    }
    #[test]
    fn test_agregar_libro_a_disposicion_nuevo() {
        let libro = test_libro();
        let mut biblioteca = test_biblioteca();
        biblioteca.agregar_libro_a_disposicion(libro.clone(), 4);
        assert_eq!(biblioteca.libros_a_disposicion.len(), 1);
        assert_eq!(biblioteca.obtener_cant_copias(&libro), 4);
    }

    #[test]
    fn test_agregar_libro_a_disposicion_existente() {
        let libro = test_libro();
        let mut biblioteca = test_biblioteca();
        biblioteca.agregar_libro_a_disposicion(libro.clone(), 2);
        biblioteca.agregar_libro_a_disposicion(libro.clone(), 3);
        assert_eq!(biblioteca.libros_a_disposicion.len(), 1);
        assert_eq!(biblioteca.obtener_cant_copias(&libro), 5);
    }

    //JSON Tests
    #[test]
    fn test_guardar_en_archivo_exito() {
        use std::fs;
        let libro = test_libro();
        let libros = vec![Libros::new(libro.clone(), 2)];
        let prestamos = vec![];
        let biblioteca = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            libros,
            prestamos,
        );
        let path = "test_biblioteca_guardar.json";
        let res = biblioteca.guardar_en_archivo(path);
        assert!(res.is_ok());
        let contenido = fs::read_to_string(path).unwrap();
        assert!(contenido.contains("Rust Book"));
        let _ = fs::remove_file(path);
    }
    #[test]
    fn test_guardar_en_archivo_error() {
        let libro = Libro::new(
            "123".to_string(),
            "Rust Book".to_string(),
            "Ferris".to_string(),
            300,
            Genero::Tecnico,
        );
        let libros = vec![Libros::new(libro.clone(), 2)];
        let prestamos = vec![];
        let biblioteca = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            libros,
            prestamos,
        );
        // Usamos un path inválido para forzar el error
        let res = biblioteca.guardar_en_archivo("/no_existe/test_biblioteca_guardar.json");
        match res {
            Err(ResultadoOperacion::ErrorArchivo(_)) => assert!(true),
            _ => panic!("Se esperaba un error de archivo"),
        }
    }
    #[test]
    fn test_cargar_de_archivo_exito() {
        use std::fs;
        let libro = test_libro();
        let libros = vec![Libros::new(libro.clone(), 2)];
        let prestamos = vec![];
        let path = "test_biblioteca_cargar.json";
        // Guardamos primero el archivo
        {
            let biblioteca = Biblioteca::new(
                "Mi Biblio".to_string(),
                "Calle 3".to_string(),
                libros.clone(),
                prestamos.clone(),
            );
            biblioteca.guardar_en_archivo(path).unwrap();
        }
        // Ahora cargamos en una biblioteca vacía
        let mut biblioteca2 =
            Biblioteca::new("Otra".to_string(), "Otra calle".to_string(), vec![], vec![]);
        let res = biblioteca2.cargar_de_archivo(path);
        assert!(res.is_ok());
        assert_eq!(biblioteca2.libros_a_disposicion.len(), 1);
        assert_eq!(biblioteca2.obtener_cant_copias(&libro), 2);
        let _ = fs::remove_file(path);
    }
    #[test]
    fn test_realizar_prestamo_y_guardar_exito() {
        use std::fs;
        let libro = test_libro();
        let cliente = test_cliente();
        let libros = vec![Libros::new(libro.clone(), 2)];
        let prestamos = vec![];
        let path = "test_prestamo_guardar.json";
        // Creamos y guardamos la biblioteca con libros disponibles
        {
            let biblioteca = Biblioteca::new(
                "Mi Biblio".to_string(),
                "Calle 3".to_string(),
                libros.clone(),
                prestamos.clone(),
            );
            biblioteca.guardar_en_archivo(path).unwrap();
        }
        // Ahora intentamos realizar el préstamo y guardar
        let fecha = test_fecha();
        let mut biblioteca2 = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            vec![],
            vec![],
        );
        let res = biblioteca2.realizar_prestamo_y_guardar(&libro, &cliente, fecha, path);
        match res {
            Ok(ResultadoOperacion::Exito) => assert!(true),
            _ => panic!("Se esperaba éxito en el préstamo"),
        }
        let contenido = fs::read_to_string(path).unwrap();
        assert!(contenido.contains("Rust Book"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_realizar_prestamo_y_guardar_fallo() {
        use std::fs;
        let libro = test_libro();
        let cliente = test_cliente();
        let libros = vec![]; // No hay copias disponibles
        let prestamos = vec![];
        let path = "test_prestamo_guardar_fallo.json";
        {
            let biblioteca = Biblioteca::new(
                "Mi Biblio".to_string(),
                "Calle 3".to_string(),
                libros.clone(),
                prestamos.clone(),
            );
            biblioteca.guardar_en_archivo(path).unwrap();
        }
        let fecha = test_fecha();
        let mut biblioteca2 = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            vec![],
            vec![],
        );
        let res = biblioteca2.realizar_prestamo_y_guardar(&libro, &cliente, fecha, path);
        match res {
            Err(ResultadoOperacion::Fallo(msg)) => {
                assert!(msg.contains("No se pudo realizar el préstamo"))
            }
            _ => panic!("Se esperaba error de préstamo"),
        }
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_cargar_de_archivo_error() {
        let mut biblioteca = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            vec![],
            vec![],
        );
        // Usamos un path inválido para forzar el error
        let res = biblioteca.cargar_de_archivo("/no_existe/test_biblioteca_cargar.json");
        match res {
            Err(ResultadoOperacion::ErrorArchivo(_)) => assert!(true),
            _ => panic!("Se esperaba un error de archivo"),
        }
    }
    #[test]
    fn test_devolver_libro_y_guardar_exito() {
        use std::fs;
        let libro = test_libro();
        let cliente = test_cliente();
        let fecha = test_fecha();
        let libros = vec![Libros::new(libro.clone(), 1)];
        let prestamos = vec![];
        let path = "test_devolver_libro_guardar_ok.json";
        // Creamos y guardamos la biblioteca con un préstamo en curso
        {
            let mut biblioteca = Biblioteca::new(
                "Mi Biblio".to_string(),
                "Calle 3".to_string(),
                libros.clone(),
                prestamos.clone(),
            );
            biblioteca.realizar_prestamo(&libro, &cliente, fecha.clone());
            biblioteca.guardar_en_archivo(path).unwrap();
        }
        // Ahora devolvemos el libro y guardamos
        let fecha_devolucion = test_fecha();
        let mut biblioteca2 = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            vec![],
            vec![],
        );
        let res = biblioteca2.devolver_libro_y_guardar(&libro, &cliente, fecha_devolucion, path);
        match res {
            Ok(ResultadoOperacion::Exito) => assert!(true),
            _ => panic!("Se esperaba éxito al devolver el libro"),
        }
        let contenido = fs::read_to_string(path).unwrap();
        assert!(contenido.contains("Devuelto"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_devolver_libro_y_guardar_fallo() {
        use std::fs;
        let libro = test_libro();
        let cliente = test_cliente();
        let libros = vec![Libros::new(libro.clone(), 1)];
        let prestamos = vec![]; // No hay préstamo registrado
        let path = "test_devolver_libro_guardar_fail.json";
        {
            let biblioteca = Biblioteca::new(
                "Mi Biblio".to_string(),
                "Calle 3".to_string(),
                libros.clone(),
                prestamos.clone(),
            );
            biblioteca.guardar_en_archivo(path).unwrap();
        }
        let fecha_devolucion = test_fecha();
        let mut biblioteca2 = Biblioteca::new(
            "Mi Biblio".to_string(),
            "Calle 3".to_string(),
            vec![],
            vec![],
        );
        let res = biblioteca2.devolver_libro_y_guardar(&libro, &cliente, fecha_devolucion, path);
        match res {
            Err(ResultadoOperacion::Fallo(msg)) => {
                assert!(msg.contains("No se pudo devolver el libro"))
            }
            _ => panic!("Se esperaba error al devolver el libro"),
        }
        let _ = fs::remove_file(path);
    }
}
