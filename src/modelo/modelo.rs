use crate::{Equipo, Mundial};

/// Estructura principal del modelo global.
pub struct ModeloSimulador {
    /// Almacena todos los equipos afiliados a la FIFA.
    afiliados_fifa: Vec<Equipo>,

    /// Cantidad de participantes en el torneo.
    pub cant_participantes: usize,

    /// Modelo del mundial. Si no está creado es None.
    pub mundial: Option<Mundial>,
}

impl Default for ModeloSimulador {
    fn default() -> Self {
        Self {
            afiliados_fifa: Vec::with_capacity(120),
            cant_participantes: 16,
            mundial: None,
        }
    }
}
