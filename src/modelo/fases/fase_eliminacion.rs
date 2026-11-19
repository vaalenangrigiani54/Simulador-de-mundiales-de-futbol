use crate::{EquipoParticipanteRef, Fase, Partido, TipoFaseEliminacion};

pub struct FaseDeEliminacionDirecta {
    pub tipo: TipoFaseEliminacion,
    partidos: Vec<Partido>,
    partido_actual: Option<usize>,
}

impl FaseDeEliminacionDirecta {
    /// Genera cruces independientes entre parejas consecutivas.
    ///
    /// # Panics
    ///
    /// En caso de recibir una cantidad impar de equipos.
    pub fn new(tipo: TipoFaseEliminacion, equipos: Vec<EquipoParticipanteRef>) -> Self {
        Self {
            tipo,
            partidos: generar_partidos(equipos),
            partido_actual: None,
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
        let siguiente = self.partido_actual.map_or(0, |idx| idx + 1);
        let Some(partido) = self.partidos.get_mut(siguiente) else {
            return false;
        };

        self.partido_actual = Some(siguiente);
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
    assert!(equipos.len().is_multiple_of(2), "La cantidad de equipos debe ser par");
    equipos
        .as_chunks::<2>().0.iter()
        .map(|pareja| Partido::new(pareja[0].clone(), pareja[1].clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Equipo, EquipoInfo};

    fn equipos(n: usize) -> Vec<EquipoParticipanteRef> {
        (0..n)
            .map(|i| EquipoParticipanteRef::from(Equipo::new(
                format!("Equipo {i}"), format!("EQ{i}"),
            )))
            .collect()
    }

    #[test]
    fn se_generan_parejas_sin_superposiciones() {
        let mut fase = FaseDeEliminacionDirecta::new(TipoFaseEliminacion::Octavos, equipos(4));
        assert!(fase.siguiente_partido());
        assert_eq!(fase.get_partido_actual().get_local().get_codigo().as_str(), "EQ0");
        assert_eq!(fase.get_partido_actual().get_visitante().get_codigo().as_str(), "EQ1");
        assert!(fase.siguiente_partido());
        assert_eq!(fase.get_partido_actual().get_local().get_codigo().as_str(), "EQ2");
        assert_eq!(fase.get_partido_actual().get_visitante().get_codigo().as_str(), "EQ3");
        assert!(!fase.siguiente_partido());
        assert_eq!(fase.get_idx_partido_actual(), 1);
    }

    #[test]
    fn una_fase_vacia_no_intenta_simular_partidos() {
        let mut fase = FaseDeEliminacionDirecta::new(TipoFaseEliminacion::Final, equipos(0));
        assert!(!fase.siguiente_partido());
    }

    #[test]
    #[should_panic(expected = "La cantidad de equipos debe ser par")]
    fn se_rechazan_los_participantes_impares() {
        FaseDeEliminacionDirecta::new(TipoFaseEliminacion::Octavos, equipos(3));
    }
}
