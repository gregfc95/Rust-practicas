/*
3 -La plataforma de streaming "StreamingRust" ofrece distintos tipos de suscripciones
(Basic, Clasic, Super) a sus usuarios. Cada suscripción tiene un costo mensual y una
duración de meses y una fecha de inicio, además los usuarios pueden pagar por sus
suscripciones con distintos medios de pago que son Efectivo, MercadoPago, Tarjeta de
Crédito, Transferencia Bancaria, Cripto. Cada medio de pago tiene sus datos
correspondientes a excepción de Efectivo.
Los usuarios solo pueden tener una suscripción activa a la vez.
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones:
➢ Crear un usuario con una determinada suscripción y medio de pago.
➢ Dado un usuario hacer un upgrade sobre la suscripción. Es decir si está a Basic
pasa a Clasic y si está en Clasic pasa a Super.
➢ Dado un determinado usuario, hacer un downgrade sobre una suscripción, si la
suscripción es del tipo Basic al hacerlo se cancelará la suscripción.
➢ Dado un usuario cancelar la suscripción.
///Para Traits, compararar este objeto con otro.
///Mejor como función libre: actúa sobre una colección de datos, no sobre una instancia.
➢ Saber el medio de pago que es más utilizado por los usuarios sobre las
suscripciones activas
➢ Saber cual es la suscripción más contratada por los usuarios sobre las suscripciones
activas.
➢ Saber cuál fue el medio de pago más utilizado.
➢ Saber cuál fue la suscripción más contratada.
*/

/// Tipos de suscripción disponibles en la plataforma.
use super::Fecha;
use std::collections::HashMap;

//Enums
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscripcionTipo {
    Basic,
    Clasic,
    Super,
}
/// Trait para controlar las acciones de suscripción de un usuario.
//Esto me parece incorrecto. Fecha deberia ser manejado de otra forma, se deberia usar Chrono/naive_date
trait SubscripcionControl {
    fn upgrade(&mut self, fecha_actual: Fecha);
    fn downgrade(&mut self, fecha_actual: Fecha);
    fn cancelar(&mut self);
}

/// Representa los medios de pago posibles para una suscripción.
/// Algunos requieren datos adicionales.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MedioPagoTipo {
    /// Pago en efectivo, sin datos adicionales.
    Efectivo,
    /// Pago con MercadoPago, requiere información del titular.
    MercadoPago(BilleteraInfo),
    /// Pago con tarjeta de crédito, requiere información del titular.
    TarjetaDeCredito(TitularInfo),
    /// Transferencia bancaria, requiere datos del titular y CBU.
    TransferenciaBancaria(TransferenciaInfo),
    /// Pago con criptomonedas, requiere dirección y red.
    Cripto(CriptoInfo),
}

//Structs
/// Información común de métodos de pago con tarjeta o billetera digital.
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

/// Información para pagos por transferencia bancaria.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransferenciaInfo {
    /// Nombre del titular de la cuenta.
    titular: String,
    /// Clave Bancaria Uniforme (CBU).
    cbu: String,
    ///ID de la transferencia
    id_transferencia: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BilleteraInfo {
    /// Nombre del titular de la cuenta.
    titular: String,
    /// Clave Virtual Uniforme (CVU).
    cvu: String,
    ///ID de la transferencia
    id_transferencia: String,
}
/// Información para pagos con criptomonedas.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CriptoInfo {
    /// Dirección de la wallet.
    wallet_address: String,
    /// Red utilizada para la transacción (por ejemplo, Ethereum).
    red: String,
}
/// Representa una suscripción activa de un usuario.
#[derive(Debug, Clone, PartialEq)]
pub struct Subscripcion {
    /// Tipo de suscripción contratada.
    tipo: SubscripcionTipo,
    /// Duración de la suscripción en meses.
    duracion_meses: u32,
    /// Fecha de inicio de la suscripción.
    fecha_inicio: Fecha,
    /// Costo mensual de la suscripción.
    costo: f32,
    /// Medio de pago utilizado.
    medio_pago: MedioPagoTipo,
}
/// Representa un usuario registrado en la plataforma.
#[derive(Debug, Clone, PartialEq)]
pub struct Usuario {
    /// Nombre del usuario.
    nombre: String,
    /// Dirección de correo electrónico.
    email: String,
    /// Fecha de registro del usuario.
    fecha_creacion: Fecha,
    /// Subscripción activa del usuario, si tiene alguna.
    subscripcion_activa: Option<Subscripcion>,
    /// Subscripción historico del usuario.
    historial_suscripciones: Vec<Subscripcion>,
}

struct StreamingRust {
    usuarios: Vec<Usuario>,
}
// Implementaciones

// Implementaciones de los métodos de SubscripcionTipo
impl SubscripcionTipo {
    fn costo(&self) -> f32 {
        match self {
            SubscripcionTipo::Basic => 10.0,
            SubscripcionTipo::Clasic => 20.0,
            SubscripcionTipo::Super => 30.0,
        }
    }
    fn nivel(&self) -> u8 {
        match self {
            SubscripcionTipo::Basic => 1,
            SubscripcionTipo::Clasic => 2,
            SubscripcionTipo::Super => 3,
        }
    }
    fn duracion(&self) -> u32 {
        match self {
            SubscripcionTipo::Basic => 1,
            SubscripcionTipo::Clasic => 6,
            SubscripcionTipo::Super => 12,
        }
    }
    fn upgrade(&self) -> Option<SubscripcionTipo> {
        match self {
            SubscripcionTipo::Basic => Some(SubscripcionTipo::Clasic),
            SubscripcionTipo::Clasic => Some(SubscripcionTipo::Super),
            SubscripcionTipo::Super => None,
        }
    }
    fn downgrade(&self) -> Option<SubscripcionTipo> {
        match self {
            SubscripcionTipo::Basic => None,
            SubscripcionTipo::Clasic => Some(SubscripcionTipo::Basic),
            SubscripcionTipo::Super => Some(SubscripcionTipo::Clasic),
        }
    }
}

// Implementaciones de los métodos de MedioPagoTipo
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

/// Implementaciones de los métodos de TransferenciaInfo, BilleteraInfo y CriptoInfo
impl TransferenciaInfo {
    pub fn new(titular: String, cbu: String, id_transferencia: String) -> Self {
        TransferenciaInfo {
            titular,
            cbu,
            id_transferencia,
        }
    }
}

impl BilleteraInfo {
    pub fn new(titular: String, cvu: String, id_transferencia: String) -> Self {
        BilleteraInfo {
            titular,
            cvu,
            id_transferencia,
        }
    }
}

impl CriptoInfo {
    pub fn new(wallet_address: String, red: String) -> Self {
        CriptoInfo {
            wallet_address,
            red,
        }
    }
}

// Implementaciones de Subscripcion
impl Subscripcion {
    /// Crea una nueva instancia de Subscripcion
    fn new(tipo: SubscripcionTipo, medio_pago: MedioPagoTipo, fecha_inicio: Fecha) -> Self {
        let costo = tipo.costo();
        let duracion = tipo.duracion();
        Subscripcion {
            tipo: tipo,
            duracion_meses: duracion,
            fecha_inicio: fecha_inicio,
            costo,
            medio_pago: medio_pago,
        }
    }
}
// Implementaciones de SubscripcionControl para Usuario
// Este trait permite a los usuarios realizar acciones sobre sus suscripciones.
// Implementa las acciones de upgrade, downgrade y cancelar suscripción.
impl SubscripcionControl for Usuario {
    fn upgrade(&mut self, fecha_actual: Fecha) {
        if let Some(ref mut subscripcion) = self.subscripcion_activa {
            if let Some(next_subscripcion) = subscripcion.tipo.upgrade() {
                let anterior = subscripcion.clone();
                self.historial_suscripciones.push(anterior);
                let nuevo_pago = subscripcion.medio_pago.clone();
                *subscripcion = Subscripcion::new(next_subscripcion, nuevo_pago, fecha_actual);
            }
        }
    }

    fn downgrade(&mut self, fecha_actual: Fecha) {
        if let Some(ref mut subscripcion) = self.subscripcion_activa {
            if let Some(next_subscripcion) = subscripcion.tipo.downgrade() {
                let anterior = subscripcion.clone();
                self.historial_suscripciones.push(anterior);
                let nuevo_pago = subscripcion.medio_pago.clone();
                *subscripcion = Subscripcion::new(next_subscripcion, nuevo_pago, fecha_actual);
            }
        }
    }
    fn cancelar(&mut self) {
        // Cancela la suscripción activa del usuario, si tiene alguna, take() guarda el valor antes de setear None
        if let Some(subscripcion) = self.subscripcion_activa.take() {
            self.historial_suscripciones.push(subscripcion);
        }
    }
}

// Implementaciones de Usuario
impl Usuario {
    fn new(
        nombre: String,
        email: String,
        subscripcion_activa: Subscripcion,
        fecha_creation: Fecha,
    ) -> Self {
        Usuario {
            nombre,
            email,
            fecha_creacion: fecha_creation,
            subscripcion_activa: Some(subscripcion_activa.clone()),
            historial_suscripciones: vec![subscripcion_activa],
        }
    }
}

pub fn mediopago_activo_mas_utilizado(usuarios: Vec<Usuario>) -> Option<MedioPagoTipo> {
    //Aplano para poder contarlas.
    let todas_las_subs: Vec<MedioPagoTipo> = usuarios
        .iter()
        .flat_map(|usuario| usuario.subscripcion_activa.clone())
        .map(|subs| subs.medio_pago.clone())
        .collect();

    let mut conteo: HashMap<MedioPagoTipo, usize> = HashMap::new();
    for medio in todas_las_subs {
        *conteo.entry(medio.clone()).or_insert(0) += 1;
    }
    conteo
        .into_iter()
        .max_by_key(|&(_, cantidad)| cantidad)
        .map(|(medio, _)| medio)
}

pub fn suscripcion_activo_mas_utilizado(usuarios: Vec<Usuario>) -> Option<SubscripcionTipo> {
    //Aplano las suscripciones, para poder contarlas.
    let todas_las_subs: Vec<SubscripcionTipo> = usuarios
        .iter()
        .flat_map(|usuario| usuario.subscripcion_activa.clone())
        .map(|subs| subs.tipo.clone())
        .collect();

    let mut conteo: HashMap<SubscripcionTipo, usize> = HashMap::new();
    for subs in todas_las_subs {
        *conteo.entry(subs.clone()).or_insert(0) += 1;
    }

    conteo
        .into_iter()
        .max_by_key(|&(_, cantidad)| cantidad)
        .map(|(tipo, _)| tipo)
}

pub fn subscripcion_mas_contratada(usuarios: Vec<Usuario>) -> Option<SubscripcionTipo> {
    //Aplano las suscripciones, para poder contarlas.
    let todas_las_subs: Vec<Subscripcion> = usuarios
        .iter()
        .flat_map(|usuario| usuario.historial_suscripciones.clone())
        .collect();

    let mut conteo: HashMap<SubscripcionTipo, usize> = HashMap::new();
    for subs in todas_las_subs {
        *conteo.entry(subs.tipo.clone()).or_insert(0) += 1;
    }
    conteo
        .into_iter()
        .max_by_key(|&(_, cantidad)| cantidad)
        .map(|(tipo, _)| tipo)
}

pub fn medio_mas_utilizado(usuarios: Vec<Usuario>) -> Option<MedioPagoTipo> {
    let todos_los_medios_pago: Vec<MedioPagoTipo> = usuarios
        .iter()
        .flat_map(|usuario| {
            usuario
                .historial_suscripciones
                .iter()
                .map(|s| s.medio_pago.clone())
        })
        .collect();

    let mut conteo: HashMap<MedioPagoTipo, usize> = HashMap::new();
    for medio in todos_los_medios_pago {
        *conteo.entry(medio.clone()).or_insert(0) += 1;
    }
    conteo
        .into_iter()
        .max_by_key(|&(_, cantidad)| cantidad)
        .map(|(medio, _)| medio)
}
