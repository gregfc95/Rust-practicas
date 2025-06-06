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
*/
use crate::tp3::ej3::Fecha;
#[derive(Debug, Clone)]
pub enum Genero {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}
#[derive(Debug, Clone)]
pub enum Estado {
    EnPrestamo,
    Devuelto,
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

struct Bibilioteca {
    nombre: String,
    direccion: String,
    libros_a_disposicion: Vec<Libros>,
    prestamos: Vec<Prestamo>,
}
struct Libros {
    tupla_libro: (Libro, u32),
}
#[derive(Debug, Clone)]
struct Libro {
    isbn: String,
    titulo: String,
    autor: String,
    num_paginas: u32,
    genero: Genero,
}
#[derive(Debug, Clone)]
struct Cliente {
    nombre: String,
    telefono: String,
    email: String,
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
#[derive(Debug, Clone)]
struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: Estado,
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

impl Bibilioteca {
    pub fn new(
        nombre: String,
        direccion: String,
        libros_a_disposicion: Vec<Libros>,
        prestamos: Vec<Prestamo>,
    ) -> Self {
        Bibilioteca {
            nombre,
            direccion,
            libros_a_disposicion,
            prestamos,
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

#[cfg(test)]
mod tests {
    use super::*;

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
        let biblioteca = Bibilioteca::new(
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
        let mut biblioteca = Bibilioteca::new(
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
        let mut biblioteca = Bibilioteca::new(
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
        let mut biblioteca = Bibilioteca::new(
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
        let mut biblioteca = Bibilioteca::new(
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
        let mut biblioteca = Bibilioteca::new(
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
        let mut biblioteca = Bibilioteca::new(
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
        let mut biblioteca = Bibilioteca::new(
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
        let mut biblioteca = Bibilioteca::new(
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
}
