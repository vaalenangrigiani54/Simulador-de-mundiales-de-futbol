use crate::config::{FONDO_POR_DEFECTO, MARGEN_EXTERIOR_POR_DEFECTO};
use eframe::egui::{Color32, Vec2};


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


#[derive(Default)]
pub struct MenuPrincipal;


impl VentanaInfo for MenuPrincipal {
    fn dimensiones(&self) -> Vec2 {
        Vec2::new(800.0, 480.0)
    }
}