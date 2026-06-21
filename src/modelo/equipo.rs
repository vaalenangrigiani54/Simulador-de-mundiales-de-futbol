use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use crate::ResultadoEquipo;

pub trait IDEquipo {
    /// Devuelve el **nombre** del equipo (inmutable).
    fn get_nombre(&self) -> &String;

    /// Devuelve el **código FIFA** del equipo (inmutable).
    fn get_codigo(&self) -> &String;
}

/// Información general de una selección de fútbol.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Equipo {
    nombre: String,
    codigo: String,
}

/// Simula el equipo que va a participar en el mundial simulado.
///
/// ## Criterio de orden:
/// 1. Puntos (+).
/// 2. Diferencia de goles (+).
/// 3. Goles a favor (+).
/// 4. Goles en contra (-).
/// 5. Partidos ganados (+).
/// 6. Partidos empatados (-).
/// 7. Partidos perdidos (-)/
/// 8. Tarjetas amarillas (-).
/// 9. Tarjetas rojas (-).
/// 10. ID.
#[derive(PartialEq, Eq)]
pub struct EquipoParticipante {
    id: Equipo,
    puntos: u16,
    goles_a_favor: u16,
    goles_en_contra: u16,
    ganados: u16,
    empatados: u16,
    perdidos: u16,
    amarillas: u16,
    rojas: u16,
    pub posicion: ResultadoEquipo,
}

/// Múltiples referencias mutables a un equipo participante.
pub type EquipoParticipanteRef = Rc<RefCell<EquipoParticipante>>;

impl Equipo {
    pub fn new(nombre: impl Into<String>, codigo: impl Into<String>) -> Self {
        Equipo {
            nombre: nombre.into(),
            codigo: codigo.into(),
        }
    }

    /// Convierte al equipo en `EquipoParticipante`.
    ///
    /// Pierde el ownership, pero se puede volver a `Equipo` con el método para descalificarse.
    ///
    /// Ver la documentación de [`EquipoParticipante`] para más información.
    pub fn clasificarse(self) -> EquipoParticipante {
        EquipoParticipante {
            id: self,
            puntos: 0,
            goles_a_favor: 0,
            goles_en_contra: 0,
            ganados: 0,
            empatados: 0,
            perdidos: 0,
            amarillas: 0,
            rojas: 0,
            posicion: ResultadoEquipo::FaseDeGrupos,
        }
    }
}

impl EquipoParticipante {
    /// Crea el equipo clasificado a partir de `id`, el cual se clona.
    pub fn new(id: &Equipo) -> Self {
        Self {
            id: id.clone(),
            puntos: 0,
            goles_a_favor: 0,
            goles_en_contra: 0,
            ganados: 0,
            empatados: 0,
            perdidos: 0,
            amarillas: 0,
            rojas: 0,
            posicion: ResultadoEquipo::FaseDeGrupos,
        }
    }

    pub fn get_puntos(&self) -> u16 {
        self.puntos
    }

    pub fn get_diferencia_goles(&self) -> i16 {
        (self.goles_a_favor - self.goles_en_contra) as i16
    }

    pub fn get_goles_a_favor(&self) -> u16 {
        self.goles_a_favor
    }

    pub fn get_goles_en_contra(&self) -> u16 {
        self.goles_en_contra
    }

    pub fn get_partidos_ganados(&self) -> u16 {
        self.ganados
    }

    pub fn get_partidos_empatados(&self) -> u16 {
        self.empatados
    }

    pub fn get_partidos_perdidos(&self) -> u16 {
        self.perdidos
    }

    pub fn get_tarjetas_amarillas(&self) -> u16 {
        self.amarillas
    }

    pub fn get_tarjetas_rojas(&self) -> u16 {
        self.rojas
    }

    pub fn sumar_partido_ganado(&mut self, avanzar: bool) {
        if avanzar {
            self.puntos += 3;
            self.ganados += 1;
        } else {
            self.puntos -= 3;
            self.ganados -= 1;
        }
    }

    pub fn sumar_partido_empatado(&mut self, avanzar: bool) {
        if avanzar {
            self.puntos += 1;
            self.empatados += 1;
        } else {
            self.puntos -= 1;
            self.empatados -= 1;
        }
    }

    pub fn sumar_partido_perdido(&mut self, avanzar: bool) {
        if avanzar {
            self.perdidos += 1;
        } else {
            self.perdidos -= 1;
        }
    }

    pub fn sumar_goles_a_favor(&mut self, goles: u16, avanzar: bool) {
        if avanzar {
            self.goles_a_favor += goles;
        } else {
            self.goles_a_favor -= goles;
        }
    }

    pub fn sumar_goles_en_contra(&mut self, goles: u16, avanzar: bool) {
        if avanzar {
            self.goles_en_contra += goles;
        } else {
            self.goles_en_contra -= goles;
        }
    }

    pub fn sumar_tarjetas_amarillas(&mut self, amarillas: u16, avanzar: bool) {
        if avanzar {
            self.amarillas += amarillas;
        } else {
            self.amarillas -= amarillas;
        }
    }

    pub fn sumar_tarjetas_rojas(&mut self, rojas: u16, avanzar: bool) {
        if avanzar {
            self.rojas += rojas;
        } else {
            self.rojas -= rojas;
        }
    }
}

impl IDEquipo for Equipo {
    fn get_nombre(&self) -> &String {
        &self.nombre
    }

    fn get_codigo(&self) -> &String {
        &self.codigo
    }
}

impl IDEquipo for EquipoParticipante {
    fn get_nombre(&self) -> &String {
        &self.id.nombre
    }

    fn get_codigo(&self) -> &String {
        &self.id.codigo
    }
}

impl Ord for EquipoParticipante {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut ord = self.puntos.cmp(&other.puntos);
        if ord.is_ne() {
            return ord;
        }

        ord = self
            .get_diferencia_goles()
            .cmp(&other.get_diferencia_goles());
        if ord.is_ne() {
            return ord;
        }

        ord = self.goles_a_favor.cmp(&other.goles_a_favor);
        if ord.is_ne() {
            return ord;
        }

        ord = other.goles_en_contra.cmp(&self.goles_en_contra);
        if ord.is_ne() {
            return ord;
        }

        ord = self.goles_a_favor.cmp(&other.goles_a_favor);
        if ord.is_ne() {
            return ord;
        }

        ord = other.goles_en_contra.cmp(&self.goles_en_contra);
        if ord.is_ne() {
            return ord;
        }

        ord = self.ganados.cmp(&other.ganados);
        if ord.is_ne() {
            return ord;
        }

        ord = self.empatados.cmp(&other.empatados);
        if ord.is_ne() {
            return ord;
        }

        ord = other.perdidos.cmp(&self.perdidos);
        if ord.is_ne() {
            return ord;
        }

        ord = other.amarillas.cmp(&self.amarillas);
        if ord.is_ne() {
            return ord;
        }

        ord = other.rojas.cmp(&self.rojas);
        if ord.is_ne() {
            return ord;
        }

        other.id.cmp(&self.id)
    }
}

impl PartialOrd for EquipoParticipante {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
