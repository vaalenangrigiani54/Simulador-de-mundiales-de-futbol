pub mod modelo;

pub use modelo::equipo::Equipo;
pub use modelo::equipo::EquipoParticipante;
pub use modelo::equipo::EquipoParticipanteRef;

pub use modelo::fase_eliminacion::{
    FaseDeEliminacionDirecta,
    TipoFaseEliminacion
};

pub use modelo::gol::Gol;
pub use modelo::grupo::Grupo;
pub use modelo::partido::Partido;
pub use modelo::resultados::ResultadoEquipo;
pub use modelo::tiempos::TiempoDelPartido;