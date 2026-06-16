#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum TiempoDelPartido {
    PrimerTiempo,
    SegundoTiempo,
    AlarguePrimerTiempo,
    AlargueSegundoTiempo,
}

impl TiempoDelPartido {
    /// Devuelve la duración (**en minutos**) del tiempo.
    pub fn duracion(&self) -> u8 {
        match self {
            TiempoDelPartido::PrimerTiempo | TiempoDelPartido::SegundoTiempo => 45,
            TiempoDelPartido::AlarguePrimerTiempo | TiempoDelPartido::AlargueSegundoTiempo => 15,
        }
    }

    /// Devuelve en qué minuto arranca cada tiempo.
    pub fn offset(&self) -> u8 {
        match self {
            TiempoDelPartido::PrimerTiempo => 0,
            TiempoDelPartido::SegundoTiempo => 45,
            TiempoDelPartido::AlarguePrimerTiempo => 90,
            TiempoDelPartido::AlargueSegundoTiempo => 105,
        }
    }
}
