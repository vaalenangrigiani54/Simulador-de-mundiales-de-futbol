use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::{EquipoParticipante, EquipoParticipanteRef, Gol};

/// Estructura de un partido de fútbol, con los equipos local y visitante y sus resultados.
pub struct Partido {
    local: Rc<RefCell<EquipoParticipante>>,
    visitante: Rc<RefCell<EquipoParticipante>>,
    goles_local: Vec<Gol>,
    goles_visitante: Vec<Gol>,
    amarillas_local: u16,
    amarillas_visitante: u16,
    rojas_local: u16,
    rojas_visitante: u16,
}

impl Partido {
    pub fn new(
        local: Rc<RefCell<EquipoParticipante>>,
        visitante: Rc<RefCell<EquipoParticipante>>,
    ) -> Self {
        Self {
            local,
            visitante,
            goles_local: Vec::with_capacity(2),
            goles_visitante: Vec::with_capacity(2),
            amarillas_local: 0,
            amarillas_visitante: 0,
            rojas_local: 0,
            rojas_visitante: 0,
        }
    }

    pub fn simular(&mut self) {}

    pub fn avanzar_resultados_a_equipos(&self) {
        self.resultados_avanzar_o_retroceder(true);
    }

    pub fn retroceder_resultados_a_equipos(&self) {
        self.resultados_avanzar_o_retroceder(false);
    }

    pub fn get_local(&self) -> Ref<'_, EquipoParticipante> {
        self.local.borrow()
    }

    pub fn get_visitante(&self) -> Ref<'_, EquipoParticipante> {
        self.visitante.borrow()
    }

    pub fn get_ganador_perdedor(&self) -> Option<(EquipoParticipanteRef, EquipoParticipanteRef)> {
        let cant_goles_local = self.goles_local.len();
        let cant_goles_visitante = self.goles_visitante.len();
        let local = self.local.clone();
        let visitante = self.visitante.clone();
        
        if cant_goles_local > cant_goles_visitante {
            Some((local, visitante))
        } else if cant_goles_local < cant_goles_visitante {
            Some((visitante, local))
        } else {
            None
        }
    }

    pub fn get_goles_local(&self) -> &Vec<Gol> {
        &self.goles_local
    }

    pub fn get_goles_visitante(&self) -> &Vec<Gol> {
        &self.goles_visitante
    }

    pub fn get_amarillas_local(&self) -> u16 {
        self.amarillas_local
    }

    pub fn get_amarillas_visitante(&self) -> u16 {
        self.amarillas_visitante
    }

    pub fn get_rojas_local(&self) -> u16 {
        self.rojas_local
    }

    pub fn get_rojas_visitante(&self) -> u16 {
        self.rojas_visitante
    }

    fn resultados_avanzar_o_retroceder(&self, avanzar: bool) {
        let cant_goles_local = self.goles_local.len();
        let cant_goles_visitante = self.goles_visitante.len();
        let mut local = self.local.borrow_mut();
        let mut visitante = self.visitante.borrow_mut();

        if cant_goles_local > cant_goles_visitante {
            local.sumar_partido_ganado(avanzar);
            visitante.sumar_partido_perdido(avanzar);
        } else if cant_goles_local < cant_goles_visitante {
            local.sumar_partido_perdido(avanzar);
            visitante.sumar_partido_ganado(avanzar);
        } else {
            local.sumar_partido_empatado(avanzar);
            visitante.sumar_partido_empatado(avanzar);
        }

        local.sumar_goles_a_favor(cant_goles_local as u16, avanzar);
        local.sumar_goles_en_contra(cant_goles_visitante as u16, avanzar);
        local.sumar_tarjetas_amarillas(self.amarillas_local, avanzar);
        local.sumar_tarjetas_rojas(self.rojas_local, avanzar);
        visitante.sumar_goles_a_favor(cant_goles_visitante as u16, avanzar);
        visitante.sumar_goles_en_contra(cant_goles_local as u16, avanzar);
        visitante.sumar_tarjetas_amarillas(self.amarillas_visitante, avanzar);
        visitante.sumar_tarjetas_rojas(self.rojas_visitante, avanzar);
    }
}
