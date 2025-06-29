/*
tarpaulin -- coverage
serde -- Archivos


5- La empresa XYZ es una plataforma de intercambio de criptoactivos que permite a los
usuarios comprar y vender distintas criptomonedas. La plataforma permite el registro de
usuarios y la gestión de sus balances en distintas criptomonedas y en dinero fíat.
De los usuarios se conoce nombre, apellido, email, dni, y si está validada su identidad o no. Cada
usuario tiene un balance de las criptomonedas que se ofrecen en la plataforma. De las
criptomonedas se conoce: nombre, prefijo y un listado de blockchains donde se pueden
enviar o recibir.

De cada blockchain se conoce el nombre, prefijo.
Implemente las estructuras, funciones asociadas y traits necesarios para resolver las
siguientes acciones relacionadas al usuario:

*➢ Ingresar dinero: se recibe un monto en fiat de un usuario y se acredita al balance de fiat de dicho usuario.
Además se crea una transacción del hecho donde los datos
que se guardan son:fecha, tipo(ingreso de dinero), monto, usuario.

*➢ Comprar determinada criptomoneda: dado un monto de fiat se compra una cantidad
de determinada criptomoneda, tenga en cuenta que al momento de realizar la
operación se obtiene del sistema la cotización actual de la criptomoneda para
acreditar la correspondiente proporción en el balance de la cripto y desacreditar en
el balance de fiat. Luego de ello se registra la transacción con los siguientes datos:
fecha, usuario, criptomoneda, tipo: compra de cripto, monto de cripto y cotización.

*➢ Vender determinada criptomoneda: dado un monto de cripto se vende por fiat, tenga
en cuenta que al momento de realizar la operación se obtiene del sistema la
cotización actual de la criptomoneda para acreditar la correspondiente proporción en
el balance de fiat y desacreditar en el balance de la criptomoneda. Luego de ello se
registra la transacción con los siguientes datos: fecha, usuario, criptomoneda, tipo:
venta de cripto,monto de cripto y cotización.

*➢ Retirar criptomoneda a blockchain: dado un monto de una cripto y una blockchain se
le descuenta del balance de dicha cripto al usuario el monto, la blockchain devuelve
un hash que representa una transacción en ella (esto hágalo retornando el nombre
de la blockchain + un número random). Luego se genera una transacción con los
siguientes datos: fecha, usuario, tipo: retiro cripto, blockchain, hash, cripto, monto,
cotización

➢ Recibir criptomoneda de blockchain: dado un monto de una cripto y una blockchain
se le acredita al balance de dicha cripto al usuario el monto. Luego se genera una
transacción con los siguientes datos: fecha, usuario, tipo: recepción cripto,
blockchain, cripto, monto, cotización.

*➢ Retirar fiat por determinado medio: dado un monto de fiat se le descuenta dicho
monto del balance al usuario y se genera una transacción con la siguiente
información: fecha, usuario, tipo: retiro fiat, monto y medio (puede ser MercadoPago
o Transferencia Bancaria)

Nota:: Tanto para comprar. vender, retirar el usuario debe estar validado.
Se debe validar siempre que haya balance suficiente para realizar la operación en los casos
de compra, venta, retiro.
Además la empresa desea saber lo siguiente en base a sus operaciones:
➢ Saber cual es la criptomoneda que más cantidad de ventas tiene
➢ Saber cual es la criptomoneda que más cantidad de compras tiene
➢ Saber cual es la criptomoneda que más volumen de ventas tiene
➢ Saber cual es la criptomoneda que más volumen de compras tiene
*/
//uses
use crate::tp4::{Fecha, fecha};
use rand;
use std::{cmp::Ordering, collections::HashMap};
// Enums
#[derive(Clone, PartialEq, Debug)]
enum MedioPago {
    MercadoPago,
    TransferenciaBancaria,
}
#[derive(PartialEq, Debug, Clone)]
enum TipoTransaccion {
    CompraCripto,
    VentaCripto,
    RetiroCripto,
    RecepcionCripto,
    IngresoDinero,
    RetiroFiat,
}

#[derive(Clone, PartialEq, Debug)]
enum PlataformaError {
    UsuarioNoEncontrado(u32),
    UsuarioNoValidado(String),
    MontoInvalido(f64),
    CotizacionNoEncontrada(String),
    FondoInsuficiente(f64),
    CriptoNoEncontrada(String),
    BlockchainNoEncontrada(String),
}

//Structs
#[derive(Clone, PartialEq, Debug)]
struct Transaccion {
    fecha: Fecha,
    tipo: TipoTransaccion,
    monto: f64,
    usuario_id: u32,
}
#[derive(Clone, PartialEq, Debug)]
struct TransaccionFiat {
    transaccion: Transaccion,
    medio_pago: MedioPago,
}

#[derive(Clone, PartialEq, Debug)]
struct TransaccionCripto {
    transaccion: Transaccion,
    criptomoneda: Criptomoneda,
    cotizacion: Option<f64>,
    hash: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blockchain {
    nombre: String,
    prefijo: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Criptomoneda {
    nombre: String,
    prefijo: String,
    blockchains: Vec<Blockchain>,
}

#[derive(Debug, Clone, PartialEq)]
struct Usuario {
    id: u32,
    nombre: String,
    apellido: String,
    email: String,
    dni: String,
    validado: bool,
    balance_fiat: f64,
    balances_cripto: HashMap<Criptomoneda, f64>,
}

#[derive(Clone, PartialEq, Debug)]
struct Plataforma {
    usuarios: Vec<Usuario>,
    criptomonedas: Vec<Criptomoneda>,
    blockchains: Vec<Blockchain>,
    transacciones_fiat: Vec<TransaccionFiat>,
    transacciones_cripto: Vec<TransaccionCripto>,
    cotizaciones: HashMap<String, f64>,
}

impl Blockchain {
    fn generar_hash(&self) -> String {
        format!("{}-{}", self.prefijo, rand::random::<u32>())
    }
}

impl TransaccionFiat {
    fn new_ingreso_fiat(medio_pago: MedioPago, fecha: Fecha, monto: f64, usuario_id: u32) -> Self {
        let transaccion =
            Transaccion::new(fecha, TipoTransaccion::IngresoDinero, monto, usuario_id);
        TransaccionFiat {
            transaccion,
            medio_pago,
        }
    }
    fn new_retiro_fiat(medio_pago: MedioPago, fecha: Fecha, monto: f64, usuario_id: u32) -> Self {
        let transaccion = Transaccion::new(fecha, TipoTransaccion::RetiroFiat, monto, usuario_id);
        TransaccionFiat {
            transaccion,
            medio_pago,
        }
    }
}

impl TransaccionCripto {
    fn new_compra(
        criptomoneda: Criptomoneda,
        cotizacion: Option<f64>,
        fecha: Fecha,
        monto: f64,
        usuario_id: u32,
    ) -> Self {
        let transaccion = Transaccion::new(fecha, TipoTransaccion::CompraCripto, monto, usuario_id);
        TransaccionCripto {
            transaccion,
            criptomoneda,
            cotizacion,
            hash: None,
        }
    }

    fn new_venta(
        criptomoneda: Criptomoneda,
        cotizacion: Option<f64>,
        fecha: Fecha,
        monto: f64,
        usuario_id: u32,
    ) -> Self {
        let transaccion = Transaccion::new(fecha, TipoTransaccion::VentaCripto, monto, usuario_id);
        TransaccionCripto {
            transaccion,
            criptomoneda,
            cotizacion,
            hash: None,
        }
    }

    fn new_retiro(
        criptomoneda: Criptomoneda,
        fecha: Fecha,
        monto: f64,
        usuario_id: u32,
        hash: Option<String>,
    ) -> Self {
        let transaccion = Transaccion::new(fecha, TipoTransaccion::RetiroCripto, monto, usuario_id);
        TransaccionCripto {
            transaccion,
            cotizacion: None,
            criptomoneda,
            hash,
        }
    }
    fn new_recepcion(
        criptomoneda: Criptomoneda,
        fecha: Fecha,
        monto: f64,
        usuario_id: u32,
    ) -> Self {
        let transaccion =
            Transaccion::new(fecha, TipoTransaccion::RecepcionCripto, monto, usuario_id);
        TransaccionCripto {
            transaccion,
            cotizacion: None,
            criptomoneda,
            hash: None,
        }
    }
}
impl Transaccion {
    fn new(fecha: Fecha, tipo: TipoTransaccion, monto: f64, usuario_id: u32) -> Self {
        Transaccion {
            fecha,
            tipo,
            monto,
            usuario_id,
        }
    }
}

impl Usuario {
    fn new(
        id: u32,
        nombre: String,
        apellido: String,
        email: String,
        dni: String,
        validado: bool,
    ) -> Self {
        Usuario {
            id,
            nombre,
            apellido,
            email,
            dni,
            validado,
            balance_fiat: 0.0,
            balances_cripto: HashMap::new(),
        }
    }

    fn es_valido(&self) -> bool {
        self.validado
    }
    fn es_validado(&self) -> Result<(), PlataformaError> {
        if self.validado {
            Ok(())
        } else {
            Err(PlataformaError::UsuarioNoValidado(self.dni.clone()))
        }
    }
    fn get_balance_fiat(&self) -> f64 {
        self.balance_fiat
    }
    fn get_balance_cripto(&self, cripto: &Criptomoneda) -> Result<f64, PlataformaError> {
        self.balances_cripto
            .get(cripto)
            .cloned()
            .ok_or_else(|| PlataformaError::CriptoNoEncontrada(cripto.prefijo.clone()))
    }
}

impl Plataforma {
    fn new() -> Self {
        Plataforma {
            usuarios: Vec::new(),
            criptomonedas: Vec::new(),
            blockchains: Vec::new(),
            transacciones_cripto: Vec::new(),
            transacciones_fiat: Vec::new(),
            cotizaciones: HashMap::new(),
        }
    }
    fn agregar_usuario(&mut self, usuario: Usuario) {
        self.usuarios.push(usuario);
    }
    fn buscar_usuario_mut(&mut self, id: u32) -> Option<&mut Usuario> {
        self.usuarios.iter_mut().find(|u| u.id == id)
    }
    fn obtener_usuario_mut(&mut self, id: u32) -> Result<&mut Usuario, PlataformaError> {
        self.buscar_usuario_mut(id)
            .ok_or(PlataformaError::UsuarioNoEncontrado(id))
    }
    fn monto_valido(monto: f64) -> Result<(), PlataformaError> {
        if monto <= 0.0 {
            Err(PlataformaError::MontoInvalido(monto))
        } else {
            Ok(())
        }
    }
    fn agregar_cotizacion(&mut self, prefijo: String, cotizacion: f64) {
        self.cotizaciones.insert(prefijo, cotizacion);
    }

    fn monto_insuficiente(monto: f64, saldo: f64) -> Result<(), PlataformaError> {
        if monto > saldo {
            Err(PlataformaError::FondoInsuficiente(saldo - monto))
        } else {
            Ok(())
        }
    }

    fn get_cotizacion(&self, prefijo: &str) -> Result<f64, PlataformaError> {
        self.cotizaciones
            .get(prefijo)
            .cloned()
            .ok_or(PlataformaError::CotizacionNoEncontrada(prefijo.to_string()))
    }

    fn get_criptomoneda(&self, prefijo: &str) -> Result<Criptomoneda, PlataformaError> {
        self.criptomonedas
            .iter()
            .find(|c| c.prefijo == prefijo)
            .cloned()
            .ok_or(PlataformaError::CriptoNoEncontrada(prefijo.to_string()))
    }

    fn existe_blockchain(&self, prefijo: &str) -> Result<Blockchain, PlataformaError> {
        self.blockchains
            .iter()
            .find(|b| b.prefijo == prefijo)
            .cloned()
            .ok_or(PlataformaError::BlockchainNoEncontrada(prefijo.to_string()))
    }

    fn ingresar_dinero(
        &mut self,
        user_id: u32,
        monto: f64,
        fecha: Fecha,
        medio: MedioPago,
    ) -> Result<(), PlataformaError> {
        Plataforma::monto_valido(monto)?;
        let usuario = self.obtener_usuario_mut(user_id)?;

        usuario.es_validado()?;

        usuario.balance_fiat += monto;

        let tx = TransaccionFiat::new_ingreso_fiat(medio, fecha, monto, usuario.id);
        self.transacciones_fiat.push(tx);
        Ok(())
    }

    fn retirar_dinero(
        &mut self,
        user_id: u32,
        monto: f64,
        fecha: Fecha,
        medio: MedioPago,
    ) -> Result<(), PlataformaError> {
        Plataforma::monto_valido(monto)?;
        let usuario = self.obtener_usuario_mut(user_id)?;

        usuario.es_validado()?;

        Plataforma::monto_insuficiente(monto, usuario.balance_fiat)?;
        usuario.balance_fiat -= monto;

        let tx = TransaccionFiat::new_retiro_fiat(medio, fecha, monto, usuario.id);
        self.transacciones_fiat.push(tx);
        Ok(())
    }

    fn comprar_cripto(
        &mut self,
        user_id: u32,
        monto: f64,
        fecha: Fecha,
        cripto: Criptomoneda,
    ) -> Result<(), PlataformaError> {
        Plataforma::monto_valido(monto)?;
        let prefijo = cripto.prefijo.clone();
        let cotizacion = self.get_cotizacion(&prefijo)?;
        let usuario = self.obtener_usuario_mut(user_id)?;

        usuario.es_validado()?;

        Plataforma::monto_insuficiente(monto, usuario.balance_fiat)?;
        let cantidad_crypto = monto / cotizacion;
        *usuario.balances_cripto.entry(cripto.clone()).or_insert(0.0) += cantidad_crypto;
        usuario.balance_fiat -= monto;
        let tx = TransaccionCripto::new_compra(cripto, Some(cotizacion), fecha, monto, user_id);
        self.transacciones_cripto.push(tx);
        Ok(())
    }

    fn vender_cripto(
        &mut self,
        user_id: u32,
        monto: f64,
        fecha: Fecha,
        cripto: Criptomoneda,
    ) -> Result<(), PlataformaError> {
        Plataforma::monto_valido(monto)?;
        let prefijo = cripto.prefijo.clone();
        let cotizacion = self.get_cotizacion(&prefijo)?;
        let usuario = self.obtener_usuario_mut(user_id)?;

        usuario.es_validado()?;

        let monto_cripto = monto / cotizacion;
        let saldo_cripto_actual = usuario.get_balance_cripto(&cripto)?;

        Plataforma::monto_insuficiente(monto_cripto, saldo_cripto_actual)?;
        *usuario.balances_cripto.entry(cripto.clone()).or_insert(0.0) -= monto_cripto;
        usuario.balance_fiat += monto;

        let tx = TransaccionCripto::new_venta(cripto, Some(cotizacion), fecha, monto, user_id);
        self.transacciones_cripto.push(tx);
        Ok(())
    }

    fn retirar_cripto_a_blockchain(
        &mut self,
        user_id: u32,
        monto: f64,
        blockchain: Blockchain,
        cripto: Criptomoneda,
        fecha: Fecha,
    ) -> Result<(), PlataformaError> {
        Plataforma::monto_valido(monto)?;
        let prefijo = blockchain.prefijo.clone();
        self.existe_blockchain(&prefijo)?;

        let usuario = self.obtener_usuario_mut(user_id)?;
        usuario.es_validado()?;
        let saldo_cripto_actual = usuario.get_balance_cripto(&cripto)?;
        Plataforma::monto_insuficiente(monto, saldo_cripto_actual)?;

        *usuario.balances_cripto.entry(cripto.clone()).or_insert(0.0) -= monto;
        let hash = blockchain.generar_hash();
        let tx = TransaccionCripto::new_retiro(cripto, fecha, monto, user_id, Some(hash.clone()));
        self.transacciones_cripto.push(tx);

        Ok(())
    }

    fn recibir_cripto_desde_blockchain(
        &mut self,
        user_id: u32,
        monto: f64,
        blockchain: Blockchain,
        cripto: Criptomoneda,
        fecha: Fecha,
    ) -> Result<(), PlataformaError> {
        Plataforma::monto_valido(monto)?;

        // Validar que la blockchain exista
        let prefijo_blockchain = &blockchain.prefijo;
        self.existe_blockchain(prefijo_blockchain)?;

        // Obtener al usuario
        let usuario = self.obtener_usuario_mut(user_id)?;

        // Acreditar el monto al usuario en esa cripto
        *usuario.balances_cripto.entry(cripto.clone()).or_insert(0.0) += monto;

        // Crear transacción de recepción
        let tx = TransaccionCripto::new_recepcion(cripto, fecha, monto, user_id);
        self.transacciones_cripto.push(tx);

        Ok(())
    }
    fn cripto_con_mas_ventas(&self) -> Option<Criptomoneda> {
        self.transacciones_cripto
            .iter()
            .filter(|tx| tx.transaccion.tipo == TipoTransaccion::VentaCripto)
            .fold(HashMap::new(), |mut acc, tx| {
                *acc.entry(tx.criptomoneda.clone()).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(cripto, _)| cripto)
    }

    fn cripto_con_mas_compras(&self) -> Option<Criptomoneda> {
        self.transacciones_cripto
            .iter()
            .filter(|tx| tx.transaccion.tipo == TipoTransaccion::CompraCripto)
            .fold(HashMap::new(), |mut acc, tx| {
                *acc.entry(tx.criptomoneda.clone()).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(cripto, _)| cripto)
    }
    fn cripto_con_mayor_volumen_venta(&self) -> Option<Criptomoneda> {
        self.transacciones_cripto
            .iter()
            .filter(|tx| tx.transaccion.tipo == TipoTransaccion::VentaCripto)
            .fold(HashMap::new(), |mut acc, tx| {
                *acc.entry(tx.criptomoneda.clone()).or_insert(0.0) += tx.transaccion.monto;
                acc
            })
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
            .map(|(cripto, _)| cripto)
    }
    fn cripto_con_mayor_volumen_compra(&self) -> Option<Criptomoneda> {
        self.transacciones_cripto
            .iter()
            .filter(|tx| tx.transaccion.tipo == TipoTransaccion::CompraCripto)
            .fold(HashMap::new(), |mut acc, tx| {
                *acc.entry(tx.criptomoneda.clone()).or_insert(0.0) += tx.transaccion.monto;
                acc
            })
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
            .map(|(cripto, _)| cripto)
    }
}

//TESTS
#[cfg(test)]
mod tests {
    use super::*;

    fn usuario_de_prueba() -> Usuario {
        Usuario::new(
            1,
            "José".to_string(),
            "Pérez".to_string(),
            "jose@example.com".to_string(),
            "12345678".to_string(),
            false,
        )
    }

    fn fecha_dummy() -> Fecha {
        Fecha::new(1, 1, 2000).expect("Fecha inválida en fecha_dummy")
    }

    fn plataforma_dummy() -> Plataforma {
        let mut plataforma = Plataforma::new();
        plataforma.agregar_usuario(usuario_de_prueba());
        plataforma
    }

    #[test]
    fn ingresar_dinero_usuario_existente() {
        let mut plataforma = plataforma_dummy();
        let resultado =
            plataforma.ingresar_dinero(1, 1000.0, fecha_dummy(), MedioPago::MercadoPago);

        assert!(resultado.is_ok());
        let usuario = plataforma
            .usuarios
            .iter()
            .find(|u| u.dni == "12345678")
            .unwrap();
        assert_eq!(usuario.balance_fiat, 1000.0);
        assert_eq!(plataforma.transacciones_fiat.len(), 1);

        let tx = &plataforma.transacciones_fiat[0];
        assert_eq!(tx.transaccion.tipo, TipoTransaccion::IngresoDinero);
        assert_eq!(tx.transaccion.monto, 1000.0);
        assert_eq!(tx.transaccion.usuario_id, 1);
    }

    #[test]
    fn test_usuario_no_encontrado() {
        let mut plataforma = plataforma_dummy();
        let resultado = plataforma.buscar_usuario_mut(99999999);
        assert!(resultado.is_none());
    }

    #[test]
    fn test_monto_valido_negativo() {
        assert_eq!(
            Plataforma::monto_valido(0.0),
            Err(PlataformaError::MontoInvalido(0.0))
        );
        assert_eq!(
            Plataforma::monto_valido(-5.0),
            Err(PlataformaError::MontoInvalido(-5.0))
        );
    }

    #[test]
    fn test_monto_valido_positivo() {
        assert!(Plataforma::monto_valido(10.0).is_ok());
    }

    #[test]
    fn ingresar_dinero_usuario_inexistente() {
        let mut plataforma = plataforma_dummy();

        let resultado =
            plataforma.ingresar_dinero(99999999, 1000.0, fecha_dummy(), MedioPago::MercadoPago);

        assert!(resultado.is_err());
        if let Err(PlataformaError::UsuarioNoEncontrado(dni)) = resultado {
            assert_eq!(dni, 99999999);
        } else {
            panic!("Se esperaba PlataformaError::UsuarioNoEncontrado");
        }

        assert_eq!(plataforma.transacciones_fiat.len(), 0);
    }

    #[test]
    fn usuario_validado() {
        let user_validado = Usuario::new(
            1,
            "José".to_string(),
            "Pérez".to_string(),
            "jose@example.com".to_string(),
            "12345678".to_string(),
            true,
        );
        assert!(user_validado.validado);
    }

    #[test]
    fn usuario_no_validado() {
        let user_no_validado = Usuario::new(
            2,
            "José".to_string(),
            "Pérez".to_string(),
            "jose@example.com".to_string(),
            "12345678".to_string(),
            false,
        );
        assert!(!user_no_validado.validado);
    }
}
