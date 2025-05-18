/*
2- Escribir un programa que defina la estructura Rectángulo que tenga campos para la
longitud y el ancho. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Rectángulo y lo
retorna.
➢ calcular_area: calcular el área y la retorna.
➢ calcular_perimetro: calcula el perímetro y lo retorna.
➢ es_cuadrado: retorna true si es cuadrado, false caso contrario
*/

pub struct Rectangulo {
    longitud: f32,
    ancho: f32,
}

impl Rectangulo {
    pub fn new(longitud: f32, ancho: f32) -> Self {
        let l: f32 = longitud.abs();
        let a: f32 = ancho.abs();
        Rectangulo {
            longitud: l,
            ancho: a,
        }
    }
    pub fn calcular_area(&self) -> f32 {
        let area = self.longitud * self.ancho;
        area
    }
    pub fn calcular_perimetro(&self) -> f32 {
        let perimetro = 2.0 * (self.longitud + self.ancho);
        perimetro
    }
    pub fn es_cuadrado(&self) -> bool {
        let ok: bool = self.longitud == self.ancho;
        ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_rectangulo() {
        let rec1 = Rectangulo::new(10.0, 5.0);
        assert_eq!(rec1.longitud, 10.0);
        assert_eq!(rec1.ancho, 5.0);
    }

    #[test]
    fn test_crear_rectangulo_cero() {
        let rec1 = Rectangulo::new(0.0, 0.0);
        assert_eq!(rec1.longitud, 0.0);
        assert_eq!(rec1.ancho, 0.0);
    }

    #[test]
    fn test_calcular_area_cero() {
        let rec1 = Rectangulo::new(0.0, 0.0);
        assert_eq!(rec1.calcular_area(), 0.0);
    }

    #[test]
    fn test_calcular_perimetro_cero() {
        let rec1 = Rectangulo::new(0.0, 0.0);
        assert_eq!(rec1.calcular_perimetro(), 0.0);
    }

    #[test]
    fn test_crear_rectangulo_negativo() {
        let rec1 = Rectangulo::new(-10.0, -5.0);
        assert_eq!(rec1.longitud, 10.0);
        assert_eq!(rec1.ancho, 5.0);
    }

    #[test]
    fn test_calcular_area() {
        let rec1 = Rectangulo::new(10.0, 5.0);
        assert_eq!(rec1.calcular_area(), 50.0);
    }

    #[test]
    fn test_calcular_perimetro() {
        let rec1 = Rectangulo::new(10.0, 5.0);
        assert_eq!(rec1.calcular_perimetro(), 30.0);
    }

    #[test]
    fn test_devolver_rectangulo() {
        let rec1 = Rectangulo::new(10.0, 10.0);
        assert_eq!(rec1.es_cuadrado(), true);
        let rec2 = Rectangulo::new(0.0, 0.0);
        assert_eq!(rec2.es_cuadrado(), true);
    }
}
