pub type MinutoDePartido = u8;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TiempoDelPartido {
    PrimerTiempo,
    SegundoTiempo,
    AlarguePrimerTiempo,
    AlargueSegundoTiempo,
}

impl TiempoDelPartido {
    /// Devuelve la duración (**en minutos**) del tiempo.
    pub fn duracion(&self) -> MinutoDePartido {
        match self {
            Self::PrimerTiempo | Self::SegundoTiempo => 45,
            Self::AlarguePrimerTiempo | Self::AlargueSegundoTiempo => 15,
        }
    }

    /// Devuelve en qué minuto arranca cada tiempo.
    pub fn offset(&self) -> MinutoDePartido {
        match self {
            Self::PrimerTiempo => 0,
            Self::SegundoTiempo => 45,
            Self::AlarguePrimerTiempo => 90,
            Self::AlargueSegundoTiempo => 105,
        }
    }

    pub fn siguiente(&self) -> Self {
        match self {
            Self::PrimerTiempo => Self::SegundoTiempo,
            Self::SegundoTiempo => Self::AlarguePrimerTiempo,
            Self::AlarguePrimerTiempo => Self::AlargueSegundoTiempo,
            Self::AlargueSegundoTiempo => Self::PrimerTiempo,
        }
    }
}
