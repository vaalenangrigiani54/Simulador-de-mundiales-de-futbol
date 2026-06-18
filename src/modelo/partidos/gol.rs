use crate::{MinutoDePartido, TiempoDelPartido};
use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Gol {
    tiempo: TiempoDelPartido,
    minuto: MinutoDePartido,
    de_penal: bool,
}

impl Gol {
    /// Un gol como cualquiera.
    ///
    /// ## Importante:
    /// El minuto que se realizó en el gol es relativo al tiempo del partido.
    pub fn normal(tiempo: TiempoDelPartido, minuto: MinutoDePartido) -> Self {
        Self {
            tiempo,
            minuto,
            de_penal: false,
        }
    }

    /// ## Importante:
    /// El minuto que se realizó en el gol de penal es relativo al tiempo del partido.
    pub fn de_penal(tiempo: TiempoDelPartido, minuto: MinutoDePartido) -> Self {
        Self {
            tiempo,
            minuto,
            de_penal: true,
        }
    }
}

impl Display for Gol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::with_capacity(12);
        let offset = self.tiempo.offset();
        let limite = offset + self.tiempo.duracion();
        let minuto_real = offset + self.minuto;

        if minuto_real >= limite {
            let resto = minuto_real - limite;
            str += format!("{limite}+{resto}'").as_str()
        } else {
            str += format!("{minuto_real}'").as_str()
        }

        if self.de_penal {
            str += " (P)"
        }

        write!(f, "{str}")
    }
}
