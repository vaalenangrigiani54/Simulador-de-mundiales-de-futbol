use crate::eventos::EventoSeleccionClasificados;
use crate::templates::boton::{agregar_boton_atras, agregar_boton_siguiente};
use crate::templates::texto::crear_texto;
use crate::ventanas::SeleccionClasificados;
use crate::{ModeloSimulador, Ventana};
use eframe::egui::{self, Color32};

const MARGEN_VERTICAL_TABLA: f32 = 25.0;

impl Ventana for SeleccionClasificados {
    type Evento = EventoSeleccionClasificados;

    fn renderizar(&self, ui: &mut egui::Ui, global: &ModeloSimulador) -> Option<Self::Evento> {
        let mut proximo_evento = None;

        render_titulo(ui);
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(MARGEN_VERTICAL_TABLA);

        ui.columns(3, |columns| {
            render_boton_atras(&mut columns[0], &mut proximo_evento);
            render_label_seleccionados(&mut columns[1], 0, global.cant_participantes);
            render_boton_siguiente(&mut columns[2], &mut proximo_evento);
        });

        proximo_evento
    }
}

fn render_titulo(ui: &mut egui::Ui) {
    let texto = crear_texto(
        "SELECCIONAR A LOS CLASIFICADOS",
        50.0,
        Color32::BLACK,
        true,
    );

    ui.vertical_centered(|ui| ui.heading(texto));
}

fn render_boton_atras(ui: &mut egui::Ui, proximo_evento: &mut Option<EventoSeleccionClasificados>) {
    agregar_boton_atras(ui, || {
        if proximo_evento.is_none() {
            *proximo_evento = Some(EventoSeleccionClasificados::IrAMenuPrincipal)
        }
    });
}

fn render_label_seleccionados(ui: &mut egui::Ui, cantidad: usize, total: usize) {
    ui.vertical_centered(|ui| {
        let texto = crear_texto(
            format!("Cantidad seleccionados: {cantidad} / {total}"),
            20.0,
            Color32::BLACK,
            false
        );

        ui.label(texto);
    });
}

fn render_boton_siguiente(
    ui: &mut egui::Ui,
    proximo_evento: &mut Option<EventoSeleccionClasificados>,
) {
    agregar_boton_siguiente(ui, || {
        if proximo_evento.is_none() {
            *proximo_evento = Some(EventoSeleccionClasificados::IrAMuestraClasificados)
        }
    });
}
