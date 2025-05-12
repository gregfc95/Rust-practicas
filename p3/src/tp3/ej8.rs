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
#[derive(Debug, PartialEq, Clone)]
pub enum Genero {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

pub struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

pub struct Playlist {
    nombre: String,
    canciones: Vec<Cancion>,
}

impl Playlist {
    pub fn new(nombre: String) -> Self {
        Playlist {
            nombre,
            canciones: Vec::new(),
        }
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
            c.titulo == cancion.titulo && c.artista == cancion.artista && c.genero == cancion.genero
        })?;
        Some(self.canciones.remove(index))
    }
    pub fn buscar_cancion_por_nombre(&self, titulo: &str) -> Option<&Cancion> {
        self.canciones.iter().find(|c| c.titulo == titulo)
    }

    pub fn obtener_canciones_por_genero(&self, genero_buscado: Genero) -> Vec<&Cancion> {
        self.canciones
            .iter()
            .filter(|c| c.genero == genero_buscado)
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
            c.titulo == cancion.titulo && c.artista == cancion.artista && c.genero == cancion.genero
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
