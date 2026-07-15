pub mod app;
pub mod common;
pub mod config;
pub mod controladores;
pub mod modelo;
pub mod renderizacion;
pub mod templates;

pub use common::{
    eventos, mvc,
    ventanas::{self, VentanaInfo},
};

pub use modelo::{
    equipo::{
        equipo_participante::EquipoParticipante,
        equipo_participante_ref::EquipoParticipanteRef,
        info_equipo::{Equipo, EquipoInfo},
    },
    fases::{
        fase::{Fase, TipoFaseEliminacion},
        fase_eliminacion::FaseDeEliminacionDirecta,
        fase_grupos::FaseDeGrupos,
    },
    global::ModeloSimulador,
    grupo::Grupo,
    mundial::Mundial,
    partidos::{
        gol::Gol,
        partido::Partido,
        simulador::SimuladorDePartidos,
        tiempos::{MinutoDePartido, TiempoDelPartido},
    },
    resultados::ResultadoEquipo,
};
