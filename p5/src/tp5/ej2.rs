/*
8- Defina la estructura Cancion con campos para el título, el artista y el género. El género
puede ser rock, pop, rap, jazz, otros. Luego modele una playlist. La playlist está compuesta
por una lista de canciones y un nombre, y se permiten hacer las siguientes acciones sobre
ella:
➔ agregar canción.
➔ eliminar canción.
➔ mover canción // mueve la canción a una determinada posición de la playlist.
➔ buscar canción por nombre.
➔ obtener las canciones de un determinado género.
➔ obtener las canciones de un determinado artista.
➔ modificar título de la playlist.
➔ eliminar todas las canciones

2- En base al ejercicio 8 del tp#3 implemente lo siguiente:
a- Realice todos los tests de la funcionalidad implementada obteniendo un coverage
de por lo menos 90%
b- Una vez obtenido dicho coverage, las canciones de la playlist deben ser
guardadas en un archivo en formato JSON, por lo tanto las operaciones que agreguen,
quiten o modifiquen la playlist deben estar respaldadas sobre dicho archivo.
No debe modificar los tests hechos en el punto a. Si puede agregar más en caso de que
haga métodos nuevos. Recuerde también que se debe seguir manteniendo un coverage de
al menos 90%,
*/
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResultadoOperacion {
    /// La operación se realizó con éxito.
    Exito,
    /// La operación falló por una razón lógica (por ejemplo, canción no encontrada).
    Fallo(String),
    /// Ocurrió un error relacionado con archivos (por ejemplo, al guardar/cargar).
    ErrorArchivo(String),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Playlist {
    nombre: String,
    canciones: Vec<Cancion>,
}
impl Genero {
    pub fn igual(&self, otro: &Genero) -> bool {
        match (self, otro) {
            (Genero::Rock, Genero::Rock)
            | (Genero::Pop, Genero::Pop)
            | (Genero::Rap, Genero::Rap)
            | (Genero::Jazz, Genero::Jazz)
            | (Genero::Otros, Genero::Otros) => true,
            _ => false,
        }
    }
}

impl Playlist {
    pub fn new(nombre: String, canciones: Vec<Cancion>) -> Self {
        Playlist { nombre, canciones }
    }

    pub fn set_nombre(&mut self, nombre: String) {
        self.nombre = nombre;
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) -> bool {
        self.canciones.push(cancion);
        true
    }

    pub fn agregar_cancion_y_guardar(
        &mut self,
        cancion: Cancion,
        path: &str,
    ) -> Result<(), ResultadoOperacion> {
        self.agregar_cancion(cancion);
        self.guardar_en_archivo(path)
    }

    pub fn guardar_en_archivo(&self, path: &str) -> Result<(), ResultadoOperacion> {
        serde_json::to_writer_pretty(
            File::create(path).map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))?,
            &self.canciones,
        )
        .map_err(|e| ResultadoOperacion::ErrorArchivo(e.to_string()))
    }

    pub fn eliminar_cancion_y_guardar(
        &mut self,
        cancion: &Cancion,
        path: &str,
    ) -> Result<(), ResultadoOperacion> {
        match self.eliminar_cancion(cancion) {
            Some(_) => self.guardar_en_archivo(path),
            None => Err(ResultadoOperacion::Fallo(
                "Canción no encontrada".to_string(),
            )),
        }
    }

    pub fn eliminar_canciones_y_guardar(&mut self, path: &str) -> Result<(), ResultadoOperacion> {
        self.eliminar_canciones();
        self.guardar_en_archivo(path)
    }

    pub fn mover_cancion_y_guardar(
        &mut self,
        cancion: &Cancion,
        nueva_posicion: usize,
        path: &str,
    ) -> Result<(), ResultadoOperacion> {
        if self.mover_cancion(cancion, nueva_posicion) {
            self.guardar_en_archivo(path)
        } else {
            Err(ResultadoOperacion::Fallo(
                "No se pudo mover la canción".to_string(),
            ))
        }
    }

    pub fn set_nombre_y_guardar(
        &mut self,
        nombre: String,
        path: &str,
    ) -> Result<(), ResultadoOperacion> {
        self.set_nombre(nombre);
        self.guardar_en_archivo(path)
    }

    pub fn eliminar_cancion(&mut self, cancion: &Cancion) -> Option<Cancion> {
        let index = self.canciones.iter().position(|c| {
            c.titulo.as_str() == cancion.titulo.as_str()
                && c.artista.as_str() == cancion.artista.as_str()
                && c.genero.igual(&cancion.genero)
        })?;
        Some(self.canciones.remove(index))
    }
    pub fn buscar_cancion_por_nombre(&self, titulo: &str) -> Option<&Cancion> {
        self.canciones.iter().find(|c| c.titulo == titulo)
    }

    pub fn obtener_canciones_por_genero(&self, genero_buscado: Genero) -> Vec<&Cancion> {
        self.canciones
            .iter()
            .filter(|c| c.genero.igual(&genero_buscado))
            .collect()
    }

    pub fn obtener_canciones_por_artista(&self, artista_buscado: &str) -> Vec<&Cancion> {
        self.canciones
            .iter()
            .filter(|c| c.artista == artista_buscado)
            .collect()
    }

    pub fn eliminar_canciones(&mut self) {
        self.canciones.clear();
    }

    pub fn mover_cancion(&mut self, cancion: &Cancion, nueva_posicion: usize) -> bool {
        if nueva_posicion > self.canciones.len() {
            return false;
        }
        let index = match self.canciones.iter().position(|c| {
            c.titulo == cancion.titulo
                && c.artista == cancion.artista
                && c.genero.igual(&cancion.genero)
        }) {
            Some(i) => i,
            None => return false,
        };

        //Si esta en la pos correcta, no hacer nada
        if index == nueva_posicion {
            return false;
        }
        let cancion_movida = self.canciones.remove(index);
        //Evitar panic al insertar fuera de rango
        let index_insert = if nueva_posicion > index {
            nueva_posicion - 1 //Hacia delante
        } else {
            nueva_posicion // Hacia atras
        };
        self.canciones.insert(index_insert, cancion_movida);
        true
    }
}
impl Cancion {
    pub fn new(titulo: String, artista: String, genero: Genero) -> Self {
        Cancion {
            titulo,
            artista,
            genero,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_cancion() {
        let cancion = Cancion::new("Cancion".to_string(), "Artista".to_string(), Genero::Rock);
        assert_eq!(cancion.titulo, "Cancion");
        assert_eq!(cancion.artista, "Artista");
        assert!(cancion.genero.igual(&Genero::Rock));
    }

    #[test]
    fn test_set_nombre_playlist() {
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let nuevo_nombre = "Mi nueva playlist".to_string();
        playlist.set_nombre(nuevo_nombre);
        assert_eq!(playlist.nombre, "Mi nueva playlist");
    }

    #[test]
    fn test_agregar_cancion() {
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let cancion = Cancion::new("Cancion".to_string(), "Artista".to_string(), Genero::Rock);
        playlist.agregar_cancion(cancion);
        assert_eq!(playlist.canciones.len(), 1);
    }
    #[test]
    fn test_eliminar_canciones() {
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let cancion = Cancion::new("Cancion".to_string(), "Artista".to_string(), Genero::Rock);
        playlist.agregar_cancion(cancion);
        assert_eq!(playlist.canciones.len(), 1);
        playlist.eliminar_canciones();
        assert_eq!(playlist.canciones.len(), 0);
    }

    #[test]
    fn test_buscar_cancion_por_nombre() {
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let cancion = Cancion::new("Cancion".to_string(), "Artista".to_string(), Genero::Rock);
        playlist.agregar_cancion(cancion);
        assert_eq!(playlist.canciones.len(), 1);
        assert!(playlist.buscar_cancion_por_nombre("Cancion").is_some());
    }

    #[test]
    fn test_buscar_cancion_por_nombre_no_existe() {
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let cancion = Cancion::new("Cancion".to_string(), "Artista".to_string(), Genero::Rock);
        playlist.agregar_cancion(cancion);
        assert_eq!(playlist.canciones.len(), 1);
        assert!(playlist.buscar_cancion_por_nombre("NoExiste").is_none());
    }

    #[test]
    fn test_buscar_cancion_por_genero() {
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let cancion_rock = Cancion::new(
            "CancionRock".to_string(),
            "Artista".to_string(),
            Genero::Rock,
        );
        let cancion_pop =
            Cancion::new("CancionPop".to_string(), "Artista".to_string(), Genero::Pop);
        let cancion_jazz = Cancion::new(
            "CancionJazz".to_string(),
            "Artista".to_string(),
            Genero::Jazz,
        );
        playlist.agregar_cancion(cancion_rock);
        playlist.agregar_cancion(cancion_pop);
        playlist.agregar_cancion(cancion_jazz);
        let vec_rock: Vec<&Cancion> = playlist.obtener_canciones_por_genero(Genero::Rock);
        let vec_otros: Vec<&Cancion> = playlist.obtener_canciones_por_genero(Genero::Otros);
        assert_eq!(vec_rock.len(), 1);
        assert_eq!(vec_otros.len(), 0);
    }

    #[test]
    fn test_buscar_cancion_por_artista() {
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let cancion_rock = Cancion::new(
            "CancionRock".to_string(),
            "ArtistaRock".to_string(),
            Genero::Rock,
        );
        let cancion_pop = Cancion::new(
            "CancionPop".to_string(),
            "ArtistaPop".to_string(),
            Genero::Pop,
        );
        let cancion_jazz = Cancion::new(
            "CancionJazz".to_string(),
            "ArtistaJazz".to_string(),
            Genero::Jazz,
        );
        playlist.agregar_cancion(cancion_rock);
        playlist.agregar_cancion(cancion_pop);
        playlist.agregar_cancion(cancion_jazz);
        let vec_rock: Vec<&Cancion> = playlist.obtener_canciones_por_artista("ArtistaRock");
        let vec_otros: Vec<&Cancion> = playlist.obtener_canciones_por_artista("ArtistaOtros");
        assert_eq!(vec_rock.len(), 1);
        assert_eq!(vec_otros.len(), 0);
    }

    #[test]
    fn test_eliminar_cancion() {
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let cancion = Cancion::new("Cancion".to_string(), "Artista".to_string(), Genero::Rock);
        playlist.agregar_cancion(cancion.clone());
        assert_eq!(playlist.canciones.len(), 1);
        assert!(playlist.eliminar_cancion(&cancion).is_some());
        assert_eq!(playlist.canciones.len(), 0);
    }

    #[test]
    fn test_eliminar_cancion_no_existe() {
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let cancion = Cancion::new("Cancion".to_string(), "Artista".to_string(), Genero::Rock);
        let cancion_jazz = Cancion::new(
            "CancionJazz".to_string(),
            "ArtistaJazz".to_string(),
            Genero::Jazz,
        );
        playlist.agregar_cancion(cancion.clone());
        assert_eq!(playlist.canciones.len(), 1);
        assert!(playlist.eliminar_cancion(&cancion_jazz).is_none());
        assert_eq!(playlist.canciones.len(), 1);
    }

    #[test]
    fn test_mover_ultima_cancion_al_inicio() {
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let a = Cancion::new("A".to_string(), "ArtistaRock".to_string(), Genero::Rock);
        let b = Cancion::new("B".to_string(), "ArtistaPop".to_string(), Genero::Pop);
        let c = Cancion::new("C".to_string(), "ArtistaJazz".to_string(), Genero::Jazz);

        playlist.agregar_cancion(a);
        playlist.agregar_cancion(b);
        playlist.agregar_cancion(c.clone());

        assert!(playlist.mover_cancion(&c, 0));
        assert_eq!(playlist.canciones[0].titulo, "C");
        assert_eq!(playlist.canciones[1].titulo, "A");
        assert_eq!(playlist.canciones[2].titulo, "B");
    }

    #[test]
    fn test_mover_cancion_posicion_invalida() {
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let a = Cancion::new("A".to_string(), "ArtistaRock".to_string(), Genero::Rock);
        let b = Cancion::new("B".to_string(), "ArtistaPop".to_string(), Genero::Pop);
        let c = Cancion::new("C".to_string(), "ArtistaJazz".to_string(), Genero::Jazz);

        playlist.agregar_cancion(a);
        playlist.agregar_cancion(b);
        playlist.agregar_cancion(c.clone());

        assert_eq!(playlist.mover_cancion(&c, 10), false);
        assert_eq!(playlist.canciones[0].titulo, "A");
        assert_eq!(playlist.canciones[1].titulo, "B");
        assert_eq!(playlist.canciones[2].titulo, "C");
    }

    #[test]
    fn test_guardar_en_archivo() {
        use std::fs;

        let cancion = Cancion::new("Cancion".to_string(), "Artista".to_string(), Genero::Rock);
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        playlist.agregar_cancion(cancion.clone());

        let path = "test_playlist.json";
        // Guardar en archivo
        let res = playlist.guardar_en_archivo(path);
        assert!(res.is_ok());

        // Leer el archivo y verificar que contiene la canción
        let contenido = fs::read_to_string(path).unwrap();
        assert!(contenido.contains("Cancion"));
        assert!(contenido.contains("Artista"));

        // Limpieza
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_guardar_en_archivo_path_error() {
        // Intentar guardar en un path inválido
        let playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let res = playlist.guardar_en_archivo("/no_existe/test_playlist.json");
        assert!(res.is_err());
    }
    #[test]
    fn test_agregar_cancion_y_guardar() {
        use std::fs;
        let cancion = Cancion::new("Cancion".to_string(), "Artista".to_string(), Genero::Rock);
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let path = "test_playlist_guardar.json";
        let res = playlist.agregar_cancion_y_guardar(cancion.clone(), path);
        assert!(res.is_ok());
        let contenido = fs::read_to_string(path).unwrap();
        assert!(contenido.contains("Cancion"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_eliminar_cancion_y_guardar() {
        use std::fs;
        let cancion = Cancion::new("Cancion".to_string(), "Artista".to_string(), Genero::Rock);
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![cancion.clone()]);
        let path = "test_playlist_eliminar.json";
        // Guardar primero para crear el archivo
        playlist.guardar_en_archivo(path).unwrap();
        let res = playlist.eliminar_cancion_y_guardar(&cancion, path);
        assert!(res.is_ok());
        let contenido = fs::read_to_string(path).unwrap();
        assert!(contenido.contains("[]")); // Debe estar vacío
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_eliminar_canciones_y_guardar() {
        use std::fs;
        let c1 = Cancion::new("A".to_string(), "Artista".to_string(), Genero::Rock);
        let c2 = Cancion::new("B".to_string(), "Artista".to_string(), Genero::Pop);
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![c1, c2]);
        let path = "test_playlist_eliminar_todas.json";
        playlist.guardar_en_archivo(path).unwrap();
        let res = playlist.eliminar_canciones_y_guardar(path);
        assert!(res.is_ok());
        let contenido = fs::read_to_string(path).unwrap();
        assert!(contenido.contains("[]"));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_mover_cancion_y_guardar() {
        use std::fs;
        let a = Cancion::new("A".to_string(), "Artista".to_string(), Genero::Rock);
        let b = Cancion::new("B".to_string(), "Artista".to_string(), Genero::Pop);
        let c = Cancion::new("C".to_string(), "Artista".to_string(), Genero::Jazz);
        let mut playlist = Playlist::new(
            "Mi playlist".to_string(),
            vec![a.clone(), b.clone(), c.clone()],
        );
        let path = "test_playlist_mover.json";
        playlist.guardar_en_archivo(path).unwrap();
        let res = playlist.mover_cancion_y_guardar(&c, 0, path);
        assert!(res.is_ok());
        let contenido = fs::read_to_string(path).unwrap();
        // La primera canción debe ser "C"
        assert!(contenido.find("\"C\"").unwrap() < contenido.find("\"A\"").unwrap());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_set_nombre_y_guardar() {
        use std::fs;
        let mut playlist = Playlist::new("Viejo nombre".to_string(), vec![]);
        let path = "test_playlist_nombre.json";
        playlist.guardar_en_archivo(path).unwrap();
        let res = playlist.set_nombre_y_guardar("Nuevo nombre".to_string(), path);
        assert!(res.is_ok());
        // El nombre no se guarda en el archivo porque solo se serializan canciones,
        // pero el método debe ejecutarse sin error.
        let _ = fs::remove_file(path);
    }
    #[test]
    fn test_guardar_en_archivo_error() {
        // Usamos un path inválido para forzar el error (por ejemplo, un directorio que no existe)
        let playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let res = playlist.guardar_en_archivo("/no_existe/test_playlist.json");
        match res {
            Err(ResultadoOperacion::ErrorArchivo(_)) => assert!(true),
            _ => panic!("Se esperaba un error de archivo"),
        }
    }
    #[test]
    fn test_eliminar_cancion_y_guardar_no_encontrada() {
        use std::fs;
        let cancion = Cancion::new("Cancion".to_string(), "Artista".to_string(), Genero::Rock);
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![]);
        let path = "test_playlist_eliminar_no_encontrada.json";
        // Guardar primero para crear el archivo
        playlist.guardar_en_archivo(path).unwrap();
        let res = playlist.eliminar_cancion_y_guardar(&cancion, path);
        match res {
            Err(ResultadoOperacion::Fallo(msg)) => assert!(msg.contains("no encontrada")),
            _ => panic!("Se esperaba un error de canción no encontrada"),
        }
        let _ = fs::remove_file(path);
    }
    #[test]
    fn test_mover_cancion_y_guardar_falla() {
        use std::fs;
        let a = Cancion::new("A".to_string(), "Artista".to_string(), Genero::Rock);
        let b = Cancion::new("B".to_string(), "Artista".to_string(), Genero::Pop);
        let mut playlist = Playlist::new("Mi playlist".to_string(), vec![a.clone(), b.clone()]);
        let path = "test_playlist_mover_falla.json";
        playlist.guardar_en_archivo(path).unwrap();

        // Intentar mover una canción que no existe
        let c = Cancion::new("C".to_string(), "Artista".to_string(), Genero::Jazz);
        let res = playlist.mover_cancion_y_guardar(&c, 0, path);
        match res {
            Err(ResultadoOperacion::Fallo(msg)) => assert!(msg.contains("No se pudo mover")),
            _ => panic!("Se esperaba un error de mover canción"),
        }

        // Intentar mover una canción a una posición inválida
        let res2 = playlist.mover_cancion_y_guardar(&a, 10, path);
        match res2 {
            Err(ResultadoOperacion::Fallo(msg)) => assert!(msg.contains("No se pudo mover")),
            _ => panic!("Se esperaba un error de mover canción"),
        }

        let _ = fs::remove_file(path);
    }
}
