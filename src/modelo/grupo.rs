use crate::EquipoParticipanteRef;

pub struct Grupo {
    id: usize,
    equipos: Vec<(usize, EquipoParticipanteRef)>
}


impl Grupo {
    pub fn try_new(id: usize, equipos: Vec<EquipoParticipanteRef>) -> Result<Self, String> {
        let num_equipos = equipos.len();
        if num_equipos < 3 {
            return Err("El equipo debe tener al menos 3 participantes".to_string());
        }

        let mut bombo = 1;
        let mut equipos_bombo = Vec::with_capacity(num_equipos);

        for equipo in equipos {
            equipos_bombo.push((bombo, equipo));
            bombo += 1;
        }
        
        Ok(Self {
            id,
            equipos: equipos_bombo
        })
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_equipos(&self) -> &Vec<(usize, EquipoParticipanteRef)> {
        &self.equipos
    }

    pub fn cantidad_equipos(&self) -> usize {
        self.equipos.len()
    }
}