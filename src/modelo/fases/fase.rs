use crate::Partido;

pub trait Fase {
    fn get_partido_actual(&self) -> &Partido;
    fn siguiente_partido(&mut self) -> bool;
    fn partido_anterior(&mut self) -> bool;
}

pub enum TipoFaseEliminacion {
    Dieciseisavos,
    Octavos,
    Cuartos,
    Semifinales,
    TercerPuesto,
    Final,
}

impl TipoFaseEliminacion {
    pub fn siguiente(&self) -> Option<Self> {
        match self {
            Self::Dieciseisavos => Some(Self::Octavos),
            Self::Octavos => Some(Self::Cuartos),
            Self::Cuartos => Some(Self::Semifinales),
            Self::Semifinales => Some(Self::TercerPuesto),
            Self::TercerPuesto => Some(Self::Final),
            Self::Final => None,
        }
    }

    pub fn anterior(&self) -> Option<Self> {
        match self {
            Self::Dieciseisavos => None,
            Self::Octavos => Some(Self::Dieciseisavos),
            Self::Cuartos => Some(Self::Octavos),
            Self::Semifinales => Some(Self::Cuartos),
            Self::TercerPuesto => Some(Self::Semifinales),
            Self::Final => Some(Self::TercerPuesto),
        }
    }
}
