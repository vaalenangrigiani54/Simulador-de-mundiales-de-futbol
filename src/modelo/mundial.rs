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
        if grupos.iter().any(|grupo| grupo.cantidad_equipos() != equipos_por_grupo) {
            return Err("Todos los grupos deben tener la misma cantidad de equipos".into());
        }

        let cantidad_participantes = grupos.len()
            .checked_mul(equipos_por_grupo)
            .and_then(|total| u16::try_from(total).ok())
            .ok_or("La cantidad de participantes excede el límite permitido")?;
        let equipos_por_grupo = u16::try_from(equipos_por_grupo)
            .map_err(|_| "La cantidad de equipos por grupo excede el límite permitido")?;
        let fase_grupos = FaseDeGrupos::new(&grupos);

        Ok(Self {
            cantidad_participantes,
            equipos_por_grupo,
            grupos,
            fase_grupos,
            fases_eliminacion: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Equipo, EquipoParticipanteRef};

    fn grupo(id: u8, cantidad: usize) -> Grupo {
        let equipos = (0..cantidad)
            .map(|i| EquipoParticipanteRef::from(Equipo::new(
                format!("Equipo {id}-{i}"), format!("E{id}{i}"),
            )))
            .collect();
        Grupo::try_new(id, equipos).unwrap()
    }

    #[test]
    fn rechaza_grupos_de_tamanios_diferentes() {
        assert!(Mundial::new(vec![grupo(0, 4), grupo(1, 3)]).is_err());
    }

    #[test]
    fn acepta_grupos_uniformes() {
        let mundial = Mundial::new(vec![grupo(0, 4), grupo(1, 4)]).unwrap();
        assert_eq!(mundial.cantidad_participantes, 8);
        assert_eq!(mundial.equipos_por_grupo, 4);
    }
}
