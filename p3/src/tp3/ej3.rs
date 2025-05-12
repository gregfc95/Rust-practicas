/*
3- Escribir un programa que defina una estructura Fecha que tenga campos para el día, el
mes y el año. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
➢ es_fecha_valida: retorna true si es una fecha válida, false caso contrario.//tenga en
cuenta los años bisiestos también.
➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose
➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose
➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a
la fecha pasada por parámetro.
*/

use chrono::prelude::*;

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
    pub fn es_fecha_valida(&self) -> bool {
        let okay: bool = NaiveDate::from_ymd_opt(self.anio, self.mes, self.dia).is_some();
        okay
    }

    pub fn es_bisiesto(&self) -> bool {
        NaiveDate::from_ymd_opt(self.anio, self.mes, self.dia)
            .unwrap()
            .leap_year()
    }

    pub fn sumar_dias(&self, dias: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.anio, self.mes, self.dia)?
            .checked_add_days(chrono::Days::new(dias.into()))
    }

    pub fn restar_dias(&self, dias: u32) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.anio, self.mes, self.dia)?
            .checked_sub_days(chrono::Days::new(dias.into()))
    }

    pub fn es_mayor(&self, otra_fecha: &Self) -> bool {
        NaiveDate::from_ymd_opt(self.anio, self.mes, self.dia)
            > NaiveDate::from_ymd_opt(otra_fecha.anio, otra_fecha.mes, otra_fecha.dia)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fecha = Fecha::new(1, 1, 2000);
        assert!(fecha.is_some());
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

        assert_eq!(resultado.month(), 1);
        assert_eq!(resultado.day(), 1);
        assert_eq!(resultado.year(), 2001);
    }

    #[test]
    fn test_restar_dias() {
        let fecha = Fecha::new(1, 1, 2000).expect("Fecha inválida");
        let resultado = fecha.restar_dias(3).expect("Error al restar días");

        assert_eq!(resultado.month(), 12);
        assert_eq!(resultado.day(), 29);
        assert_eq!(resultado.year(), 1999);
    }

    #[test]
    fn test_fecha_es_mayor() {
        let fecha1 = Fecha::new(1, 1, 2000).expect("Fecha inválida");
        let fecha2 = Fecha::new(1, 1, 2001).expect("Fecha inválida");
        assert!(false == fecha1.es_mayor(&fecha2));
        assert!(true == fecha2.es_mayor(&fecha1));
    }
}
