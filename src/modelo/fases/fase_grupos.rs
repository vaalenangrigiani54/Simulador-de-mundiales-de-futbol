use std::usize;

use crate::{Fase, Grupo, Partido};

pub struct FaseDeGrupos {
    /// Por cada grupo, se definen en orden los partidos que se juegan.
    /// El campo booleano indica si el partido ya fue simulado o todavía no.
    partidos: Vec<Vec<Partido>>,

    /// (Grupo actual, Partido actual)
    partido_actual: Option<(usize, usize)>
}


impl FaseDeGrupos {
    pub fn new(grupos: &Vec<Grupo>) -> Self {
        Self {
            partidos: generar_partidos(grupos),
            partido_actual: None
        }
    }

    pub fn get_idx_partido_actual(&self) -> (usize, usize) {
        self.partido_actual.unwrap_or_default()
    }
}


impl Fase for FaseDeGrupos {
    fn get_partido_actual(&self) -> &Partido {
        let idx = self.get_idx_partido_actual();
        &self.partidos[idx.0][idx.1]
    }

    fn siguiente_partido(&mut self) -> bool {
        if let Some((mut idx_grupo, mut idx_partido)) = self.partido_actual {
            let Some(partidos_grupo_actual) = self.partidos.get(idx_grupo) else {
                return false;
            };

            idx_partido += 1;
            if idx_partido >= partidos_grupo_actual.len() {
                idx_grupo += 1;
                idx_partido = 0;
            }

            self.partido_actual = Some((idx_grupo, idx_partido));
            if idx_grupo >= self.partidos.len() {
                return false;
            }
        } else {
            self.partido_actual = Some((0, 0));
        }

        let (idx_grupo, idx_partido) = self.partido_actual.unwrap_or_default();
        let partido = &mut self.partidos[idx_grupo][idx_partido];
        partido.simular();

        true
    }

    fn partido_anterior(&mut self) -> bool {
        if let Some((mut idx_grupo, mut idx_partido)) = self.partido_actual {
            if idx_partido == 0 {
                if idx_grupo == 0 {
                    self.partido_actual = None;
                    return false;
                }

                idx_grupo -= 1;
            } else {
                idx_partido -= 1;
            }

            self.partido_actual = Some((idx_grupo, idx_partido));
        } else {
            return false;
        }

        true
    }
}


fn generar_partidos(grupos: &Vec<Grupo>) -> Vec<Vec<Partido>> {
    let partidos = Vec::new();

    partidos
}