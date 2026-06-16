use std::ops::{Deref, DerefMut};

use eframe::egui;
use crate::{Mundial, VistaMenuPrincipal};

/// Todas las vistas deben implementar este trait para poder ser mostradas.
pub trait Vista {
    /// El valor de retorno es una decisión de si se va a cambiar de vista o no.
    fn actualizar(&mut self, ui: &mut egui::Ui, modelo: &mut Option<Mundial>) -> Option<VistaObj>;
}


/// Implementación del trait `Vista` como un objeto.
pub struct VistaObj(Box<dyn Vista>);


impl VistaObj {
    pub fn new<V: Vista + Default + 'static>() -> Self {
        Self(Box::new(V::default()))
    }
}


impl Default for VistaObj {
    fn default() -> Self {
        Self::new::<VistaMenuPrincipal>()
    }
}


impl Deref for VistaObj {
    type Target = dyn Vista;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}


impl DerefMut for VistaObj {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}