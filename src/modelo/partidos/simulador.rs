use rand::{RngExt, rngs::ThreadRng};

use crate::{Gol, MinutoDePartido, TiempoDelPartido};

const PROBABILIDAD_GOL: f64 = 0.15;
const PROBABILIDAD_GOL_PENAL: f64 = 0.25;
const PROBABILIDAD_AMARILLA: f64 = 0.08;
const PROBABILIDAD_ROJA: f64 = 0.01;


/// Wrapper para generar partidos de manera un poco más realista.
pub struct SimuladorDePartidos {
    random: ThreadRng,
    tiempo_actual: TiempoDelPartido,
    minuto_actual: MinutoDePartido,
}


impl SimuladorDePartidos {
    pub fn inicializar() -> Self {
        Self {
            random: rand::rng(),
            tiempo_actual: TiempoDelPartido::PrimerTiempo,
            minuto_actual: 0
        }
    }

    pub fn next_gol(&mut self, goles: &mut Vec<Gol>) {
        if self.random.random_bool(PROBABILIDAD_GOL) {
            if self.random.random_bool(PROBABILIDAD_GOL_PENAL) {
                goles.push(Gol::de_penal(self.tiempo_actual, self.minuto_actual));
            } else {
                goles.push(Gol::normal(self.tiempo_actual, self.minuto_actual));
            }
        }
    }

    pub fn next_amarilla(&mut self, amarillas: &mut u16) {
        if self.random.random_bool(PROBABILIDAD_AMARILLA) {
            *amarillas += 1;
        }
    }

    pub fn next_roja(&mut self, rojas: &mut u16) {
        if self.random.random_bool(PROBABILIDAD_ROJA) {
            *rojas += 1;
        }
    }

    pub fn siguiente_minuto(&mut self) -> bool {
        if self.minuto_actual > self.tiempo_actual.duracion() + 10 {
            return false;
        }

        self.minuto_actual += 1;
        true
    }

    pub fn siguiente_tiempo(&mut self) {
        self.tiempo_actual = self.tiempo_actual.siguiente();
        self.minuto_actual = 0;
    }
}