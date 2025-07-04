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

TP4 Ej4

🧾 Implementar una funcionalidad que permita obtener un informe de ventas realizadas por un vendedor específico, filtrando solo aquellas ventas que contengan al menos un producto de una categoría dada.

Este informe debe incluir, ordenado cronológicamente de la venta más reciente a la más antigua, lo siguiente para cada venta:

-Fecha de la venta
-Productos vendidos y sus cantidades
-Monto total final de la venta
-Medio de pago utilizado

La consulta se debe realizar a partir de un identificador único del vendedor (por ejemplo, su número de legajo, según cómo lo hayan modelado), y una categoría de producto como filtro.

debe retonar una lista de venta con que haya uno de dicha categoria, se retorna esa venta
manejar si el vendedor existe

En caso de que el vendedor no tenga ventas que cumplan esa condición, el sistema debe reflejar esa situación de forma adecuada.

🔧 Esta funcionalidad debe implementarse como un método dentro del struct principal del sistema.

🧪 Además, deben incluir los tests necesarios para verificar el correcto funcionamiento de esta funcionalidad.

📌 Firma esperada del método:
get_historial_ventas(id: id_vendedor, categoria: CategoriaProducto) -> ???
*/
//Jose Fernandez legajo 19639/4

use std::{collections::HashMap, hash::Hash};

//Fecha

use chrono::prelude::*;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fecha {
    dia: u32,
    mes: u32,
    anio: i32,
}

impl Fecha {
    pub fn new(dia: u32, mes: u32, anio: i32) -> Option<Fecha> {
        if NaiveDate::from_ymd_opt(anio, mes, dia).is_some() {
            Some(Fecha { dia, mes, anio })
        } else {
            None
        }
    }

    pub fn new_fecha_actual() -> Fecha {
        let hoy = chrono::Local::now().naive_local();
        Fecha::new(hoy.day(), hoy.month(), hoy.year()).unwrap()
    }
    pub fn es_fecha_valida(&self) -> bool {
        let okay: bool = NaiveDate::from_ymd_opt(self.anio, self.mes, self.dia).is_some();
        okay
    }

    pub fn es_bisiesto(&self) -> bool {
        NaiveDate::from_ymd_opt(self.anio, self.mes, self.dia)
            .unwrap()
            .leap_year()
    }

    pub fn sumar_dias(&self, dias: u32) -> Option<Fecha> {
        NaiveDate::from_ymd_opt(self.anio, self.mes, self.dia)?
            .checked_add_days(chrono::Days::new(dias.into()))
            .map(|d| Fecha {
                anio: d.year(),
                mes: d.month(),
                dia: d.day(),
            })
    }

    pub fn restar_dias(&self, dias: u32) -> Option<Fecha> {
        NaiveDate::from_ymd_opt(self.anio, self.mes, self.dia)?
            .checked_sub_days(chrono::Days::new(dias.into()))
            .map(|d| Fecha {
                anio: d.year(),
                mes: d.month(),
                dia: d.day(),
            })
    }

    pub fn es_mayor(&self, otra_fecha: &Self) -> bool {
        NaiveDate::from_ymd_opt(self.anio, self.mes, self.dia)
            > NaiveDate::from_ymd_opt(otra_fecha.anio, otra_fecha.mes, otra_fecha.dia)
    }
    pub fn igual(&self, otra: &Fecha) -> bool {
        self.dia == otra.dia && self.mes == otra.mes && self.anio == otra.anio
    }
}

//Ejercicio 4 INICIO
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

#[derive(Debug, Clone, PartialEq)]
struct Reporte {
    /// Lista de ventas asociadas a este reporte.
    por_categoria: Vec<ReporteCategoria>,
    por_vendedor: Vec<ReporteVendedor>,
}
#[derive(Debug, Clone, PartialEq)]
struct ReporteCategoria {
    categoria: CategoriaProducto,
    total_ventas: u32,
}
#[derive(Debug, Clone, PartialEq)]
struct ReporteVendedor {
    vendedor: u32,
    total_ventas: u32,
}
#[derive(Debug, Clone, PartialEq)]

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
#[derive(Debug, Clone, PartialEq)]

struct Producto {
    nombre: String,
    categoria: CategoriaProducto,
    precio_base: f64,
}
#[derive(Debug, Clone, PartialEq)]

struct ProductoVendido {
    producto: Producto,
    cantidad: u32,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]

struct Persona {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: String,
}
#[derive(Debug, Clone, PartialEq)]
struct Vendedor {
    persona: Persona,
    legajo: u32,
    salario: f64,
    antiguedad: u32,
}
#[derive(Debug, Clone, PartialEq)]

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
#[derive(Debug, Clone, PartialEq)]
struct SistemaVenta {
    vendedores: HashMap<u32, Vendedor>,
    clientes: HashMap<String, Cliente>,
    ventas: Vec<Venta>,
    reportes: Vec<Reporte>,
}
impl SistemaVenta {
    fn new() -> Self {
        SistemaVenta {
            vendedores: HashMap::new(),
            clientes: HashMap::new(),
            ventas: vec![],
            reportes: vec![],
        }
    }
    fn registrar_vendedor(&mut self, vendedor: Vendedor) {
        self.vendedores.insert(vendedor.legajo, vendedor);
    }
    fn registrar_cliente(&mut self, cliente: Cliente) {
        self.clientes.insert(cliente.persona.dni.clone(), cliente);
    }
    fn registrar_venta(&mut self, venta: Venta) {
        self.ventas.push(venta);
    }
    fn generar_reporte(&mut self) {
        let reporte = Reporte {
            por_categoria: self.ventas_por_categoria(),
            por_vendedor: self.ventas_por_vendedor(),
        };
        self.reportes.push(reporte);
    }
    fn ventas_por_categoria(&self) -> Vec<ReporteCategoria> {
        let mut mapa: HashMap<CategoriaProducto, u32> = HashMap::new();

        for venta in &self.ventas {
            for producto in &venta.productos {
                let cat = &producto.producto.categoria;
                *mapa.entry(cat.clone()).or_insert(0) += 1;
            }
        }

        mapa.into_iter()
            .map(|(categoria, cantidad_ventas)| ReporteCategoria {
                categoria,
                total_ventas: cantidad_ventas,
            })
            .collect()
    }
    fn ventas_por_vendedor(&self) -> Vec<ReporteVendedor> {
        let mut mapa: HashMap<u32, u32> = HashMap::new(); // legajo -> cantidad de ventas

        for venta in &self.ventas {
            *mapa.entry(venta.vendedor.legajo).or_insert(0) += 1;
        }

        mapa.into_iter()
            .map(|(legajo, cantidad_ventas)| ReporteVendedor {
                vendedor: legajo,
                total_ventas: cantidad_ventas,
            })
            .collect()
    }
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
/*
TP4 Ej4

🧾 Implementar una funcionalidad que permita obtener un informe de ventas realizadas por un vendedor específico, filtrando solo aquellas ventas que contengan al menos un producto de una categoría dada.

Este informe debe incluir, ordenado cronológicamente de la venta más reciente a la más antigua, lo siguiente para cada venta:

-Fecha de la venta
-Productos vendidos y sus cantidades
-Monto total final de la venta
-Medio de pago utilizado

La consulta se debe realizar a partir de un identificador único del vendedor (por ejemplo, su número de legajo, según cómo lo hayan modelado), y una categoría de producto como filtro.

*debe retonar una lista de venta con que haya uno de dicha categoria, se retorna esa venta
*manejar si el vendedor existe

En caso de que el vendedor no tenga ventas que cumplan esa condición, el sistema debe reflejar esa situación de forma adecuada.

🔧 Esta funcionalidad debe implementarse como un método dentro del struct principal del sistema.

🧪 Además, deben incluir los tests necesarios para verificar el correcto funcionamiento de esta funcionalidad.

📌 Firma esperada del método:
get_historial_ventas(id: id_vendedor, categoria: CategoriaProducto) -> ???
*/
#[derive(Debug, Clone, PartialEq)]
struct Informe {
    fecha: Fecha,
    productos_vendidos: Vec<ProductoVendido>,
    monto_total: f64,
    medio_pago: MedioPagoTipo,
}
//Profesor dijo que creara una nueva Struct para el segundo entregable, se copiaron todos los metodos usados en SistemaVenta
#[derive(Debug, Clone, PartialEq)]
struct SistemaEntregable {
    vendedores: HashMap<u32, Vendedor>, // legajo del vendedor
    clientes: HashMap<String, Cliente>, // dni del cliente
    ventas: Vec<Venta>,
    reportes: Vec<Reporte>,
    //Dado un vendedor, va a tener asociado un informe de ventas por categoria, es decir vendedor A dada una categoria va a volver un informe por venta
    informes: Vec<Informe>, //Esto deberia ser un HashMap con el legajo del vendedor -> lista de informes, pero el profesor necesita order por fecha y Hashmap no permite orden
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorSistema {
    VendedorNoEncontrado,
    NoHayVentas,
    VendedorNoTieneVentas,
}

impl Informe {
    fn new(
        fecha: Fecha,                             //fecha de la venta
        productos_vendidos: Vec<ProductoVendido>, //productos vendidos en la venta
        monto_total: f64,                         // monto total de la venta
        medio_pago: MedioPagoTipo,                // medio de pago utilizado para dicha venta
    ) -> Self {
        Informe {
            fecha: fecha,
            productos_vendidos,
            monto_total,
            medio_pago,
        }
    }
}
//Implementacion de sistema entregable con los metodos copiados de SistemaVenta
impl SistemaEntregable {
    fn new() -> Self {
        SistemaEntregable {
            vendedores: HashMap::new(), // legajo del vendedor
            clientes: HashMap::new(),   // dni del cliente
            ventas: vec![],
            reportes: vec![],
            informes: vec![], //Esto deberia ser un HashMap con el legajo del vendedor -> lista de informes, pero el profesor necesita order por fecha y Hashmap no permite orden
        }
    }
    fn registrar_vendedor(&mut self, vendedor: Vendedor) {
        self.vendedores.insert(vendedor.legajo, vendedor);
    }
    fn registrar_cliente(&mut self, cliente: Cliente) {
        self.clientes.insert(cliente.persona.dni.clone(), cliente);
    }
    fn registrar_venta(&mut self, venta: Venta) {
        self.ventas.push(venta);
    }
    fn generar_reporte(&mut self) {
        let reporte = Reporte {
            por_categoria: self.ventas_por_categoria(),
            por_vendedor: self.ventas_por_vendedor(),
        };
        self.reportes.push(reporte);
    }
    fn ventas_por_categoria(&self) -> Vec<ReporteCategoria> {
        let mut mapa: HashMap<CategoriaProducto, u32> = HashMap::new();

        for venta in &self.ventas {
            for producto in &venta.productos {
                let cat = &producto.producto.categoria;
                *mapa.entry(cat.clone()).or_insert(0) += 1;
            }
        }

        mapa.into_iter()
            .map(|(categoria, cantidad_ventas)| ReporteCategoria {
                categoria,
                total_ventas: cantidad_ventas,
            })
            .collect()
    }
    fn ventas_por_vendedor(&self) -> Vec<ReporteVendedor> {
        let mut mapa: HashMap<u32, u32> = HashMap::new(); // legajo -> cantidad de ventas

        for venta in &self.ventas {
            *mapa.entry(venta.vendedor.legajo).or_insert(0) += 1;
        }

        mapa.into_iter()
            .map(|(legajo, cantidad_ventas)| ReporteVendedor {
                vendedor: legajo,
                total_ventas: cantidad_ventas,
            })
            .collect()
    }

    /// Metodos para el entregable
    fn vendedor_existe(&self, id_legajo: &u32) -> Result<bool, ErrorSistema> {
        if self.vendedores.contains_key(id_legajo) {
            Ok(true)
        } else {
            Err(ErrorSistema::VendedorNoEncontrado)
        }
    }

    fn ventas_existe(&self) -> Result<bool, ErrorSistema> {
        if self.ventas.is_empty() {
            return Err(ErrorSistema::NoHayVentas);
        }
        Ok(true)
    }

    fn vendedor_no_tiene_ventas(
        &self,
        ventas_del_vendedor: &Vec<Venta>,
    ) -> Result<bool, ErrorSistema> {
        if ventas_del_vendedor.is_empty() {
            return Err(ErrorSistema::VendedorNoTieneVentas);
        }
        Ok(true)
    }

    fn filtrar_ventas_por_vendedor_y_categoria(
        &self,
        id_legajo: u32,
        categoria: CategoriaProducto,
    ) -> Vec<Venta> {
        self.ventas
            .iter()
            .filter(|v| {
                v.vendedor.legajo == id_legajo
                    && v.productos
                        .iter()
                        .any(|p| p.producto.categoria == categoria)
            })
            .cloned()
            .collect()
    }
    /// Ordena las ventas por fecha de forma mas reciente a mas antigua usando metodos de "fecha"
    fn ordenar_ventas_por_fecha_reciente_antiguo(&self, ventas: &mut Vec<Venta>) {
        ventas.sort_by(|a, b| {
            if a.fecha.igual(&b.fecha) {
                std::cmp::Ordering::Equal
            } else if a.fecha.es_mayor(&b.fecha) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
    }
    fn insertar_informe_en_sistema(
        &mut self,
        vec_informe: Vec<Informe>,
    ) -> Result<(), ErrorSistema> {
        // Insertar el informe en el sistema
        self.informes.extend(vec_informe);
        Ok(())
    }

    fn get_historial_venta(
        &self, //      &mut self si quieren que insertemos el informe en el sistema?
        id_legajo: u32,
        categoria: CategoriaProducto,
    ) -> Result<Vec<Informe>, ErrorSistema> {
        // Verificar si el vendedor existe, sino devuelve un ErrorSistema::VendedorNoEncontrado
        self.vendedor_existe(&id_legajo)?;
        // Verificar si hay ventas registradas, sino devuelve un ErrorSistema::NoHayVentas
        self.ventas_existe()?;

        //Aca tenemos filtramos las ventas por el vendedor y la categoria de producto
        let mut ventas_filtradas =
            self.filtrar_ventas_por_vendedor_y_categoria(id_legajo, categoria);

        self.vendedor_no_tiene_ventas(&ventas_filtradas)?;
        //Ordenar las ventas por fecha de forma mas reciente a mas antigua
        self.ordenar_ventas_por_fecha_reciente_antiguo(&mut ventas_filtradas);

        // Crear un vector de informes
        let mut vect_informes: Vec<Informe> = vec![];
        for venta in &ventas_filtradas {
            // Calcular el monto total de la venta
            let monto_total = venta.calcular_precio_final();
            // Obtener la fecha de la venta
            let fecha = venta.fecha.clone();
            // Obtener el medio de pago utilizado
            let medio_pago = venta.medio_pago.clone();
            // Obtener los productos vendidos de dicha venta
            let productos_vendidos = venta.productos.clone();
            // Crear el informe
            let informe = Informe::new(fecha, productos_vendidos, monto_total, medio_pago);
            vect_informes.push(informe);
        }
        // No se si correcto insertar en el sistema el informe y lo devolverlo? pero debo cambiar la firma a '&self' error: (cannot borrow `*self` as mutable, as it is behind a `&` reference)

        //self.insertar_informe_en_sistema(vect_informes.clone());
        // Retornamos el informe en forma de Vec porque necesito Orden (mas reciente)
        Ok(vect_informes)
    }
}

//Tests
#[cfg(test)]
mod tests {
    use core::error;

    use crate::tp4::fecha;

    use super::CategoriaProducto::*;
    use super::MedioPagoTipo::*;
    use super::*;

    //Funciones auxiliares para crear datos de prueba
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

    fn sistema_venta() -> SistemaVenta {
        let vendedor1 = vendedor(1);
        let mut sistema = SistemaVenta::new();
        //agregar vendedor
        sistema.registrar_vendedor(vendedor1);
        sistema
    }

    //Sistema entregable, se usa para los tests del entregable
    fn sistema_entregable() -> SistemaEntregable {
        let vendedor1 = vendedor(1);
        let mut sistema = SistemaEntregable::new();
        //agregar vendedor
        sistema.registrar_vendedor(vendedor1);
        sistema
    }

    fn ventas_dummy_1() -> Vec<Venta> {
        let v1 = Venta::new(
            Fecha::new_fecha_actual(),
            cliente(false),
            vendedor(1),
            Efectivo,
            vec![
                prod_vendido("Remera", Ropa, 100.0, 2),
                prod_vendido("Celular", Tecnologia, 1500.0, 2),
            ],
        );
        let v2 = Venta::new(
            Fecha::new(1, 1, 2024).expect("Fecha inválida en ventas_dummy_1"),
            cliente(false),
            vendedor(2),
            Efectivo,
            vec![prod_vendido("Silla", Hogar, 300.0, 1)],
        );
        let v3 = Venta::new(
            fecha_dummy(),
            cliente(false),
            vendedor(1),
            Efectivo,
            vec![prod_vendido("Pan", Alimentos, 50.0, 2)],
        );
        let v4 = Venta::new(
            fecha_dummy(),
            cliente(false),
            vendedor(2),
            Efectivo,
            vec![prod_vendido("Celular", Tecnologia, 500.0, 1)],
        );
        let v5 = Venta::new(
            Fecha::new(3, 1, 2024).expect("Fecha inválida en ventas_dummy_1"),
            cliente(false),
            vendedor(2),
            Efectivo,
            vec![prod_vendido("Mesa", Hogar, 200.0, 1)],
        );
        let ventas = vec![v1, v2, v3, v4, v5];
        ventas
    }

    #[test]
    fn test_sistema_entregable_new() {
        let sistema = SistemaEntregable::new();
        assert!(sistema.vendedores.is_empty());
        assert!(sistema.clientes.is_empty());
        assert!(sistema.ventas.is_empty());
        assert!(sistema.reportes.is_empty());
    }

    #[test]
    fn registrar_vendedor_entregable_exitoso() {
        let mut sistema = sistema_entregable();
        let v = vendedor(2);
        sistema.registrar_vendedor(v.clone());
        assert!(sistema.vendedores.contains_key(&v.legajo));
    }
    #[test]
    fn registrar_cliente_entregable_exitoso() {
        let mut sistema = sistema_entregable();
        let c = cliente(false);
        sistema.registrar_cliente(c.clone());
        assert!(sistema.clientes.contains_key(&c.persona.dni));
    }
    #[test]
    fn registrar_venta_entregable_exitoso() {
        let mut sistema = sistema_entregable();
        let v = ventas_dummy_1();

        for venta in v {
            sistema.registrar_venta(venta);
        }
        assert_eq!(sistema.ventas.len(), 5);
    }
    #[test]
    fn test_reporte_entregable_ventas_categoria() {
        let mut sistema = sistema_entregable();
        let ventas = ventas_dummy_1();
        for v in ventas {
            sistema.registrar_venta(v);
        }
        sistema.generar_reporte();
        let reporte = &sistema.reportes[0];
        let categorias = &reporte.por_categoria;
        assert_eq!(categorias.len(), 4); // Ropa, Hogar, Alimentos
        assert!(categorias.iter().any(|c| c.categoria == Ropa));
        assert!(categorias.iter().any(|c| c.categoria == Hogar));
        assert!(categorias.iter().any(|c| c.categoria == Alimentos));
    }

    #[test]
    fn test_reporte_entregable_ventas_vendedor() {
        let mut sistema = sistema_entregable();
        let ventas = ventas_dummy_1();
        for v in ventas {
            sistema.registrar_venta(v);
        }
        sistema.generar_reporte();
        let reporte = &sistema.reportes[0];
        let vendedores = &reporte.por_vendedor;
        assert_eq!(vendedores.len(), 2); // Vendedor 1 y Vendedor 2
        assert!(vendedores.iter().any(|v| v.vendedor == 1));
        assert!(vendedores.iter().any(|v| v.vendedor == 2));
    }

    #[test]
    fn test_vendedor_existe() {
        let sistema = sistema_entregable();
        assert!(sistema.vendedor_existe(&1).is_ok());
    }
    #[test]
    fn test_vendedor_no_existe() {
        let sistema = sistema_entregable();
        let error_esperado = ErrorSistema::VendedorNoEncontrado;
        assert!(sistema.vendedor_existe(&999).is_err());
        assert!(matches!(sistema.vendedor_existe(&999), error_esperado));
    }
    #[test]
    fn test_ventas_existe() {
        let mut sistema = sistema_entregable();
        let ventas = ventas_dummy_1();
        for v in ventas {
            sistema.registrar_venta(v);
        }
        // Debe ser Ok, porque sí hay ventas
        assert!(sistema.ventas_existe().is_ok());
    }
    #[test]
    fn test_ventas_no_existe() {
        let sistema = sistema_entregable();
        let error_esperado = ErrorSistema::NoHayVentas;
        assert!(sistema.ventas_existe().is_err());
        assert!(matches!(sistema.ventas_existe(), error_esperado));
    }
    #[test]
    fn test_vendedor_no_tiene_ventas() {
        let mut sistema = sistema_entregable();
        let ventas = ventas_dummy_1();
        for v in ventas {
            sistema.registrar_venta(v);
        }
        let vendedor_sin_ventas = vendedor(999);
        sistema.registrar_vendedor(vendedor_sin_ventas);
        let ventas_del_vendedor: Vec<Venta> = vec![];
        let error_esperado = ErrorSistema::VendedorNoTieneVentas;
        assert!(
            sistema
                .vendedor_no_tiene_ventas(&ventas_del_vendedor)
                .is_err()
        );
        assert!(matches!(
            sistema.vendedor_no_tiene_ventas(&ventas_del_vendedor),
            error_esperado
        ));
    }
    #[test]
    fn test_vendedor_tiene_ventas() {
        let mut sistema = sistema_entregable();
        let ventas = ventas_dummy_1();
        for v in ventas {
            sistema.registrar_venta(v);
        }
        // Filtrar las ventas del vendedor 1
        let ventas_del_vendedor: Vec<Venta> = sistema
            .ventas
            .iter()
            .filter(|venta| venta.vendedor.legajo == 1)
            .cloned()
            .collect();

        // Debe ser Ok, porque sí tiene ventas
        assert!(
            sistema
                .vendedor_no_tiene_ventas(&ventas_del_vendedor)
                .is_ok()
        );
    }
    #[test]
    fn test_ordenar_ventas_por_fecha_reciente_antiguo_positivo() {
        let mut sistema = sistema_entregable();
        // Fechas: actual, 2000, 2024
        let mut ventas = ventas_dummy_1();
        sistema.ordenar_ventas_por_fecha_reciente_antiguo(&mut ventas);
        // Verificar que las fechas están ordenadas de más reciente a más antigua
        assert_eq!(ventas.len(), 5);
        assert!(ventas[0].fecha.es_mayor(&ventas[1].fecha));
        assert!(ventas[1].fecha.es_mayor(&ventas[2].fecha));
    }
    #[test]
    fn test_filtrar_ventas_por_vendedor_y_categoria_positivo() {
        let mut sistema = sistema_entregable();
        let ventas = ventas_dummy_1();
        for v in ventas {
            sistema.registrar_venta(v);
        }
        // Vendedor 1 tiene ventas de Ropa y Alimentos y Tecnologia
        let ventas_filtradas = sistema.filtrar_ventas_por_vendedor_y_categoria(1, Ropa);
        assert!(!ventas_filtradas.is_empty());
        // Todas las ventas filtradas deben ser del vendedor 1 y contener al menos un producto de Ropa
        for venta in &ventas_filtradas {
            assert_eq!(venta.vendedor.legajo, 1);
            assert!(venta.productos.iter().any(|p| p.producto.categoria == Ropa));
        }
    }

    #[test]
    fn test_filtrar_ventas_por_vendedor_y_categoria_negativo() {
        let mut sistema = sistema_entregable();
        let ventas = ventas_dummy_1();
        for v in ventas {
            sistema.registrar_venta(v);
        }
        // Vendedor 1 no tiene ventas de Hogar
        let ventas_filtradas = sistema.filtrar_ventas_por_vendedor_y_categoria(1, Hogar);
        assert!(ventas_filtradas.is_empty());
    }

    #[test]
    fn test_get_historial_venta_error_vendedor_no_encontrado() {
        let sistema = sistema_entregable();
        let resultado = sistema.get_historial_venta(999, Ropa);
        assert!(matches!(resultado, Err(ErrorSistema::VendedorNoEncontrado)));
    }
    #[test]
    fn test_get_historial_venta_error_no_hay_ventas() {
        let mut sistema = sistema_entregable();
        // El vendedor existe, pero no hay ventas registradas
        let v = vendedor(1);
        sistema.registrar_vendedor(v);
        let resultado = sistema.get_historial_venta(1, Ropa);
        assert!(matches!(resultado, Err(ErrorSistema::NoHayVentas)));
    }
    #[test]
    fn test_get_historial_venta_error_vendedor_no_tiene_ventas_de_categoria() {
        let mut sistema = sistema_entregable();
        let ventas = ventas_dummy_1();
        for v in ventas {
            sistema.registrar_venta(v);
        }
        // Vendedor 1 no tiene ventas de Hogar
        let resultado = sistema.get_historial_venta(1, Hogar);
        assert!(matches!(
            resultado,
            Err(ErrorSistema::VendedorNoTieneVentas)
        ));
    }
    #[test]
    fn test_get_historial_venta_exitoso() {
        let mut sistema = sistema_entregable();
        let ventas = ventas_dummy_1();
        for v in ventas {
            sistema.registrar_venta(v);
        }
        // Vendedor 1 tiene ventas de Ropa y Alimentos y Tecnologia
        let resultado = sistema.get_historial_venta(1, Ropa);
        assert!(resultado.is_ok());
    }

    //Test SistemaVenta
    #[test]
    fn test_sistema_venta_new() {
        let sistema = SistemaVenta::new();
        assert!(sistema.vendedores.is_empty());
        assert!(sistema.clientes.is_empty());
        assert!(sistema.ventas.is_empty());
        assert!(sistema.reportes.is_empty());
    }

    #[test]
    fn test_transferencia_info_new() {
        let info = TransferenciaInfo::new(
            "Titular".to_string(),
            "CBU123".to_string(),
            "ID1".to_string(),
        );
        assert_eq!(info.titular, "Titular");
        assert_eq!(info.cbu, "CBU123");
        assert_eq!(info.id_transferencia, "ID1");
    }
    #[test]
    fn test_titular_new_tarjeta() {
        let info = TitularInfo::new(
            "Titular".to_string(),
            "1234".to_string(),
            fecha_dummy(),
            "999".to_string(),
        );
        assert_eq!(info.nombre, "Titular");
        assert_eq!(info.numero, "1234");
        assert!(info.fecha_vencimiento.es_fecha_valida());
        assert_eq!(info.codigo_seguridad, "999");
    }

    #[test]
    fn registrar_vendedor_exitoso() {
        let mut sistema = sistema_venta();
        let v = vendedor(2);
        sistema.registrar_vendedor(v.clone());
        assert!(sistema.vendedores.contains_key(&v.legajo));
    }
    #[test]
    fn registrar_cliente_exitoso() {
        let mut sistema = sistema_venta();
        let c = cliente(false);
        sistema.registrar_cliente(c.clone());
        assert!(sistema.clientes.contains_key(&c.persona.dni));
    }

    #[test]
    fn test_registrar_venta_exitoso() {
        let mut sistema = sistema_venta();
        let v = Venta::new(
            fecha_dummy(),
            cliente(false),
            vendedor(1),
            Efectivo,
            vec![prod_vendido("Notebook", Tecnologia, 1000.0, 1)],
        );
        sistema.registrar_venta(v.clone());
        assert!(sistema.ventas.contains(&v));
    }

    #[test]
    fn test_reporte_ventas_categoria() {
        let mut sistema = sistema_venta();
        let ventas = ventas_dummy_1();
        for v in ventas {
            sistema.registrar_venta(v);
        }
        sistema.generar_reporte();
        let reporte = &sistema.reportes[0];
        let categorias = &reporte.por_categoria;
        assert_eq!(categorias.len(), 4); // Ropa, Hogar, Alimentos
        assert!(categorias.iter().any(|c| c.categoria == Ropa));
        assert!(categorias.iter().any(|c| c.categoria == Hogar));
        assert!(categorias.iter().any(|c| c.categoria == Alimentos));
    }

    #[test]
    fn test_reporte_ventas_vendedor() {
        let mut sistema = sistema_venta();
        let ventas = ventas_dummy_1();
        for v in ventas {
            sistema.registrar_venta(v);
        }
        sistema.generar_reporte();
        let reporte = &sistema.reportes[0];
        let vendedores = &reporte.por_vendedor;
        assert_eq!(vendedores.len(), 2); // Vendedor 1 y Vendedor 2
        assert!(vendedores.iter().any(|v| v.vendedor == 1));
        assert!(vendedores.iter().any(|v| v.vendedor == 2));
    }

    #[test]
    fn test_reporte_ventas_vendedor_sin_ventas() {
        let mut sistema = sistema_venta();
        let ventas = ventas_dummy_1();
        let vendedor_sin_ventas = vendedor(3);
        sistema.registrar_vendedor(vendedor_sin_ventas);
        for v in ventas {
            sistema.registrar_venta(v);
        }
        sistema.generar_reporte();
        let reporte = &sistema.reportes[0];
        let vendedores = &reporte.por_vendedor;
        assert_eq!(vendedores.len(), 2); // Vendedor 1 y Vendedor 2
        assert!(vendedores.iter().any(|v| v.vendedor == 1));
        assert!(vendedores.iter().any(|v| v.vendedor == 2));
        // Vendedor 3 no tiene ventas, no debe aparecer en el reporte
        assert!(!vendedores.iter().any(|v| v.vendedor == 3));
    }

    #[test]
    fn test_descuento_categoria() {
        assert_eq!(Tecnologia.obtener_descuento(), 0.0);
        assert_eq!(Alimentos.obtener_descuento(), 0.02);
        assert_eq!(Ropa.obtener_descuento(), 0.01);
        assert_eq!(Hogar.obtener_descuento(), 0.03);
        assert_eq!(Otros.obtener_descuento(), 0.01);
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
    fn test_new_informe() {
        let fecha = fecha_dummy();
        let productos = vec![prod_vendido("Remera", Ropa, 100.0, 2)];
        let monto_total = 198.0; // 100 * 2 * 0.99
        let medio_pago = Efectivo;
        let informe = Informe::new(fecha.clone(), productos, monto_total, medio_pago.clone());
        assert_eq!(informe.fecha, fecha);
        assert_eq!(informe.monto_total, monto_total);
        assert_eq!(informe.medio_pago, medio_pago);
    }

    //Test de fechas
    fn test_fecha_dummy(d: u32, m: u32, a: i32) -> Fecha {
        Fecha::new(d, m, a).expect("Fecha inválida en test_fecha_dummy")
    }
    #[test]
    fn test_new() {
        let fecha = Fecha::new(1, 1, 2000);
        assert!(fecha.is_some());
    }
    #[test]
    fn test_new_actual() {
        let fecha = Fecha::new_fecha_actual();
        assert!(fecha.es_fecha_valida());
    }
    #[test]
    fn test_es_fecha_none() {
        let fecha = Fecha::new(31, 13, 2000);
        assert!(fecha.is_none());
    }

    #[test]
    fn test_es_fecha_valida() {
        let fecha = Fecha::new(1, 5, 2000).expect("Fecha Invalida");
        assert!(fecha.es_fecha_valida())
    }

    #[test]
    fn test_es_fecha_invalida() {
        let fecha = Fecha::new(31, 13, 2000);
        assert!(fecha.is_none());
    }

    #[test]
    fn test_es_bisiesto() {
        let fecha = Fecha::new(29, 2, 2000);
        assert!(true == fecha.unwrap().es_bisiesto());
    }

    #[test]
    fn test_no_es_bisiesto() {
        let fecha = Fecha::new(28, 2, 2001);
        assert!(false == fecha.unwrap().es_bisiesto());
    }

    #[test]
    fn test_sumar_dias() {
        let fecha = Fecha::new(29, 12, 2000).expect("Fecha inválida");
        let resultado = fecha.sumar_dias(3).expect("Error al sumar días");

        assert_eq!(resultado.mes, 1);
        assert_eq!(resultado.dia, 1);
        assert_eq!(resultado.anio, 2001);
    }

    #[test]
    fn test_restar_dias() {
        let fecha = Fecha::new(1, 1, 2000).expect("Fecha inválida");
        let resultado = fecha.restar_dias(3).expect("Error al restar días");

        assert_eq!(resultado.mes, 12);
        assert_eq!(resultado.dia, 29);
        assert_eq!(resultado.anio, 1999);
    }

    #[test]
    fn test_fecha_es_mayor() {
        let fecha1 = Fecha::new(1, 1, 2000).expect("Fecha inválida");
        let fecha2 = Fecha::new(1, 1, 2001).expect("Fecha inválida");
        assert!(false == fecha1.es_mayor(&fecha2));
        assert!(true == fecha2.es_mayor(&fecha1));
    }

    #[test]
    fn test_igual_positivo() {
        let fecha1 = test_fecha_dummy(1, 1, 2000);
        let fecha2 = test_fecha_dummy(1, 1, 2000);
        assert!(fecha1.igual(&fecha2));
    }

    #[test]
    fn test_igual_negativo() {
        let fecha1 = test_fecha_dummy(1, 1, 2000);
        let fecha2 = test_fecha_dummy(2, 1, 2000);
        assert!(!fecha1.igual(&fecha2));
    }
}
