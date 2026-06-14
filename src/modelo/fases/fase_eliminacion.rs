use crate::{EquipoParticipanteRef, Fase, Partido, TipoFaseEliminacion};


pub struct FaseDeEliminacionDirecta {
    pub tipo: TipoFaseEliminacion,
    partidos: Vec<Partido>,
    partido_actual: Option<usize>,
}


impl FaseDeEliminacionDirecta {
    pub fn new(tipo: TipoFaseEliminacion, equipos: Vec<EquipoParticipanteRef>) -> Self {
        Self {
            tipo,
            partidos: generar_partidos(equipos),
            partido_actual: None
        }
    }

    pub fn get_idx_partido_actual(&self) -> usize {
        self.partido_actual.unwrap_or_default()
    }
}


impl Fase for FaseDeEliminacionDirecta {
    fn get_partido_actual(&self) -> &Partido {
        &self.partidos[self.get_idx_partido_actual()]
    }

    fn siguiente_partido(&mut self) -> bool {
        if let Some(mut idx_partido) = self.partido_actual {
            idx_partido += 1;
            if idx_partido >= self.partidos.len() {
                return false;
            }

            self.partido_actual = Some(idx_partido);
        } else {
            self.partido_actual = Some(0);
        }

        let idx = self.get_idx_partido_actual();
        let partido = &mut self.partidos[idx];
        partido.simular();

        true
    }

    fn partido_anterior(&mut self) -> bool {
        if let Some(idx_partido) = self.partido_actual {
            if idx_partido == 0 {
                self.partido_actual = None;
                return false;
            }

            self.partido_actual = Some(idx_partido - 1)
        } else {
            return false;
        }

        true
    }
}


fn generar_partidos(equipos: Vec<EquipoParticipanteRef>) -> Vec<Partido> {
    let mut partidos = Vec::new();

    for i in 0..equipos.len()-1 {
        let local = equipos[i].clone();
        let visitante = equipos[i + 1].clone();
        
        partidos.push(Partido::new(local, visitante));
    }

    partidos
}