use std::{
    cell::Ref,
    fmt::{Debug, Display},
};

use crate::{
    EquipoParticipante, EquipoParticipanteRef, Gol, SimuladorDePartidos, modelo::equipo::IDEquipo,
};

/// Estructura de un partido de fútbol, con los equipos local y visitante y sus resultados.
pub struct Partido {
    local: EquipoParticipanteRef,
    visitante: EquipoParticipanteRef,
    goles_local: Vec<Gol>,
    goles_visitante: Vec<Gol>,
    amarillas_local: u16,
    amarillas_visitante: u16,
    rojas_local: u16,
    rojas_visitante: u16,
    ya_simulado: bool,
}

impl Partido {
    pub fn new(local: EquipoParticipanteRef, visitante: EquipoParticipanteRef) -> Self {
        Self {
            local,
            visitante,
            goles_local: Vec::with_capacity(2),
            goles_visitante: Vec::with_capacity(2),
            amarillas_local: 0,
            amarillas_visitante: 0,
            rojas_local: 0,
            rojas_visitante: 0,
            ya_simulado: false,
        }
    }

    pub fn simular(&mut self) {
        if self.ya_simulado {
            return;
        }

        let mut simulador = SimuladorDePartidos::inicializar();

        // PRIMER Y SEGUNDO TIEMPO
        for _ in 0..2 {
            while simulador.siguiente_minuto() {
                simulador.next_gol(&mut self.goles_local);
                simulador.next_gol(&mut self.goles_visitante);
                simulador.next_amarilla(&mut self.amarillas_local);
                simulador.next_amarilla(&mut self.amarillas_visitante);
                simulador.next_roja(&mut self.rojas_local);
                simulador.next_roja(&mut self.rojas_visitante);
            }

            simulador.siguiente_tiempo();
        }

        self.ya_simulado = true;
    }

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
        let mut local = self.local.borrow_mut();
        let mut visitante = self.visitante.borrow_mut();

        match self.get_ganador_perdedor() {
            Some((ganador, perdedor)) => {
                ganador.borrow_mut().sumar_partido_ganado(avanzar);
                perdedor.borrow_mut().sumar_partido_perdido(avanzar);
            }
            None => {
                local.sumar_partido_empatado(avanzar);
                visitante.sumar_partido_empatado(avanzar);
            }
        }

        let cant_goles_local = self.goles_local.len();
        let cant_goles_visitante = self.goles_visitante.len();

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

impl Display for Partido {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} vs. {}",
            self.get_local().get_nombre(),
            self.get_visitante().get_nombre()
        )
    }
}

impl Debug for Partido {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self}\n")?;

        writeln!(f, "({}):", self.get_local().get_codigo())?;

        write!(f, "{} goles: ", self.goles_local.len())?;
        for gol in &self.goles_local {
            write!(f, "[ {gol} ]")?;
        }

        writeln!(f, "\n{} tarjetas amarillas.", self.amarillas_local)?;
        writeln!(f, "{} tarjetas rojas.", self.rojas_local)?;

        writeln!(f, "\n({}):", self.get_visitante().get_codigo())?;

        write!(f, "{} goles: ", self.goles_visitante.len())?;
        for gol in &self.goles_visitante {
            write!(f, "[ {gol} ]")?;
        }

        writeln!(f, "\n{} tarjetas amarillas.", self.amarillas_visitante)?;
        writeln!(f, "{} tarjetas rojas.", self.rojas_visitante)
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use super::*;
    use crate::Equipo;

    #[test]
    fn test_simulacion() {
        let equipo_loc = Rc::new(RefCell::new(Equipo::new("Argentina", "ARG").clasificarse()));
        let equipo_vis = Rc::new(RefCell::new(Equipo::new("Francia", "FRA").clasificarse()));

        let mut partido = Partido::new(equipo_loc, equipo_vis);
        partido.simular();

        println!("{:?}", partido);
    }
}
