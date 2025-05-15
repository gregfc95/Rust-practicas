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
*/
#[derive(Debug, Clone)]
pub enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}
#[derive(Debug, Clone)]
pub struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

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
}
