pub mod app;
pub mod config;
pub mod modelo;
pub mod mvc;
pub mod vistas;

pub use mvc::{Vista, VistaObj};

pub use modelo::{
    equipo::{Equipo, EquipoParticipante, EquipoParticipanteRef},
    fases::{
        fase::{Fase, TipoFaseEliminacion},
        fase_eliminacion::FaseDeEliminacionDirecta,
        fase_grupos::FaseDeGrupos,
    },
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

pub use vistas::menu_principal::MenuPrincipal;
