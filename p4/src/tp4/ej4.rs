/*
4 -Se requiere implementar un sistema de ventas de productos. De cada producto se
conoce el nombre, una categoría y un precio base, y algunos productos pueden tener
descuentos aplicables dependiendo de la categoría. Además, se debe registrar al vendedor
que realizó la venta y al cliente. De ellos se conoce nombre, apellido, dirección, dni y del
vendedor nro de legajo, antigüedad y salario. Los clientes pueden tener un beneficio de
descuento si tienen suscripción al newsletter, de ser así se tiene el correo electrónico del
mismo.

El sistema debe permitir registrar las ventas realizadas y asociar el medio de pago utilizado.
Los medios de pago aceptados son: tarjeta de crédito, tarjeta de débito, transferencia
bancaria y efectivo.
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones:
➢ Crear una venta con: fecha, cliente, vendedor, medio de pago y un listado de
productos.
➢ Calcular el precio final de una venta en base a los productos que hay en ella. Para
calcularlo tenga en cuenta que pueden haber determinados productos de alguna
categoría donde debería aplicarse un descuento. Tanto la categoría como el
porcentaje de descuento a aplicar son datos que le brinda el sistema. Es decir el
sistema tiene una lista de las categorías con el descuento a aplicar. Además se debe
aplicar un porcentaje de descuento general si el cliente tiene suscripción al
newsletter.
➢ Para llevar un control de las ventas realizadas, se debe implementar un reporte que
permita visualizar las ventas totales por categoría de producto y otro por vendedor.

*/
use super::Fecha;
use std::collections::HashMap;

//Enums
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MedioPagoTipo {
    /// Pago en efectivo, sin datos adicionales.
    Efectivo,
    /// Pago con tarjeta de crédito, requiere información del titular.
    TarjetaDeCredito(TitularInfo),
    /// Pago con tarjeta de debito, requiere información del titular.
    TarjetaDeDebito(TitularInfo),
    /// Transferencia bancaria, requiere datos del titular y CBU.
    TransferenciaBancaria(TransferenciaInfo),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CategoriaProducto {
    Tecnologia,
    Alimentos,
    Ropa,
    Hogar,
    Otros,
}

//Traits
trait Descuento {
    fn obtener_descuento(&self) -> f64;
}
trait DescuentoCliente {
    fn descuento_newsletter(&self) -> f64;
}
//Structs
struct Reporte {
    ventas: Vec<Venta>,
}

struct Venta {
    fecha: Fecha,
    cliente: Cliente,
    vendedor: Vendedor,
    medio_pago: MedioPagoTipo,
    productos: Vec<ProductoVendido>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub struct TitularInfo {
    /// Nombre del titular del medio de pago.
    nombre: String,
    /// Número del medio de pago (tarjeta, cuenta, etc.).
    numero: String,
    /// Fecha de vencimiento del medio de pago.
    fecha_vencimiento: Fecha,
    /// Código de seguridad (CVV).
    codigo_seguridad: String,
}
struct Producto {
    nombre: String,
    categoria: CategoriaProducto,
    precio_base: f64,
}
struct ProductoVendido {
    producto: Producto,
    cantidad: u32,
}
struct Persona {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: String,
}
struct Vendedor {
    persona: Persona,
    legajo: u32,
    salario: f64,
    antiguedad: u32,
}
struct Cliente {
    persona: Persona,
    newsletter: bool,
    correo: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransferenciaInfo {
    /// Nombre del titular de la cuenta.
    titular: String,
    /// Clave Bancaria Uniforme (CBU).
    cbu: String,
    ///ID de la transferencia
    id_transferencia: String,
}

// Implementaciones de las estructuras

impl TitularInfo {
    pub fn new(
        nombre: String,
        numero: String,
        fecha_vencimiento: Fecha,
        codigo_seguridad: String,
    ) -> Self {
        TitularInfo {
            nombre,
            numero,
            fecha_vencimiento,
            codigo_seguridad,
        }
    }
}

impl TransferenciaInfo {
    pub fn new(titular: String, cbu: String, id_transferencia: String) -> Self {
        TransferenciaInfo {
            titular,
            cbu,
            id_transferencia,
        }
    }
}

impl Descuento for CategoriaProducto {
    fn obtener_descuento(&self) -> f64 {
        match self {
            CategoriaProducto::Tecnologia => 0.0,
            CategoriaProducto::Alimentos => 0.02,
            CategoriaProducto::Ropa => 0.01,
            CategoriaProducto::Hogar => 0.03,
            CategoriaProducto::Otros => 0.01,
        }
    }
}

impl DescuentoCliente for Cliente {
    fn descuento_newsletter(&self) -> f64 {
        if self.newsletter { 0.20 } else { 0.0 }
    }
}

/// Información para pagos por transferencia bancaria.

impl Venta {
    pub fn new(
        fecha: Fecha,
        cliente: Cliente,
        vendedor: Vendedor,
        medio_pago: MedioPagoTipo,
        productos: Vec<ProductoVendido>,
    ) -> Self {
        Venta {
            fecha,
            cliente,
            vendedor,
            medio_pago,
            productos,
        }
    }

    pub fn descuento_newsletter(&self, newsletter: bool) -> f64 {
        if newsletter {
            return 0.20;
        } else {
            return 0.0;
        }
    }

    pub fn calcular_precio_final(&self) -> f64 {
        let subtotal: f64 = self
            .productos
            .iter()
            .map(|p| {
                let base_total = p.producto.precio_base * (p.cantidad as f64);
                let descuento = p.producto.categoria.obtener_descuento();
                base_total * (1.0 - descuento)
            })
            .sum();

        let descuento_newsletter = self.cliente.descuento_newsletter();
        subtotal * (1.0 - descuento_newsletter)
    }
}

impl Reporte {
    pub fn ventas_por_categoria(&self) -> HashMap<CategoriaProducto, f64> {
        let mut resumen: HashMap<CategoriaProducto, f64> = HashMap::new();

        for venta in &self.ventas {
            for producto_vendido in &venta.productos {
                let categoria = &producto_vendido.producto.categoria;
                let base = producto_vendido.producto.precio_base;
                let cantidad = producto_vendido.cantidad;
                let descuento = categoria.obtener_descuento();
                let total = base * (cantidad as f64) * (1.0 - descuento);

                resumen
                    .entry(categoria.clone())
                    .and_modify(|v| *v += total)
                    .or_insert(total);
            }
        }

        resumen
    }

    pub fn ventas_por_vendedor(&self) -> HashMap<u32, f64> {
        let mut resumen: HashMap<u32, f64> = HashMap::new(); // legajo -> total vendido

        for venta in &self.ventas {
            let legajo = venta.vendedor.legajo;
            let total = venta.calcular_precio_final();

            resumen
                .entry(legajo)
                .and_modify(|v| *v += total)
                .or_insert(total);
        }

        resumen
    }
}

#[cfg(test)]
mod tests {
    use super::CategoriaProducto::*;
    use super::MedioPagoTipo::*;
    use super::*;

    fn fecha_dummy() -> Fecha {
        Fecha::new(1, 1, 2000).expect("Fecha inválida en fecha_dummy")
    }
    fn titular_info() -> TitularInfo {
        TitularInfo::new(
            "Juan Perez".to_string(),
            "1234".to_string(),
            fecha_dummy(),
            "999".to_string(),
        )
    }
    fn transferencia_info() -> TransferenciaInfo {
        TransferenciaInfo::new("Pedro".to_string(), "CBU123".to_string(), "ID2".to_string())
    }
    fn persona(nombre: &str) -> Persona {
        Persona {
            nombre: nombre.to_string(),
            apellido: "Apellido".to_string(),
            direccion: "Calle 1".to_string(),
            dni: "12345678".to_string(),
        }
    }
    fn vendedor(legajo: u32) -> Vendedor {
        Vendedor {
            persona: persona("Vendedor"),
            legajo,
            salario: 1000.0,
            antiguedad: 5,
        }
    }
    fn cliente(newsletter: bool) -> Cliente {
        Cliente {
            persona: persona("Cliente"),
            newsletter,
            correo: if newsletter {
                Some("mail@mail.com".to_string())
            } else {
                None
            },
        }
    }
    fn producto(nombre: &str, cat: CategoriaProducto, precio: f64) -> Producto {
        Producto {
            nombre: nombre.to_string(),
            categoria: cat,
            precio_base: precio,
        }
    }
    fn prod_vendido(
        nombre: &str,
        cat: CategoriaProducto,
        precio: f64,
        cantidad: u32,
    ) -> ProductoVendido {
        ProductoVendido {
            producto: producto(nombre, cat, precio),
            cantidad,
        }
    }

    #[test]
    fn test_descuento_categoria() {
        assert_eq!(Tecnologia.obtener_descuento(), 0.0);
        assert_eq!(Alimentos.obtener_descuento(), 0.02);
        assert_eq!(Ropa.obtener_descuento(), 0.01);
    }

    #[test]
    fn test_descuento_cliente_newsletter() {
        let c = cliente(true);
        let c2 = cliente(false);
        assert_eq!(c.descuento_newsletter(), 0.20);
        assert_eq!(c2.descuento_newsletter(), 0.0);
    }

    #[test]
    fn test_venta_precio_final_sin_descuentos() {
        let v = Venta::new(
            fecha_dummy(),
            cliente(false),
            vendedor(1),
            Efectivo,
            vec![prod_vendido("Notebook", Tecnologia, 1000.0, 1)],
        );
        assert_eq!(v.calcular_precio_final(), 1000.0);
    }

    #[test]
    fn test_venta_precio_final_con_descuento_categoria() {
        let v = Venta::new(
            fecha_dummy(),
            cliente(false),
            vendedor(1),
            Efectivo,
            vec![prod_vendido("Pan", Alimentos, 100.0, 2)],
        );
        // 2% descuento en alimentos
        assert!((v.calcular_precio_final() - 196.0).abs() < 0.01);
    }

    #[test]
    fn test_venta_precio_final_con_newsletter() {
        let v = Venta::new(
            fecha_dummy(),
            cliente(true),
            vendedor(1),
            Efectivo,
            vec![prod_vendido("Remera", Ropa, 200.0, 1)],
        );
        // 1% descuento ropa, luego 20% newsletter
        let esperado = 200.0 * 0.99 * 0.8;
        assert!((v.calcular_precio_final() - esperado).abs() < 0.01);
    }

    #[test]
    fn test_reporte_ventas_por_categoria() {
        let v1 = Venta::new(
            fecha_dummy(),
            cliente(false),
            vendedor(1),
            Efectivo,
            vec![prod_vendido("Remera", Ropa, 100.0, 2)],
        );
        let v2 = Venta::new(
            fecha_dummy(),
            cliente(false),
            vendedor(2),
            Efectivo,
            vec![prod_vendido("Silla", Hogar, 300.0, 1)],
        );
        let rep = Reporte {
            ventas: vec![v1, v2],
        };
        let cat = rep.ventas_por_categoria();
        assert!((cat[&Ropa] - 198.0).abs() < 0.01);
        assert!((cat[&Hogar] - 291.0).abs() < 0.01);
    }

    #[test]
    fn test_reporte_ventas_por_vendedor() {
        let v1 = Venta::new(
            fecha_dummy(),
            cliente(false),
            vendedor(1),
            Efectivo,
            vec![prod_vendido("Remera", Ropa, 100.0, 2)],
        );
        let v2 = Venta::new(
            fecha_dummy(),
            cliente(true),
            vendedor(1),
            Efectivo,
            vec![prod_vendido("Silla", Hogar, 300.0, 1)],
        );
        let v3 = Venta::new(
            fecha_dummy(),
            cliente(false),
            vendedor(2),
            Efectivo,
            vec![prod_vendido("Pan", Alimentos, 50.0, 2)],
        );
        let rep = Reporte {
            ventas: vec![v1, v2, v3],
        };
        let vend = rep.ventas_por_vendedor();
        // vendedor 1 hizo dos ventas
        assert!(vend.contains_key(&1));
        assert!(vend.contains_key(&2));
        // vendedor 1 total: (100*2*0.99) + (300*0.97*0.8)
        let esperado = 198.0 + 232.8;
        assert!((vend[&1] - esperado).abs() < 0.01);
    }
}
