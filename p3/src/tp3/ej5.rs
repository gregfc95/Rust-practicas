/*
5- Escribir un programa que defina una estructura Producto que tenga campos para el
nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los
siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Producto y lo retorna.
➢ calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre
el precio bruto
➢ aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de
descuento sobre el precio bruto
➢ calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el
precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los
parámetros son opcionales.
*/

pub struct Producto {
    nombre: String,
    precio_bruto: f32,
    id: u32,
}

impl Producto {
    pub fn new(nombre: String, precio_bruto: f32, id: u32) -> Producto {
        Producto {
            nombre,
            precio_bruto,
            id,
        }
    }
    pub fn calcular_impuestos(&self, imp: f32) -> f32 {
        self.precio_bruto * (imp / 100.0)
    }
    pub fn aplicar_descuento(&self, desc: f32) -> f32 {
        self.precio_bruto * (desc / 100.0)
    }

    pub fn calcular_precio_total(&self, imp: Option<f32>, desc: Option<f32>) -> f32 {
        let impuesto = match imp {
            Some(i) => i,
            None => 0.0,
        };
        let descuento = match desc {
            Some(d) => d,
            None => 0.0,
        };
        let resultado: f32 = self.precio_bruto + self.calcular_impuestos(impuesto)
            - self.aplicar_descuento(descuento);
        resultado
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Producto::new("p".to_string(), 1.0, 1);
        assert_eq!(p.precio_bruto, 1.0);
        assert_eq!(p.id, 1);
        assert_eq!(p.nombre, "p".to_string());
    }
    #[test]
    fn test_calcular_precio_total() {
        let p = Producto::new("p".to_string(), 50.0, 1);
        let imp = Some(21.0);
        let desc = Some(10.0);
        assert_eq!(p.calcular_precio_total(imp, desc), 55.5);
    }

    #[test]
    fn test_calcular_precio_total_0() {
        let p = Producto::new("p".to_string(), 50.0, 1);
        assert_eq!(p.calcular_precio_total(None, None), 50.0);
    }

    #[test]
    fn test_calcular_impuestos() {
        let p = Producto::new("p".to_string(), 50.0, 1);
        assert_eq!(p.calcular_impuestos(21.0), 10.5);
    }
    #[test]
    fn test_calcular_impuestos_0() {
        let p = Producto::new("p".to_string(), 50.0, 1);
        assert_eq!(p.calcular_impuestos(0.0), 0.0);
    }

    #[test]
    fn test_calcular_descuento() {
        let p = Producto::new("p".to_string(), 50.0, 1);
        assert_eq!(p.aplicar_descuento(10.0), 5.0);
    }

    #[test]
    fn test_calcular_descuento_0() {
        let p = Producto::new("p".to_string(), 50.0, 1);
        assert_eq!(p.aplicar_descuento(0.0), 0.0);
    }
}
