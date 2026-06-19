use crate::config::{FONDO_POR_DEFECTO, MARGEN_EXTERIOR_POR_DEFECTO};
use crate::{EventoGlobal, MenuPrincipal, Mundial};
use eframe::egui::{Color32, Ui, Vec2};
use std::ops::{Deref, DerefMut};

/// Información de ventana que es segura para usar a través de trait-objects.
pub trait VentanaInfo {
    /// Ancho y Alto de la ventana (en píxeles).
    fn dimensiones(&self) -> Vec2;

    /// Color de fondo de la ventana.
    fn color_de_fondo(&self) -> Color32 {
        FONDO_POR_DEFECTO
    }

    /// Margen exterior de la ventana sin contar las dimensiones (Padding inverso).
    fn margen_exterior(&self) -> f32 {
        MARGEN_EXTERIOR_POR_DEFECTO
    }
}

/// Todas las vistas deben implementar este trait para poder ser mostradas.
pub trait Ventana: VentanaInfo {
    type Evento;

    /// Dibuja los componentes y devuelve una posible acción a ejecutarse (evento).
    fn renderizar(ui: &mut Ui) -> Option<Self::Evento>;
}

/// Este trait también lo deben implementar todos los tipos de vista.
///
/// Requiere `VentanaInfo` para que los controladores expongan las configuraciones
/// generales a través de `dyn Controlador`.
pub trait Controlador: VentanaInfo {
    fn actualizar(&mut self, ui: &mut Ui, modelo: &mut Option<Mundial>) -> EventoGlobal;
}

// Trait `Controlador` tratado como un objeto.
pub struct ControladorObj(Box<dyn Controlador>);

impl ControladorObj {
    pub fn new<V: Controlador + Ventana + Default + 'static>() -> Self {
        Self(Box::new(V::default()))
    }
}

impl Default for ControladorObj {
    fn default() -> Self {
        Self::new::<MenuPrincipal>()
    }
}

impl Deref for ControladorObj {
    type Target = dyn Controlador;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl DerefMut for ControladorObj {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}
