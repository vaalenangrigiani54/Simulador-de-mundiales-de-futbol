use eframe::egui::{self, Color32};
use crate::{ModeloSimulador, Ventana};
use crate::eventos::EventoMuestraClasificados;
use crate::templates::boton::{agregar_boton_atras, agregar_boton_siguiente};
use crate::templates::texto::crear_texto;
use crate::ventanas::MuestraClasificados;

impl Ventana for MuestraClasificados {
    type Evento = EventoMuestraClasificados;

    fn renderizar(&self, ui: &mut egui::Ui, global: &ModeloSimulador) -> Option<Self::Evento> {
        let mut proximo_evento = None;

        render_titulo(ui, global.cant_participantes);
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(30.0);
        
        ui.vertical_centered_justified(|ui| {
            render_boton_atras(ui, &mut proximo_evento);
            render_boton_siguiente(ui, &mut proximo_evento);
        });

        proximo_evento
    }
}

fn render_titulo(ui: &mut egui::Ui, cantidad: usize) {
    let texto = crear_texto(format!("LOS {cantidad} CLASIFICADOS"), 50.0, Color32::BLACK, true);

    ui.vertical_centered(|ui| ui.heading(texto));
}

fn render_boton_atras(ui: &mut egui::Ui, proximo_evento: &mut Option<EventoMuestraClasificados>) {
    agregar_boton_atras(ui, || {
        if proximo_evento.is_none() {
            *proximo_evento = Some(EventoMuestraClasificados::IrASeleccionarEquiposClasificados)
        }
    });
}

fn render_boton_siguiente(ui: &mut egui::Ui, proximo_evento: &mut Option<EventoMuestraClasificados>) {
    agregar_boton_siguiente(ui, || {
        if proximo_evento.is_none() {
            *proximo_evento = Some(EventoMuestraClasificados::IrAInicioDelMundial)
        }
    });
}