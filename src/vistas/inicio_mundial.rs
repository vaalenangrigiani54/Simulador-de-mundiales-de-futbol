use eframe::egui::{self, Color32};
use crate::eventos::EventoInicioMundial;
use crate::templates::boton::{agregar_boton, propiedades_boton_atras, propiedades_boton_menu, propiedades_boton_siguiente};
use crate::templates::texto::crear_texto;
use crate::ventanas::InicioMundial;
use crate::{ModeloSimulador, Ventana};

impl Ventana for InicioMundial {
    type Evento = EventoInicioMundial;

    fn renderizar(&self, ui: &mut eframe::egui::Ui, global: &ModeloSimulador) -> Option<Self::Evento> {
        let mut proximo_evento = None;

        render_titulo(ui);
        ui.add_space(25.0);

        ui.columns(3, |columns| {
            render_boton_atras(&mut columns[0], &mut proximo_evento);
            render_boton_menu(&mut columns[1], &mut proximo_evento);
            render_boton_siguiente(&mut columns[2], &mut proximo_evento);
        });

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

fn render_boton_atras(ui: &mut egui::Ui, proximo_evento: &mut Option<EventoInicioMundial>) {
    let cfg = propiedades_boton_atras(true, None);
    
    agregar_boton(ui, cfg, || {
        if proximo_evento.is_none() {
            *proximo_evento = Some(EventoInicioMundial::IrAEditarGrupos)
        }
    });
}

fn render_boton_menu(ui: &mut egui::Ui, proximo_evento: &mut Option<EventoInicioMundial>) {
    let cfg = propiedades_boton_menu(true, None);
    
    agregar_boton(ui, cfg, || {
        if proximo_evento.is_none() {
            *proximo_evento = Some(EventoInicioMundial::VolverAlMenuPrincipal)
        }
    });
}

fn render_boton_siguiente(
    ui: &mut egui::Ui,
    proximo_evento: &mut Option<EventoInicioMundial>,
) {
    let cfg = propiedades_boton_siguiente(true, None);
    
    agregar_boton(ui, cfg, || {
        if proximo_evento.is_none() {
            *proximo_evento = Some(EventoInicioMundial::EmpezarSimulacion)
        }
    });
}