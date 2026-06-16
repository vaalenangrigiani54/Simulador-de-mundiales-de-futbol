pub mod app;
pub mod modelo;
pub mod vistas;

pub use modelo::{
    fases::{
        fase::{
            Fase,
            TipoFaseEliminacion
        },
        fase_eliminacion::FaseDeEliminacionDirecta,
        fase_grupos::FaseDeGrupos
    },
    partidos::{
        gol::Gol,
        partido::Partido,
        simulador::SimuladorDePartidos,
        tiempos::{
            MinutoDePartido,
            TiempoDelPartido
        }
    },
    equipo::{
        Equipo,
        EquipoParticipante,
        EquipoParticipanteRef
    },
    grupo::Grupo,
    mundial::Mundial,
    resultados::ResultadoEquipo
};

pub use vistas::{
    menu_principal::VistaMenuPrincipal,
    vista::{
        Vista,
        VistaObj
    }
};