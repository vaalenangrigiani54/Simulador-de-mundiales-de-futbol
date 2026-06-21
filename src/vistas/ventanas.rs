use crate::VentanaInfo;
use eframe::egui::Vec2;

#[derive(Default)]
pub struct MenuPrincipal;

impl VentanaInfo for MenuPrincipal {
    fn dimensiones(&self) -> Vec2 {
        Vec2::new(800.0, 480.0)
    }
}
