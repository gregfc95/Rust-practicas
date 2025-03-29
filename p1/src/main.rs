mod ej1;
mod ej2;
mod ej3;
mod ej4;
use std::io;

fn main() {
    println!("Selecciona un ejercicio (1-4):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer entrada");
    let input = input.trim();

    let opcion: u32 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada no válida. Debe ser un número.");
            return;
        }
    };

    match opcion {
        1 => ej1::ej1(),
        2 => ej2::ej2(),
        3 => ej3::ej3(),
        4 => ej4::ej4(),
        _ => println!("Opción no válida"),
    }
}
