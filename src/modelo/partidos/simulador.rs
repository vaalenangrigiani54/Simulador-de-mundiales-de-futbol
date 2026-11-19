use rand::{RngExt, rngs::StdRng};

use crate::{Gol, MinutoDePartido, TiempoDelPartido};

const MAX_GOLES: u16 = 10;
const MAX_AMARILLAS: u16 = 12;
const MAX_ROJAS: u16 = 8;

const PROBABILIDAD_GOL: f64 = 0.01;
const PROBABILIDAD_GOL_PENAL: f64 = 0.1;
const PROBABILIDAD_AMARILLA: f64 = 0.05;
const PROBABILIDAD_ROJA: f64 = 0.005;

/// Wrapper para generar partidos de manera un poco más realista.
pub struct SimuladorDePartidos {
    random: StdRng,
    tiempo_actual: TiempoDelPartido,
    minuto_actual: MinutoDePartido,
}

impl SimuladorDePartidos {
    pub fn inicializar() -> Self {
        Self {
            random: rand::make_rng(),
            tiempo_actual: TiempoDelPartido::PrimerTiempo,
            minuto_actual: 0,
        }
    }

    pub fn next_gol(&mut self, goles: &mut Vec<Gol>) {
        if goles.len() < usize::from(MAX_GOLES) && self.random.random_bool(PROBABILIDAD_GOL) {
            if self.random.random_bool(PROBABILIDAD_GOL_PENAL) {
                goles.push(Gol::de_penal(self.tiempo_actual, self.minuto_actual));
            } else {
                goles.push(Gol::normal(self.tiempo_actual, self.minuto_actual));
            }
        }
    }

    pub fn next_amarilla(&mut self, amarillas: &mut u16) {
        if *amarillas < MAX_AMARILLAS && self.random.random_bool(PROBABILIDAD_AMARILLA) {
            *amarillas += 1;
        }
    }

    pub fn next_roja(&mut self, rojas: &mut u16) {
        if *rojas < MAX_ROJAS && self.random.random_bool(PROBABILIDAD_ROJA) {
            *rojas += 1;
        }
    }

    pub fn siguiente_minuto(&mut self) -> bool {
        if self.minuto_actual >= self.tiempo_actual.duracion() + 10 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn los_limites_de_eventos_no_se_superan() {
        let mut simulador = SimuladorDePartidos::inicializar();
        let mut goles = (0..MAX_GOLES)
            .map(|_| Gol::normal(TiempoDelPartido::PrimerTiempo, 1))
            .collect::<Vec<_>>();
        let mut amarillas = MAX_AMARILLAS;
        let mut rojas = MAX_ROJAS;

        for _ in 0..100 {
            simulador.next_gol(&mut goles);
            simulador.next_amarilla(&mut amarillas);
            simulador.next_roja(&mut rojas);
        }

        assert_eq!(goles.len(), usize::from(MAX_GOLES));
        assert_eq!(amarillas, MAX_AMARILLAS);
        assert_eq!(rojas, MAX_ROJAS);
    }

    #[test]
    fn el_tiempo_no_excede_la_duracion_mas_diez_minutos() {
        let mut simulador = SimuladorDePartidos::inicializar();
        for _ in 0..55 {
            assert!(simulador.siguiente_minuto());
        }
        assert!(!simulador.siguiente_minuto());
    }
}
