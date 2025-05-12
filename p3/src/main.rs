pub mod tp3;
use tp3::ej1::Persona;
use tp3::ej2::Rectangulo;
fn main() {
    //ej1
    let persona = Persona::new("Carla".to_string(), 30, None);
    let persona2 = Persona::new("Juan".to_string(), 99, Some("calle 123".to_string()));
    println!("{} {}", persona.to_string(), persona2.to_string());

    //ej2
    let rectangulo = Rectangulo::new(10.0, 5.0);
    println!(
        "Area: {}, Perimetro: {}, Esto es un cuadrado?: {}",
        rectangulo.calcular_area(),
        rectangulo.calcular_perimetro(),
        rectangulo.es_cuadrado()
    );

    //ej3
}
