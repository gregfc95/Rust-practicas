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

pub enum Estado {
    EnPrestamo,
    Devuelto,
}

struct Bibilioteca {
    nombre: String,
    direccion: String,
    libros_a_disposicion: Vec<Libros>,
    clientes: Vec<Cliente>,
    prestamos: Vec<Prestamo>,
}
struct Libros {
    tupla_libro: (Libro, u32),
}
struct Libro {
    isbn: String,
    titulo: String,
    autor: String,
    num_paginas: u32,
    genero: Genero,
}

struct Cliente {
    nombre: String,
    telefono: String,
    email: String,
}

struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Fecha,
    estado: Estado,
}

impl Bibilioteca {
    pub fn new(
        nombre: String,
        direccion: String,
        libros_a_disposicion: Vec<Libros>,
        clientes: Vec<Cliente>,
        prestamos: Vec<Prestamo>,
    ) -> Self {
        Bibilioteca {
            nombre,
            direccion,
            libros_a_disposicion,
            clientes,
            prestamos,
        }
    }

    pub fn incrementar_copia_libro(&mut self, libro: Libro) {
        if let Some(index) = self
            .libros_a_disposicion
            .iter()
            .position(|L| L.tupla_libro.0.igual_isbn(&libro))
        {
            self.libros_a_disposicion[index].incrementar();
        }
    }

    pub fn decrementar_copia_libro(&mut self, libro: Libro) {
        if let Some(index) = self
            .libros_a_disposicion
            .iter()
            .position(|L| L.tupla_libro.0.igual_isbn(&libro))
        {
            self.libros_a_disposicion[index].decrementar();
        }
    }

    pub fn obtener_cant_copias(&self, libro: Libro) -> u32 {
        if let Some(index) = self
            .libros_a_disposicion
            .iter()
            .position(|L| L.tupla_libro.0.igual_isbn(&libro))
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
