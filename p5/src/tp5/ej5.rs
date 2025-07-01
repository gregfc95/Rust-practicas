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

5- En base al ejercicio 3 del tp#4 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Todas las suscripciones deben almacenarse en un archivo en formato JSON,
implemente lo necesario para que toda la funcionalidad de las suscripciones se realice
guardando, leyendo o modificando archivos.No debe modificar los tests hechos en el punto
a. Si puede agregar más en caso de que haga métodos nuevos para cumplir con este
punto. Recuerde también que se debe seguir manteniendo un coverage de al menos 90%
*/

/// Tipos de suscripción disponibles en la plataforma.
use super::Fecha;
use serde::de;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write};
//Enums
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubscripcionTipo {
    Basic,
    Clasic,
    Super,
}
#[derive(Debug)]
enum ErrorSubscripcion {
    UpgradeNoDisponible,
    DowngradeNoDisponible,
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
/// Trait para controlar las acciones de suscripción de un usuario.
//Esto me parece incorrecto. Fecha deberia ser manejado de otra forma, se deberia usar Chrono/naive_date
trait SubscripcionControl {
    fn upgrade(&mut self, fecha_actual: Fecha) -> Result<(), ErrorSubscripcion>;
    fn downgrade(&mut self, fecha_actual: Fecha) -> Result<(), ErrorSubscripcion>;
    fn cancelar(&mut self);
}

/// Representa los medios de pago posibles para una suscripción.
/// Algunos requieren datos adicionales.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TransferenciaInfo {
    /// Nombre del titular de la cuenta.
    titular: String,
    /// Clave Bancaria Uniforme (CBU).
    cbu: String,
    ///ID de la transferencia
    id_transferencia: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct BilleteraInfo {
    /// Nombre del titular de la cuenta.
    titular: String,
    /// Clave Virtual Uniforme (CVU).
    cvu: String,
    ///ID de la transferencia
    id_transferencia: String,
}
/// Información para pagos con criptomonedas.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CriptoInfo {
    /// Dirección de la wallet.
    wallet_address: String,
    /// Red utilizada para la transacción (por ejemplo, Ethereum).
    red: String,
}
/// Representa una suscripción activa de un usuario.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
// Implementa las acciones de upgrade, downgrade e cancelar suscripción.
impl SubscripcionControl for Usuario {
    fn upgrade(&mut self, fecha_actual: Fecha) -> Result<(), ErrorSubscripcion> {
        if let Some(ref mut subscripcion) = self.subscripcion_activa {
            if let Some(next_tipo) = subscripcion.tipo.upgrade() {
                let anterior = subscripcion.clone();
                self.historial_suscripciones.push(anterior);
                let medio_pago = subscripcion.medio_pago.clone();
                *subscripcion = Subscripcion::new(next_tipo, medio_pago, fecha_actual);
                Ok(())
            } else {
                Err(ErrorSubscripcion::UpgradeNoDisponible)
            }
        } else {
            Err(ErrorSubscripcion::UpgradeNoDisponible)
        }
    }

    fn downgrade(&mut self, fecha_actual: Fecha) -> Result<(), ErrorSubscripcion> {
        if let Some(ref mut subscripcion) = self.subscripcion_activa {
            if let Some(prev_tipo) = subscripcion.tipo.downgrade() {
                let anterior = subscripcion.clone();
                self.historial_suscripciones.push(anterior);
                let medio_pago = subscripcion.medio_pago.clone();
                *subscripcion = Subscripcion::new(prev_tipo, medio_pago, fecha_actual);
                Ok(())
            } else {
                Err(ErrorSubscripcion::DowngradeNoDisponible)
            }
        } else {
            Err(ErrorSubscripcion::DowngradeNoDisponible)
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
impl StreamingRust {
    fn new() -> Self {
        StreamingRust {
            usuarios: Vec::new(),
        }
    }
    /// Agrega un nuevo usuario a la plataforma.
    fn agregar_usuario(&mut self, usuario: Usuario) {
        self.usuarios.push(usuario);
    }
    /// medio de pago más utilizado por los usuarios sobre las suscripciones activas.
    fn medio_pago_activo_mas_utilizado(&self) -> Option<MedioPagoTipo> {
        let mut conteo: HashMap<MedioPagoTipo, usize> = HashMap::new();
        for usuario in &self.usuarios {
            if let Some(subs) = &usuario.subscripcion_activa {
                *conteo.entry(subs.medio_pago.clone()).or_insert(0) += 1;
            }
        }
        conteo
            .into_iter()
            .max_by_key(|&(_, cantidad)| cantidad)
            .map(|(medio, _)| medio)
    }
    /// Suscripción más utilizada por los usuarios sobre las suscripciones activas.
    fn subscripcion_activo_mas_utilizado(&self) -> Option<SubscripcionTipo> {
        let mut conteo: HashMap<SubscripcionTipo, usize> = HashMap::new();
        for usuario in &self.usuarios {
            if let Some(subs) = &usuario.subscripcion_activa {
                *conteo.entry(subs.tipo.clone()).or_insert(0) += 1;
            }
        }
        conteo
            .into_iter()
            .max_by_key(|&(_, cantidad)| cantidad)
            .map(|(tipo, _)| tipo)
    }

    /// Suscripción más contratada por los usuarios sobre el historial de suscripciones.
    fn subscripcion_mas_contratada(&self) -> Option<SubscripcionTipo> {
        let mut conteo: HashMap<SubscripcionTipo, usize> = HashMap::new();
        for usuario in &self.usuarios {
            for subs in &usuario.historial_suscripciones {
                *conteo.entry(subs.tipo.clone()).or_insert(0) += 1;
            }
        }
        conteo
            .into_iter()
            .max_by_key(|&(_, cantidad)| cantidad)
            .map(|(tipo, _)| tipo)
    }

    /// Medio de pago más utilizado por los usuarios sobre el historial de suscripciones.
    fn medio_pago_mas_utilizado(&self) -> Option<MedioPagoTipo> {
        let mut conteo: HashMap<MedioPagoTipo, usize> = HashMap::new();
        for usuario in &self.usuarios {
            for subs in &usuario.historial_suscripciones {
                *conteo.entry(subs.medio_pago.clone()).or_insert(0) += 1;
            }
        }
        conteo
            .into_iter()
            .max_by_key(|&(_, cantidad)| cantidad)
            .map(|(medio, _)| medio)
    }
    //JSON funciones
    /// Guarda todos los usuarios y sus suscripciones en un archivo JSON.
    pub fn guardar_en_archivo(&self, path: &str) -> Result<(), ResultadoOperacion> {
        let file =
            File::create(path).map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?;
        serde_json::to_writer_pretty(file, &self.usuarios)
            .map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))
    }

    /// Carga todos los usuarios y sus suscripciones desde un archivo JSON.
    pub fn cargar_de_archivo(&mut self, path: &str) -> Result<(), ResultadoOperacion> {
        let mut file =
            File::open(path).map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?;
        let mut contenido = String::new();
        file.read_to_string(&mut contenido)
            .map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?;
        let usuarios: Vec<Usuario> = serde_json::from_str(&contenido)
            .map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?;
        self.usuarios = usuarios;
        Ok(())
    }

    /// Agrega un usuario y guarda automáticamente en archivo.
    pub fn agregar_usuario_y_guardar(
        &mut self,
        usuario: Usuario,
        path: &str,
    ) -> Result<ResultadoOperacion, ResultadoOperacion> {
        self.cargar_de_archivo(path).ok(); // Ignora error si el archivo no existe
        self.agregar_usuario(usuario);
        self.guardar_en_archivo(path)?;
        Ok(ResultadoOperacion::Exito)
    }

    /// Realiza un upgrade de suscripción para un usuario y guarda en archivo.
    pub fn upgrade_usuario_y_guardar(
        &mut self,
        email: &str,
        fecha_actual: Fecha,
        path: &str,
    ) -> Result<ResultadoOperacion, ResultadoOperacion> {
        self.cargar_de_archivo(path).ok();
        if let Some(usuario) = self.usuarios.iter_mut().find(|u| u.email == email) {
            match usuario.upgrade(fecha_actual) {
                Ok(_) => {
                    self.guardar_en_archivo(path)?;
                    Ok(ResultadoOperacion::Exito)
                }
                Err(e) => Err(ResultadoOperacion::Fallo(format!("{:?}", e))),
            }
        } else {
            Err(ResultadoOperacion::Fallo(
                "Usuario no encontrado".to_string(),
            ))
        }
    }

    /// Realiza un downgrade de suscripción para un usuario y guarda en archivo.
    pub fn downgrade_usuario_y_guardar(
        &mut self,
        email: &str,
        fecha_actual: Fecha,
        path: &str,
    ) -> Result<ResultadoOperacion, ResultadoOperacion> {
        self.cargar_de_archivo(path).ok();
        if let Some(usuario) = self.usuarios.iter_mut().find(|u| u.email == email) {
            match usuario.downgrade(fecha_actual) {
                Ok(_) => {
                    self.guardar_en_archivo(path)?;
                    Ok(ResultadoOperacion::Exito)
                }
                Err(e) => Err(ResultadoOperacion::Fallo(format!("{:?}", e))),
            }
        } else {
            Err(ResultadoOperacion::Fallo(
                "Usuario no encontrado".to_string(),
            ))
        }
    }

    /// Cancela la suscripción de un usuario y guarda en archivo.
    pub fn cancelar_usuario_y_guardar(
        &mut self,
        email: &str,
        path: &str,
    ) -> Result<ResultadoOperacion, ResultadoOperacion> {
        self.cargar_de_archivo(path).ok();
        if let Some(usuario) = self.usuarios.iter_mut().find(|u| u.email == email) {
            usuario.cancelar();
            self.guardar_en_archivo(path)?;
            Ok(ResultadoOperacion::Exito)
        } else {
            Err(ResultadoOperacion::Fallo(
                "Usuario no encontrado".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MedioPagoTipo::*;
    use super::SubscripcionTipo::*;
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
    fn billetera_info() -> BilleteraInfo {
        BilleteraInfo::new("Maria".to_string(), "CVU123".to_string(), "ID1".to_string())
    }
    fn cripto_info() -> CriptoInfo {
        CriptoInfo::new("WALLET1".to_string(), "ETH".to_string())
    }
    fn transferencia_info() -> TransferenciaInfo {
        TransferenciaInfo::new("Pedro".to_string(), "CBU123".to_string(), "ID2".to_string())
    }

    fn sub_basic() -> Subscripcion {
        Subscripcion::new(Basic, Efectivo, fecha_dummy())
    }
    fn sub_clasic() -> Subscripcion {
        Subscripcion::new(Clasic, MercadoPago(billetera_info()), fecha_dummy())
    }
    fn sub_super() -> Subscripcion {
        Subscripcion::new(Super, TarjetaDeCredito(titular_info()), fecha_dummy())
    }

    fn usuario_basic() -> Usuario {
        Usuario::new(
            "Juan".to_string(),
            "juan@mail.com".to_string(),
            sub_basic(),
            fecha_dummy(),
        )
    }
    fn usuario_clasic() -> Usuario {
        Usuario::new(
            "Maria".to_string(),
            "maria@mail.com".to_string(),
            sub_clasic(),
            fecha_dummy(),
        )
    }
    fn usuario_super() -> Usuario {
        Usuario::new(
            "Pedro".to_string(),
            "pedro@mail.com".to_string(),
            sub_super(),
            fecha_dummy(),
        )
    }

    #[test]
    fn test_upgrade_de_basic_a_clasic() {
        let mut user = usuario_basic();
        let result = user.upgrade(fecha_dummy());
        assert!(result.is_ok(),);

        let subs_activa = user
            .subscripcion_activa
            .as_ref()
            .expect("Debería haber una subscripción activa");
        assert_eq!(subs_activa.tipo, Clasic);
    }
    #[test]
    fn test_upgrade_de_clasic_a_super() {
        let mut user = usuario_clasic();
        let result = user.upgrade(fecha_dummy());
        assert!(result.is_ok(),);

        let subs_activa = user
            .subscripcion_activa
            .as_ref()
            .expect("Debería haber una subscripción activa");
        assert_eq!(subs_activa.tipo, Super);
    }
    #[test]
    fn test_upgrade_de_super_falla() {
        let mut user = usuario_super();
        let result = user.upgrade(fecha_dummy());

        assert!(matches!(
            result,
            Err(ErrorSubscripcion::UpgradeNoDisponible)
        ));

        let subs_activa = user
            .subscripcion_activa
            .as_ref()
            .expect("Debería seguir habiendo una subscripción activa");
        assert_eq!(
            subs_activa.tipo, Super,
            "El tipo de subscripción debería seguir siendo Super"
        );
    }

    #[test]
    fn test_downgrade_de_super_a_clasic() {
        let mut user = usuario_super();
        let result = user.downgrade(fecha_dummy());
        assert!(
            result.is_ok(),
            "El downgrade de Super a Clasic debería ser válido"
        );

        let subs_activa = user
            .subscripcion_activa
            .as_ref()
            .expect("Debería haber una subscripción activa");
        assert_eq!(subs_activa.tipo, Clasic);
    }
    #[test]
    fn test_downgrade_de_clasic_a_basic() {
        let mut user = usuario_clasic();
        let result = user.downgrade(fecha_dummy());
        assert!(
            result.is_ok(),
            "El downgrade de Clasic a Basic debería ser válido"
        );

        let subs_activa = user
            .subscripcion_activa
            .as_ref()
            .expect("Debería haber una subscripción activa");
        assert_eq!(subs_activa.tipo, Basic);
    }

    #[test]
    fn test_downgrade_de_basic_falla() {
        let mut user = usuario_basic();
        let result = user.downgrade(fecha_dummy());

        assert!(
            matches!(result, Err(ErrorSubscripcion::DowngradeNoDisponible)),
            "El downgrade desde Basic debería fallar con el error adecuado"
        );

        let subs_activa = user
            .subscripcion_activa
            .as_ref()
            .expect("Deberia seguir habiendo una subscripcion activa");
        assert_eq!(
            subs_activa.tipo, Basic,
            "El tipo de subscripcion deberia seguir siendo Basic"
        );
    }

    #[test]
    fn test_cancelar_con_subscripcion_activa() {
        let mut user = usuario_basic();
        let cantidad_historial_antes = user.historial_suscripciones.len();

        user.cancelar();

        assert!(
            user.subscripcion_activa.is_none(),
            "La suscripcion activa deberia haber sido eliminada"
        );
        assert_eq!(
            user.historial_suscripciones.len(),
            cantidad_historial_antes + 1,
            "La suscripcion cancelada deberia haberse agregado al historial"
        );
    }

    #[test]
    fn test_cancelar_sin_subscripcion_activa_no_paniquear() {
        let mut user = usuario_basic();
        user.cancelar(); // Primera vez: debería cancelarla
        let historial_despues_1 = user.historial_suscripciones.len();

        user.cancelar(); // Segunda vez: no debería hacer nada

        assert!(
            user.subscripcion_activa.is_none(),
            "No deberia haber una suscripcion activa después de cancelar dos veces"
        );
        assert_eq!(
            user.historial_suscripciones.len(),
            historial_despues_1,
            "El historial no debería haberse modificado al intentar cancelar sin una suscripción activa"
        );
    }

    #[test]
    fn test_medio_pago_activo_mas_utilizado() {
        let mut streaming = StreamingRust::new();
        streaming.agregar_usuario(usuario_clasic()); // MercadoPago
        streaming.agregar_usuario(usuario_clasic()); // MercadoPago
        streaming.agregar_usuario(usuario_super()); // Tarjeta

        let resultado = streaming.medio_pago_activo_mas_utilizado();
        assert_eq!(
            resultado,
            Some(MedioPagoTipo::MercadoPago(billetera_info())),
            "MercadoPago debería ser el medio de pago activo más utilizado"
        );
    }

    #[test]
    fn test_subscripcion_activa_mas_utilizada() {
        let mut streaming = StreamingRust::new();
        streaming.agregar_usuario(usuario_basic()); // Basic
        streaming.agregar_usuario(usuario_basic()); // Basic
        streaming.agregar_usuario(usuario_super()); // Super

        let resultado = streaming.subscripcion_activo_mas_utilizado();
        assert_eq!(
            resultado,
            Some(SubscripcionTipo::Basic),
            "Basic debería ser la suscripción activa más utilizada"
        );
    }

    #[test]
    fn test_medio_pago_mas_utilizado_en_historial() {
        let mut user = usuario_clasic(); // MercadoPago
        user.cancelar(); // Agrega al historial
        user.cancelar(); // No cambia nada
        let mut streaming = StreamingRust::new();
        streaming.agregar_usuario(user);

        let resultado = streaming.medio_pago_mas_utilizado();
        assert_eq!(
            resultado,
            Some(MedioPagoTipo::MercadoPago(billetera_info())),
            "MercadoPago debería ser el medio de pago más usado en el historial"
        );
    }

    #[test]
    fn test_subscripcion_mas_contratada_en_historial() {
        let mut user = usuario_super(); // Super
        user.downgrade(fecha_dummy()); // Super → Clasic
        user.downgrade(fecha_dummy()); // Clasic → Basic
        user.cancelar(); // Basic al historial

        let mut streaming = StreamingRust::new();
        streaming.agregar_usuario(user);

        let resultado = streaming.subscripcion_mas_contratada();
        assert_eq!(
            resultado,
            Some(SubscripcionTipo::Super),
            "Super debería ser la suscripción más contratada en historial, ya que fue la inicial"
        );
    }

    #[test]
    fn test_guardar_en_archivo_exito() {
        use std::fs;
        let mut streaming = StreamingRust::new();
        streaming.agregar_usuario(usuario_basic());
        streaming.agregar_usuario(usuario_clasic());
        let path = "test_streaming_guardar.json";
        let res = streaming.guardar_en_archivo(path);
        assert!(res.is_ok());
        let contenido = fs::read_to_string(path).unwrap();
        assert!(contenido.contains("juan@mail.com"));
        assert!(contenido.contains("maria@mail.com"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_guardar_en_archivo_error() {
        let mut streaming = StreamingRust::new();
        streaming.agregar_usuario(usuario_basic());
        // Usamos un path inválido para forzar el error
        let res = streaming.guardar_en_archivo("/no_existe/test_streaming_guardar.json");
        match res {
            Err(ResultadoOperacion::ErrorArchivo(_)) => assert!(true),
            _ => panic!("Se esperaba un error de archivo"),
        }
    }

    #[test]
    fn test_cargar_de_archivo_exito() {
        use std::fs;
        let mut streaming = StreamingRust::new();
        streaming.agregar_usuario(usuario_basic());
        streaming.agregar_usuario(usuario_clasic());
        let path = "test_streaming_cargar.json";
        streaming.guardar_en_archivo(path).unwrap();
        // Ahora cargamos en una instancia vacía
        let mut streaming2 = StreamingRust::new();
        let res = streaming2.cargar_de_archivo(path);
        assert!(res.is_ok());
        assert_eq!(streaming2.usuarios.len(), 2);
        assert!(
            streaming2
                .usuarios
                .iter()
                .any(|u| u.email == "juan@mail.com")
        );
        assert!(
            streaming2
                .usuarios
                .iter()
                .any(|u| u.email == "maria@mail.com")
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_cargar_de_archivo_error() {
        let mut streaming = StreamingRust::new();
        // Usamos un path inválido para forzar el error
        let res = streaming.cargar_de_archivo("/no_existe/test_streaming_cargar.json");
        match res {
            Err(ResultadoOperacion::ErrorArchivo(_)) => assert!(true),
            _ => panic!("Se esperaba un error de archivo"),
        }
    }

    #[test]
    fn test_agregar_usuario_y_guardar_exito() {
        use std::fs;
        let path = "test_streaming_agregar_usuario.json";
        let mut streaming = StreamingRust::new();
        let res = streaming.agregar_usuario_y_guardar(usuario_basic(), path);
        assert!(matches!(res, Ok(ResultadoOperacion::Exito)));
        // Verifica que el archivo contiene el usuario
        let contenido = fs::read_to_string(path).unwrap();
        assert!(contenido.contains("juan@mail.com"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_agregar_usuario_y_guardar_error() {
        let mut streaming = StreamingRust::new();
        // Usamos un path inválido para forzar el error
        let res = streaming.agregar_usuario_y_guardar(
            usuario_basic(),
            "/no_existe/test_streaming_agregar_usuario.json",
        );
        match res {
            Err(ResultadoOperacion::ErrorArchivo(_)) => assert!(true),
            _ => panic!("Se esperaba un error de archivo"),
        }
    }

    #[test]
    fn test_upgrade_usuario_y_guardar_exito() {
        use std::fs;
        let path = "test_streaming_upgrade_usuario.json";
        // Guardamos un usuario Basic
        {
            let mut streaming = StreamingRust::new();
            streaming.agregar_usuario(usuario_basic());
            streaming.guardar_en_archivo(path).unwrap();
        }
        // Ahora hacemos upgrade y guardamos
        let mut streaming2 = StreamingRust::new();
        let res = streaming2.upgrade_usuario_y_guardar("juan@mail.com", fecha_dummy(), path);
        assert!(matches!(res, Ok(ResultadoOperacion::Exito)));
        // Verifica que el usuario ahora es Clasic
        let mut streaming3 = StreamingRust::new();
        streaming3.cargar_de_archivo(path).unwrap();
        let user = streaming3
            .usuarios
            .iter()
            .find(|u| u.email == "juan@mail.com")
            .unwrap();
        assert_eq!(
            user.subscripcion_activa.as_ref().unwrap().tipo,
            SubscripcionTipo::Clasic
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_upgrade_usuario_y_guardar_usuario_no_encontrado() {
        let path = "test_streaming_upgrade_usuario_no_user.json";
        let mut streaming = StreamingRust::new();
        streaming.guardar_en_archivo(path).unwrap();
        let res = streaming.upgrade_usuario_y_guardar("noexiste@mail.com", fecha_dummy(), path);
        match res {
            Err(ResultadoOperacion::Fallo(msg)) => assert!(msg.contains("Usuario no encontrado")),
            _ => panic!("Se esperaba error de usuario no encontrado"),
        }
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_upgrade_usuario_y_guardar_upgrade_no_disponible() {
        use std::fs;
        let path = "test_streaming_upgrade_usuario_no_disponible.json";
        // Guardamos un usuario Super (no puede hacer upgrade)
        {
            let mut streaming = StreamingRust::new();
            streaming.agregar_usuario(usuario_super());
            streaming.guardar_en_archivo(path).unwrap();
        }
        let mut streaming2 = StreamingRust::new();
        let res = streaming2.upgrade_usuario_y_guardar("pedro@mail.com", fecha_dummy(), path);
        match res {
            Err(ResultadoOperacion::Fallo(msg)) => assert!(msg.contains("UpgradeNoDisponible")),
            _ => panic!("Se esperaba error UpgradeNoDisponible"),
        }
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_downgrade_usuario_y_guardar_exito() {
        use std::fs;
        let path = "test_streaming_downgrade_usuario.json";
        // Guardamos un usuario Clasic
        {
            let mut streaming = StreamingRust::new();
            streaming.agregar_usuario(usuario_clasic());
            streaming.guardar_en_archivo(path).unwrap();
        }
        // Ahora hacemos downgrade y guardamos
        let mut streaming2 = StreamingRust::new();
        let res = streaming2.downgrade_usuario_y_guardar("maria@mail.com", fecha_dummy(), path);
        assert!(matches!(res, Ok(ResultadoOperacion::Exito)));
        // Verifica que el usuario ahora es Basic
        let mut streaming3 = StreamingRust::new();
        streaming3.cargar_de_archivo(path).unwrap();
        let user = streaming3
            .usuarios
            .iter()
            .find(|u| u.email == "maria@mail.com")
            .unwrap();
        assert_eq!(
            user.subscripcion_activa.as_ref().unwrap().tipo,
            SubscripcionTipo::Basic
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_downgrade_usuario_y_guardar_usuario_no_encontrado() {
        let path = "test_streaming_downgrade_usuario_no_user.json";
        let mut streaming = StreamingRust::new();
        streaming.guardar_en_archivo(path).unwrap();
        let res = streaming.downgrade_usuario_y_guardar("noexiste@mail.com", fecha_dummy(), path);
        match res {
            Err(ResultadoOperacion::Fallo(msg)) => assert!(msg.contains("Usuario no encontrado")),
            _ => panic!("Se esperaba error de usuario no encontrado"),
        }
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_downgrade_usuario_y_guardar_downgrade_no_disponible() {
        use std::fs;
        let path = "test_streaming_downgrade_usuario_no_disponible.json";
        // Guardamos un usuario Basic (no puede hacer downgrade)
        {
            let mut streaming = StreamingRust::new();
            streaming.agregar_usuario(usuario_basic());
            streaming.guardar_en_archivo(path).unwrap();
        }
        let mut streaming2 = StreamingRust::new();
        let res = streaming2.downgrade_usuario_y_guardar("juan@mail.com", fecha_dummy(), path);
        match res {
            Err(ResultadoOperacion::Fallo(msg)) => assert!(msg.contains("DowngradeNoDisponible")),
            _ => panic!("Se esperaba error DowngradeNoDisponible"),
        }
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_cancelar_usuario_y_guardar_exito() {
        use std::fs;
        let path = "test_streaming_cancelar_usuario.json";
        // Guardamos un usuario Clasic
        {
            let mut streaming = StreamingRust::new();
            streaming.agregar_usuario(usuario_clasic());
            streaming.guardar_en_archivo(path).unwrap();
        }
        // Ahora cancelamos la suscripción y guardamos
        let mut streaming2 = StreamingRust::new();
        let res = streaming2.cancelar_usuario_y_guardar("maria@mail.com", path);
        assert!(matches!(res, Ok(ResultadoOperacion::Exito)));
        // Verifica que el usuario ahora no tiene suscripción activa
        let mut streaming3 = StreamingRust::new();
        streaming3.cargar_de_archivo(path).unwrap();
        let user = streaming3
            .usuarios
            .iter()
            .find(|u| u.email == "maria@mail.com")
            .unwrap();
        assert!(user.subscripcion_activa.is_none());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_cancelar_usuario_y_guardar_usuario_no_encontrado() {
        let path = "test_streaming_cancelar_usuario_no_user.json";
        let mut streaming = StreamingRust::new();
        streaming.guardar_en_archivo(path).unwrap();
        let res = streaming.cancelar_usuario_y_guardar("noexiste@mail.com", path);
        match res {
            Err(ResultadoOperacion::Fallo(msg)) => assert!(msg.contains("Usuario no encontrado")),
            _ => panic!("Se esperaba error de usuario no encontrado"),
        }
        let _ = std::fs::remove_file(path);
    }
}
