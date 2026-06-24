use eframe::egui::{self, Color32};
use crate::eventos::EventoInicioMundial;
use crate::templates::texto::crear_texto;
use crate::ventanas::InicioMundial;
use crate::{ModeloSimulador, Ventana};

impl Ventana for InicioMundial {
    type Evento = EventoInicioMundial;

    fn renderizar(&self, ui: &mut eframe::egui::Ui, global: &ModeloSimulador) -> Option<Self::Evento> {
        let mut proximo_evento = None;

        render_titulo(ui);
        ui.add_space(25.0);

        proximo_evento
    }
}

fn render_titulo(ui: &mut egui::Ui) {
    let texto = crear_texto(
        format!("COPA MUNDIAL DE FÚTBOL 2026"),
        50.0,
        Color32::BLACK,
        true,
    );

    ui.vertical_centered(|ui| ui.heading(texto));
    ui.add_space(10.0);
    ui.separator();
}