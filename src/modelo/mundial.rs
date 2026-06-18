use crate::{FaseDeEliminacionDirecta, FaseDeGrupos, Grupo};

pub struct Mundial {
    pub cantidad_participantes: u16,
    pub equipos_por_grupo: u16,
    pub grupos: Vec<Grupo>,
    pub fase_grupos: FaseDeGrupos,
    pub fases_eliminacion: Vec<FaseDeEliminacionDirecta>,
}

impl Mundial {
    pub fn new(grupos: Vec<Grupo>) -> Result<Self, String> {
        let equipos_por_grupo = grupos.first().ok_or("No hay grupos")?.cantidad_equipos();
        let fase_grupos = FaseDeGrupos::new(&grupos);

        Ok(Self {
            cantidad_participantes: (equipos_por_grupo * grupos.len()) as u16,
            equipos_por_grupo: equipos_por_grupo as u16,
            grupos,
            fase_grupos,
            fases_eliminacion: Vec::new(),
        })
    }
}
