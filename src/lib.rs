pub mod app;
pub mod config;
pub mod controladores;
pub mod eventos;
pub mod modelo;
pub mod mvc;
pub mod templates;
pub mod vistas;

pub use eventos::{EventoGlobal, EventoMenuPrincipal};

pub use mvc::{Controlador, ControladorObj, Ventana, VentanaInfo};

pub use modelo::{
    equipo::{Equipo, EquipoParticipante, EquipoParticipanteRef},
    fases::{
        fase::{Fase, TipoFaseEliminacion},
        fase_eliminacion::FaseDeEliminacionDirecta,
        fase_grupos::FaseDeGrupos,
    },
    grupo::Grupo,
    modelo::ModeloSimulador,
    mundial::Mundial,
    partidos::{
        gol::Gol,
        partido::Partido,
        simulador::SimuladorDePartidos,
        tiempos::{MinutoDePartido, TiempoDelPartido},
    },
    resultados::ResultadoEquipo,
};

pub use vistas::ventanas::MenuPrincipal;
