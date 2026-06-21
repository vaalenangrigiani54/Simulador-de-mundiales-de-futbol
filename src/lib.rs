pub mod app;
pub mod config;
pub mod controladores;
pub mod eventos;
pub mod modelo;
pub mod mvc;
pub mod templates;
pub mod vistas;

pub use mvc::{Controlador, ControladorObj, Ventana};

pub use modelo::{
    equipo::{
        equipo_participante::EquipoParticipante,
        equipo_participante_ref::EquipoParticipanteRef,
        info_equipo::{
            Equipo,
            EquipoInfo
        }
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

pub use vistas::ventanas::{
    self,
    VentanaInfo
};
