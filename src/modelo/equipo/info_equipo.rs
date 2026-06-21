use std::fmt::Display;

pub trait EquipoInfo {
    /// Devuelve el **nombre** del equipo (inmutable).
    fn get_nombre(&self) -> &String;

    /// Devuelve el **código FIFA** del equipo (inmutable).
    fn get_codigo(&self) -> &String;
}

/// Información general de una selección de fútbol.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Equipo {
    nombre: String,
    codigo: String,
}

impl Equipo {
    pub fn new(nombre: impl Into<String>, codigo: impl Into<String>) -> Self {
        Equipo {
            nombre: nombre.into(),
            codigo: codigo.into(),
        }
    }
}

impl EquipoInfo for Equipo {
    fn get_nombre(&self) -> &String {
        &self.nombre
    }

    fn get_codigo(&self) -> &String {
        &self.codigo
    }
}

impl Display for Equipo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.nombre, self.codigo)
    }
}
