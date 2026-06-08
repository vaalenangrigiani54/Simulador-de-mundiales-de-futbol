use crate::{EquipoParticipanteRef, Partido};

pub enum TipoFaseEliminacion {
    Dieciseisavos,
    Octavos,
    Cuartos,
    Semifinales,
    TercerPuesto,
    Final,
}


pub struct FaseDeEliminacionDirecta {
    tipo: TipoFaseEliminacion,
    partidos: Vec<Partido>
}


impl FaseDeEliminacionDirecta {
    pub fn new(tipo: TipoFaseEliminacion, equipos: Vec<EquipoParticipanteRef>) -> Self {
        Self {
            tipo,
            partidos: generar_partidos(equipos)
        }
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