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

    fn fecha_dummy() -> Fecha {
        Fecha::new(1, 1, 2000).expect("Fecha inválida en fecha_dummy")
    }
    fn usuario_validado(id: u32) -> Usuario {
        Usuario::new(
            id,
            "Ana".to_string(),
            "García".to_string(),
            "ana@mail.com".to_string(),
            "11111111".to_string(),
            true,
        )
    }
    fn usuario_no_validado(id: u32) -> Usuario {
        Usuario::new(
            id,
            "Juan".to_string(),
            "Pérez".to_string(),
            "juan@mail.com".to_string(),
            "22222222".to_string(),
            false,
        )
    }
    fn cripto_btc() -> Criptomoneda {
        Criptomoneda {
            nombre: "Bitcoin".to_string(),
            prefijo: "BTC".to_string(),
            blockchains: vec![blockchain_btc()],
        }
    }
    fn cripto_eth() -> Criptomoneda {
        Criptomoneda {
            nombre: "Ethereum".to_string(),
            prefijo: "ETH".to_string(),
            blockchains: vec![blockchain_eth()],
        }
    }
    fn blockchain_btc() -> Blockchain {
        Blockchain {
            nombre: "BitcoinChain".to_string(),
            prefijo: "BTC".to_string(),
        }
    }
    fn blockchain_eth() -> Blockchain {
        Blockchain {
            nombre: "EthereumChain".to_string(),
            prefijo: "ETH".to_string(),
        }
    }
    fn plataforma_basica() -> Plataforma {
        let mut p = Plataforma::new();
        p.criptomonedas.push(cripto_btc());
        p.criptomonedas.push(cripto_eth());
        p.blockchains.push(blockchain_btc());
        p.blockchains.push(blockchain_eth());
        p.agregar_cotizacion("BTC".to_string(), 10000.0);
        p.agregar_cotizacion("ETH".to_string(), 2000.0);
        p
    }

    #[test]
    fn test_usuario_es_valido() {
        let u = usuario_validado(1);
        assert!(u.es_valido());
        let u2 = usuario_no_validado(2);
        assert!(!u2.es_valido());
    }

    #[test]
    fn test_usuario_get_balance_fiat() {
        let u = usuario_validado(1);
        assert_eq!(u.get_balance_fiat(), 0.0);
    }

    #[test]
    fn test_usuario_get_balance_cripto_none() {
        let u = usuario_validado(1);
        let btc = cripto_btc();
        assert!(u.get_balance_cripto(&btc).is_err());
    }

    #[test]
    fn test_blockchain_generar_hash() {
        let b = blockchain_btc();
        let hash = b.generar_hash();
        assert!(hash.starts_with("BTC-"));
    }

    #[test]
    fn test_plataforma_new() {
        let p = Plataforma::new();
        assert_eq!(p.usuarios.len(), 0);
        assert_eq!(p.criptomonedas.len(), 0);
        assert_eq!(p.blockchains.len(), 0);
    }

    #[test]
    fn test_plataforma_agregar_usuario() {
        let mut p = Plataforma::new();
        p.agregar_usuario(usuario_validado(1));
        assert_eq!(p.usuarios.len(), 1);
    }

    #[test]
    fn test_plataforma_buscar_usuario_mut() {
        let mut p = Plataforma::new();
        p.agregar_usuario(usuario_validado(1));
        assert!(p.buscar_usuario_mut(1).is_some());
        assert!(p.buscar_usuario_mut(2).is_none());
    }

    #[test]
    fn test_plataforma_obtener_usuario_mut() {
        let mut p = Plataforma::new();
        p.agregar_usuario(usuario_validado(1));
        assert!(p.obtener_usuario_mut(1).is_ok());
        assert!(p.obtener_usuario_mut(2).is_err());
    }

    #[test]
    fn test_plataforma_monto_valido() {
        assert!(Plataforma::monto_valido(10.0).is_ok());
        assert!(Plataforma::monto_valido(0.0).is_err());
        assert!(Plataforma::monto_valido(-1.0).is_err());
    }

    #[test]
    fn test_plataforma_agregar_cotizacion_y_get() {
        let mut p = Plataforma::new();
        p.agregar_cotizacion("BTC".to_string(), 10000.0);
        assert_eq!(p.get_cotizacion("BTC").unwrap(), 10000.0);
        assert!(p.get_cotizacion("ETH").is_err());
    }

    #[test]
    fn test_plataforma_get_criptomoneda() {
        let mut p = Plataforma::new();
        p.criptomonedas.push(cripto_btc());
        assert!(p.get_criptomoneda("BTC").is_ok());
        assert!(p.get_criptomoneda("ETH").is_err());
    }

    #[test]
    fn test_plataforma_existe_blockchain() {
        let mut p = Plataforma::new();
        p.blockchains.push(blockchain_btc());
        assert!(p.existe_blockchain("BTC").is_ok());
        assert!(p.existe_blockchain("ETH").is_err());
    }

    #[test]
    fn test_plataforma_ingresar_dinero() {
        let mut p = plataforma_basica();
        p.agregar_usuario(usuario_validado(1));
        let res = p.ingresar_dinero(1, 1000.0, fecha_dummy(), MedioPago::MercadoPago);
        assert!(res.is_ok());
        let u = p.buscar_usuario_mut(1).unwrap();
        assert_eq!(u.balance_fiat, 1000.0);
        assert_eq!(p.transacciones_fiat.len(), 1);
    }

    #[test]
    fn test_plataforma_retirar_dinero() {
        let mut p = plataforma_basica();
        p.agregar_usuario(usuario_validado(1));
        p.ingresar_dinero(1, 1000.0, fecha_dummy(), MedioPago::MercadoPago)
            .unwrap();
        let res = p.retirar_dinero(1, 500.0, fecha_dummy(), MedioPago::TransferenciaBancaria);
        assert!(res.is_ok());
        let u = p.buscar_usuario_mut(1).unwrap();
        assert_eq!(u.balance_fiat, 500.0);
        assert_eq!(p.transacciones_fiat.len(), 2);
    }

    #[test]
    fn test_plataforma_comprar_cripto() {
        let mut p = plataforma_basica();
        p.agregar_usuario(usuario_validado(1));
        p.ingresar_dinero(1, 10000.0, fecha_dummy(), MedioPago::MercadoPago)
            .unwrap();
        let btc = p.get_criptomoneda("BTC").unwrap();
        let res = p.comprar_cripto(1, 5000.0, fecha_dummy(), btc.clone());
        assert!(res.is_ok());
        let u = p.buscar_usuario_mut(1).unwrap();
        // 5000 / 10000 = 0.5 BTC
        assert!((u.get_balance_cripto(&btc).unwrap() - 0.5).abs() < 0.0001);
        assert_eq!(u.balance_fiat, 5000.0);
        assert_eq!(p.transacciones_cripto.len(), 1);
    }

    #[test]
    fn test_plataforma_vender_cripto() {
        let mut p = plataforma_basica();
        p.agregar_usuario(usuario_validado(1));
        p.ingresar_dinero(1, 10000.0, fecha_dummy(), MedioPago::MercadoPago)
            .unwrap();
        let btc = p.get_criptomoneda("BTC").unwrap();
        p.comprar_cripto(1, 5000.0, fecha_dummy(), btc.clone())
            .unwrap();
        let res = p.vender_cripto(1, 2000.0, fecha_dummy(), btc.clone());
        assert!(res.is_ok());
        let u = p.buscar_usuario_mut(1).unwrap();
        // Vendió 2000/10000 = 0.2 BTC
        assert!((u.get_balance_cripto(&btc).unwrap() - 0.3).abs() < 0.0001);
        assert!((u.balance_fiat - 7000.0).abs() < 0.0001);
        assert_eq!(p.transacciones_cripto.len(), 2);
    }

    #[test]
    fn test_plataforma_retirar_cripto_a_blockchain() {
        let mut p = plataforma_basica();
        p.agregar_usuario(usuario_validado(1));
        p.ingresar_dinero(1, 10000.0, fecha_dummy(), MedioPago::MercadoPago)
            .unwrap();
        let btc = p.get_criptomoneda("BTC").unwrap();
        p.comprar_cripto(1, 10000.0, fecha_dummy(), btc.clone())
            .unwrap();
        let bc = p.existe_blockchain("BTC").unwrap();
        let res = p.retirar_cripto_a_blockchain(1, 0.5, bc.clone(), btc.clone(), fecha_dummy());
        assert!(res.is_ok());
        let u = p.buscar_usuario_mut(1).unwrap();
        assert!((u.get_balance_cripto(&btc).unwrap() - 0.5).abs() < 0.0001);
        assert_eq!(p.transacciones_cripto.len(), 2);
    }

    #[test]
    fn test_plataforma_recibir_cripto_desde_blockchain() {
        let mut p = plataforma_basica();
        p.agregar_usuario(usuario_validado(1));
        let btc = p.get_criptomoneda("BTC").unwrap();
        let bc = p.existe_blockchain("BTC").unwrap();
        let res =
            p.recibir_cripto_desde_blockchain(1, 0.25, bc.clone(), btc.clone(), fecha_dummy());
        assert!(res.is_ok());
        let u = p.buscar_usuario_mut(1).unwrap();
        assert!((u.get_balance_cripto(&btc).unwrap() - 0.25).abs() < 0.0001);
        assert_eq!(p.transacciones_cripto.len(), 1);
    }

    #[test]
    fn test_plataforma_cripto_con_mas_ventas_y_compras() {
        let mut p = plataforma_basica();
        p.agregar_usuario(usuario_validado(1));
        p.ingresar_dinero(1, 20000.0, fecha_dummy(), MedioPago::MercadoPago)
            .unwrap();
        let btc = p.get_criptomoneda("BTC").unwrap();
        let eth = p.get_criptomoneda("ETH").unwrap();
        p.comprar_cripto(1, 10000.0, fecha_dummy(), btc.clone())
            .unwrap();
        p.comprar_cripto(1, 4000.0, fecha_dummy(), eth.clone())
            .unwrap();
        p.comprar_cripto(1, 2000.0, fecha_dummy(), eth.clone())
            .unwrap();
        p.vender_cripto(1, 5000.0, fecha_dummy(), btc.clone())
            .unwrap();
        p.vender_cripto(1, 1000.0, fecha_dummy(), eth.clone())
            .unwrap();
        // Agrego una venta extra de BTC para que BTC tenga más ventas que ETH
        p.vender_cripto(1, 1000.0, fecha_dummy(), btc.clone())
            .unwrap();
        // ETH tiene más compras (2), BTC más ventas (2)
        assert_eq!(p.cripto_con_mas_compras().unwrap().prefijo, "ETH");
        assert_eq!(p.cripto_con_mas_ventas().unwrap().prefijo, "BTC");
    }

    #[test]
    fn test_plataforma_cripto_con_mayor_volumen_venta_y_compra() {
        let mut p = plataforma_basica();
        p.agregar_usuario(usuario_validado(1));
        p.ingresar_dinero(1, 20000.0, fecha_dummy(), MedioPago::MercadoPago)
            .unwrap();
        let btc = p.get_criptomoneda("BTC").unwrap();
        let eth = p.get_criptomoneda("ETH").unwrap();
        p.comprar_cripto(1, 10000.0, fecha_dummy(), btc.clone())
            .unwrap();
        p.comprar_cripto(1, 4000.0, fecha_dummy(), eth.clone())
            .unwrap();
        p.comprar_cripto(1, 2000.0, fecha_dummy(), eth.clone())
            .unwrap();
        p.vender_cripto(1, 5000.0, fecha_dummy(), btc.clone())
            .unwrap();
        p.vender_cripto(1, 1000.0, fecha_dummy(), eth.clone())
            .unwrap();
        p.vender_cripto(1, 1000.0, fecha_dummy(), btc.clone())
            .unwrap();
        // Mayor volumen de compra: BTC (10000), venta: BTC (5000)
        assert_eq!(p.cripto_con_mayor_volumen_compra().unwrap().prefijo, "BTC");
        assert_eq!(p.cripto_con_mayor_volumen_venta().unwrap().prefijo, "BTC");
    }
}
