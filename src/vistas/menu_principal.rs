use crate::{EventoMenuPrincipal, MenuPrincipal, Ventana};
use eframe::egui;

impl Ventana for MenuPrincipal {
    type Evento = EventoMenuPrincipal;

    fn renderizar(ui: &mut egui::Ui) -> Option<Self::Evento> {
        let mut proximo_evento = None;

        render_titulo(ui);
        ui.add_space(10.0);

        render_descripcion(ui);
        ui.add_space(15.0);

        render_boton_salir(ui, &mut proximo_evento);

        proximo_evento
    }
}

fn render_titulo(ui: &mut egui::Ui) {
    ui.heading("Información del Sistema");
}

fn render_descripcion(ui: &mut egui::Ui) {
    ui.label("Esta es una pantalla secundaria muy sencilla.");
    ui.label("Versión de la aplicación: 1.0.0");
    ui.label("Desarrollado con Rust y egui.");
}

fn render_boton_salir(ui: &mut egui::Ui, proximo_evento: &mut Option<EventoMenuPrincipal>) {
    let boton = ui.button("Salir");

    if boton.clicked() && proximo_evento.is_none() {
        *proximo_evento = Some(EventoMenuPrincipal::Salir)
    }
}
