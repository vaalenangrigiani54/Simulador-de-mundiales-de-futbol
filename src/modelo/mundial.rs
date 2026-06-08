use crate::{EquipoParticipanteRef, FechaDelTorneo, Partido, modelo::grupo::Grupo};

pub struct Mundial {
    pub cantidad_participantes: u16,
    pub equipos_por_grupo: u16,
    pub grupos: Vec<Grupo>,
    pub fechas: Vec<FechaDelTorneo>
}


impl Mundial {
    pub fn new(grupos: Vec<Grupo>) -> Result<Self, String> {
        let equipos_por_grupo = grupos.first().ok_or("No hay grupos")?.cantidad_equipos();
        let fechas = generar_partidos(&grupos);

        Ok(Self {
            cantidad_participantes: (equipos_por_grupo * grupos.len()) as u16,
            equipos_por_grupo: equipos_por_grupo as u16,
            grupos,
            fechas
        })
    }
}


fn generar_partidos(grupos: &Vec<Grupo>) -> Vec<FechaDelTorneo> {
    let fechas = Vec::new();
    let partidos_por_grupo: Vec<Vec<Partido>>;

    for grupo in grupos {
        let equipos = vector_auxiliar_equipos(grupo);
        
        if equipos.len() & 1 != 0 {
            equipos.push(None);
        }
    }

    fechas
}


fn vector_auxiliar_equipos(grupo: &Grupo) -> Vec<Option<EquipoParticipanteRef>> {
    let mut equipos_aux = Vec::with_capacity(grupo.cantidad_equipos());

    for (_, equipo) in grupo.get_equipos() {
        equipos_aux.push(Some(equipo.clone()));
    }

    equipos_aux
}