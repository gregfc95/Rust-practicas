/*
7- Defina una estructura llamada ConcesionarioAuto donde se conoce el nombre, la
dirección y tiene una capacidad máxima para albergar X cantidad de autos. De los autos se
conocen los campos de la marca, modelo, año, precio bruto y color que pueden ser:rojo,
verde, azul, amarillo, blanco o negro.
Para dichas estructuras implemente los siguientes métodos:
❖ ConcesionarioAuto:
➢ new: que pasando los parámetros correspondientes, crea un
ConcesionarioAuto y lo retorna.
➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar
la máxima cantidad para albergarlos y retorna true, en caso de que lo supere
no lo agrega y retorna false.
➢ eliminar_auto(auto): elimina un auto de la lista de autos.
➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
❖ Auto:
➢ new: que pasando los parámetros correspondientes, crea un Auto y lo
retorna.
➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
■ si es de color primario le aplica un recargo del 25%, sino le aplica un
descuento del 10%.
■ si la marca es BMW le aplica un recargo del 15%-
■ si el año es menor a 2000 le aplica un descuento del 5%.
*/
#[derive(Debug, PartialEq)]
pub enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

pub struct Auto {
    marca: String,
    modelo: String,
    anio: u32,
    precio_bruto: f32,
    color: Color,
}

pub struct Concesionario {
    nombre: String,
    direccion: String,
    capacidad_maxima: u32,
    autos: Vec<Auto>,
}

impl Concesionario {
    pub fn new(nombre: String, direccion: String, capacidad_maxima: u32, autos: Vec<Auto>) -> Self {
        Concesionario {
            nombre,
            direccion,
            capacidad_maxima,
            autos,
        }
    }
    pub fn agregar_auto(&mut self, auto: Auto) -> bool {
        if self.autos.len() < self.capacidad_maxima as usize {
            self.autos.push(auto);
            true
        } else {
            false
        }
    }

    pub fn eliminar_auto(&mut self, auto: &Auto) -> Option<Auto> {
        //TODO PartialEq
        let index = self.autos.iter().position(|a| {
            a.anio == auto.anio
                && a.modelo == auto.modelo
                && a.marca == auto.marca
                && a.color == auto.color
        })?;
        Some(self.autos.remove(index))
    }

    pub fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        self.autos.iter().find(|a| {
            a.anio == auto.anio
                && a.modelo == auto.modelo
                && a.marca == auto.marca
                && a.color == auto.color
        })
    }
}

impl Auto {
    pub fn new(marca: String, modelo: String, anio: u32, precio_bruto: f32, color: Color) -> Self {
        Auto {
            marca,
            modelo,
            anio,
            precio_bruto,
            color,
        }
    }

    pub fn calcular_precio(&self) -> f32 {
        let mut precio = self.precio_bruto;
        if matches!(self.color, Color::Rojo | Color::Amarillo | Color::Azul) {
            precio = Auto::recargo_color_primario(precio)
        } else {
            precio = Auto::descuento_color(precio)
        }
        if self.marca.eq_ignore_ascii_case("bmw") {
            precio = Auto::recargo_marca_bmw(precio)
        }
        if self.anio < 2000 {
            precio = Auto::descuento_anio(precio)
        }
        precio
    }

    fn recargo_color_primario(precio: f32) -> f32 {
        return precio * 1.25;
    }
    fn recargo_marca_bmw(precio: f32) -> f32 {
        return precio * 1.15;
    }
    fn descuento_color(precio: f32) -> f32 {
        return precio * 0.90;
    }
    fn descuento_anio(precio: f32) -> f32 {
        return precio * 0.95;
    }
}

//TESTS
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    #[test]
    fn crear_auto() {
        let auto = Auto::new(
            "bmw".to_string(),
            "x5".to_string(),
            2010,
            10000.0,
            Color::Negro,
        );
        assert_eq!(auto.marca, "bmw");
        assert_eq!(auto.anio, 2010);
        assert_eq!(auto.color, Color::Negro);
        assert_eq!(auto.precio_bruto, 10000.0);
        assert_eq!(auto.modelo, "x5");
    }

    #[test]
    fn crear_concesionario() {
        let concesionario =
            Concesionario::new("TuAutoExpress".to_string(), "calle".to_string(), 10, vec![]);
        assert_eq!(concesionario.nombre, "TuAutoExpress");
        assert_eq!(concesionario.direccion, "calle");
        assert_eq!(concesionario.capacidad_maxima, 10);
    }

    #[test]
    fn test_calcular_precio_recargo_color() {
        let a1 = Auto::new(
            "Renault".to_string(),
            "x5".to_string(),
            2010,
            10000.0,
            Color::Amarillo,
        );
        assert_eq!(a1.calcular_precio(), 12500.0);
    }
    #[test]
    fn test_calcular_precio_descuento_color() {
        let a1 = Auto::new(
            "Renault".to_string(),
            "x5".to_string(),
            2010,
            10000.0,
            Color::Negro,
        );
        assert_eq!(a1.calcular_precio(), 9000.0);
    }

    #[test]
    fn test_calcular_precio_recargo_marca_y_color() {
        let a1 = Auto::new(
            "BMW".to_string(),
            "x5".to_string(),
            2010,
            10000.0,
            Color::Azul,
        );
        assert_eq!(a1.calcular_precio(), 14375.0);
        assert_eq!(a1.precio_bruto, 10000.0);
    }

    #[test]
    fn test_calcular_precio_descuento_color_anio() {
        let a1 = Auto::new(
            "Toyota".to_string(),
            "x5".to_string(),
            1995,
            10000.0,
            Color::Blanco,
        );
        assert_eq!(a1.calcular_precio(), 8550.0);
        assert_eq!(a1.precio_bruto, 10000.0);
    }
}
