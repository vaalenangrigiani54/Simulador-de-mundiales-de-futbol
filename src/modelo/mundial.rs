use crate::{Partido, Grupo};

pub struct Mundial {
    pub cantidad_participantes: u16,
    pub equipos_por_grupo: u16,
    pub grupos: Vec<Grupo>,
    pub partidos_fase_grupos: Vec<Partido>,
    pub partidos_fase_eliminacion: Vec<Partido>
}


impl Mundial {
    pub fn new(grupos: Vec<Grupo>) -> Result<Self, String> {
        let equipos_por_grupo = grupos.first().ok_or("No hay grupos")?.cantidad_equipos();

        Ok(Self {
            cantidad_participantes: (equipos_por_grupo * grupos.len()) as u16,
            equipos_por_grupo: equipos_por_grupo as u16,
            grupos,
            partidos_fase_grupos: Vec::new(),
            partidos_fase_eliminacion: Vec::new()
        })
    }
}