use crate::config::FONDO_POR_DEFECTO;
use crate::{Mundial, Vista, VistaObj};
use eframe::egui;

#[derive(Default)]
pub struct MenuPrincipal;

impl Vista for MenuPrincipal {
    fn dimensiones(&self) -> egui::Vec2 {
        return egui::Vec2 { x: 640.0, y: 480.0 };
    }

    fn color_de_fondo(&self) -> egui::Color32 {
        return FONDO_POR_DEFECTO;
    }

    fn actualizar(&mut self, ui: &mut egui::Ui, _modelo: &mut Option<Mundial>) -> Option<VistaObj> {
        let mut proxima_vista = None;

        ui.heading("Información del Sistema");
        ui.add_space(10.0);

        ui.label("Esta es una pantalla secundaria muy sencilla.");
        ui.label("Versión de la aplicación: 1.0.0");
        ui.label("Desarrollado con Rust y egui.");

        ui.add_space(15.0);

        if ui.button("Entendido").clicked() {
            // proxima_vista = Some(VistaObj::new::<???>());
        }

        proxima_vista
    }
}
