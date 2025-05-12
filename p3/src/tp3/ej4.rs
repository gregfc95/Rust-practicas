/*
4- Escribir un programa que defina la estructura Triángulo que tenga campos para las
longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos:
➢ new: que pasando los parámetros correspondientes, crea un Triángulo y lo retorna.
➢ determinar_tipo: retorna el tipo del triángulo, los tipos pueden ser equilátero,
isósceles o escaleno.
➢ calcular_area: calcular el área y la retorna.
➢ calcular_perimetro: calcula el perímetro y lo retorna.
*/

pub struct Triangulo {
    lado1: f32,
    lado2: f32,
    lado3: f32,
}
#[derive(Debug, PartialEq)]
pub enum Tipo {
    Equilatero,
    Isosceles,
    Escaleno,
}
impl Triangulo {
    fn determinar_tipo(&self) -> Tipo {
        if self.lado1 == self.lado2 && self.lado2 == self.lado3 {
            Tipo::Equilatero
        } else if self.lado1 == self.lado2 || self.lado2 == self.lado3 || self.lado1 == self.lado3 {
            Tipo::Isosceles
        } else {
            Tipo::Escaleno
        }
    }
    pub fn new(lado1: f32, lado2: f32, lado3: f32) -> Self {
        Triangulo {
            lado1,
            lado2,
            lado3,
        }
    }

    pub fn calcular_area(&self) -> f32 {
        let s: f32 = (self.lado1 + self.lado2 + self.lado3) / 2.0;
        let heron: f32 = (s * (s - self.lado1) * (s - self.lado2) * (s - self.lado3)).sqrt();
        heron
    }

    pub fn calcular_perimetro(&self) -> f32 {
        let resultado: f32 = self.lado1 + self.lado2 + self.lado3;
        resultado
    }

    pub fn triangulo_to_string(&self) -> String {
        let tipo = self.determinar_tipo();
        match tipo {
            Tipo::Equilatero => "Equilatero".to_string(),
            Tipo::Isosceles => "Isosceles".to_string(),
            Tipo::Escaleno => "Escaleno".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = Triangulo::new(1.0, 2.0, 3.0);
        assert_eq!(t.lado1, 1.0);
        assert_eq!(t.lado2, 2.0);
        assert_eq!(t.lado3, 3.0);
    }

    #[test]
    fn test_determinar_tipo() {
        let t = Triangulo::new(1.0, 1.0, 1.0);
        assert_eq!(t.determinar_tipo(), Tipo::Equilatero);
        let t = Triangulo::new(1.0, 2.0, 3.0);
        assert_eq!(t.determinar_tipo(), Tipo::Escaleno);
        let t = Triangulo::new(1.0, 1.0, 2.0);
        assert_eq!(t.determinar_tipo(), Tipo::Isosceles);
    }

    #[test]
    fn test_calcular_area() {
        let t = Triangulo::new(1.0, 1.0, 1.0);
        assert_eq!(t.calcular_area(), 0.4330127);
        let t = Triangulo::new(3.0, 4.0, 5.0);
        assert_eq!(t.calcular_area(), 6.0);
    }

    #[test]
    fn test_calcular_perimetro() {
        let t = Triangulo::new(1.0, 1.0, 1.0);
        assert_eq!(t.calcular_perimetro(), 3.0);
        let t = Triangulo::new(3.0, 4.0, 5.0);
        assert_eq!(t.calcular_perimetro(), 12.0);
        let t = Triangulo::new(0.0, 0.0, 0.0);
        assert_eq!(t.calcular_perimetro(), 0.0);
    }
}
