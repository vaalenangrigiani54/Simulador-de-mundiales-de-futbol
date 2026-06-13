use crate::{Grupo, Partido};

pub struct FaseDeGrupos {
    /// Por cada grupo, se definen en orden los partidos que se juegan.
    /// El campo booleano indica si el partido ya fue simulado o todavía no.
    partidos: Vec<Vec<(Partido, bool)>>,

    /// (Grupo actual, Partido actual)
    partido_actual: (u8, u8)
}


impl FaseDeGrupos {
    pub fn new(grupos: &Vec<Grupo>) -> Self {
        Self {
            partidos: generar_partidos(grupos),
            partido_actual: (0, 0)
        }
    }

    pub fn get_partido_actual(&self) -> &Partido {
        let idx_grupo = self.partido_actual.0 as usize;
        let idx_partido = self.partido_actual.1 as usize;
        &self.partidos[idx_grupo][idx_partido].0
    }

    pub fn get_idx_partido_actual(&self) -> (u8, u8) {
        self.partido_actual
    }
}


fn generar_partidos(grupos: &Vec<Grupo>) -> Vec<Vec<(Partido, bool)>> {
    let partidos = Vec::new();

    partidos
}