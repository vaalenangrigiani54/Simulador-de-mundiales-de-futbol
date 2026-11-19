use crate::EquipoParticipanteRef;

pub struct Grupo {
    id: u8,
    equipos: Vec<(usize, EquipoParticipanteRef)>,
}

impl Grupo {
    pub fn try_new(id: u8, equipos: Vec<EquipoParticipanteRef>) -> Result<Self, String> {
        let num_equipos = equipos.len();
        if num_equipos < 3 {
            return Err("El grupo debe tener al menos 3 participantes".to_string());
        }

        let equipos_bombo = (1..).zip(equipos).collect();

        Ok(Self {
            id,
            equipos: equipos_bombo,
        })
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_equipos(&self) -> &Vec<(usize, EquipoParticipanteRef)> {
        &self.equipos
    }

    pub fn cantidad_equipos(&self) -> usize {
        self.equipos.len()
    }
}
