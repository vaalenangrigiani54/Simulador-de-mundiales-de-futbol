use crate::templates::boton::*;
use crate::templates::texto::crear_texto;
use crate::{EventoMenuPrincipal, MenuPrincipal, Ventana};
use eframe::egui::{self, Color32, Vec2};

impl Ventana for MenuPrincipal {
    type Evento = EventoMenuPrincipal;

    fn renderizar(&self, ui: &mut egui::Ui) -> Option<Self::Evento> {
        let mut proximo_evento = None;

        render_titulo(ui);
        ui.add_space(30.0);

        render_boton_seleccion_clasificados(ui, &mut proximo_evento);
        ui.add_space(10.0);
        render_boton_clasificacion_aleatoria(ui, &mut proximo_evento);
        ui.add_space(10.0);
        render_boton_salir(ui, &mut proximo_evento);

        proximo_evento
    }
}

fn render_titulo(ui: &mut egui::Ui) {
    let texto = crear_texto("SIMULADOR DE MUNDIALES", 50.0, Color32::BLACK, true);

    ui.vertical_centered(|ui| ui.heading(texto));
}

fn render_boton_seleccion_clasificados(
    ui: &mut egui::Ui,
    proximo_evento: &mut Option<EventoMenuPrincipal>,
) {
    let texto = crear_texto(
        "Seleccionar equipos clasificados",
        28.0,
        Color32::BLACK,
        false,
    );

    agregar_boton(
        ui,
        PropiedadesBoton {
            texto,
            dimensiones: Vec2 {
                x: ui.available_width(),
                y: 35.0,
            },
            fondo_rgb: (255, 255, 200),
        },
        || {
            if proximo_evento.is_none() {
                *proximo_evento = Some(EventoMenuPrincipal::IrASeleccionarEquiposClasificados);
            }
        },
    );
}

fn render_boton_clasificacion_aleatoria(
    ui: &mut egui::Ui,
    proximo_evento: &mut Option<EventoMenuPrincipal>,
) {
    let texto = crear_texto(
        "Clasificar equipos aleatoriamente",
        28.0,
        Color32::BLACK,
        false,
    );

    agregar_boton(
        ui,
        PropiedadesBoton {
            texto,
            dimensiones: Vec2 {
                x: ui.available_width(),
                y: 35.0,
            },
            fondo_rgb: (255, 200, 255),
        },
        || {
            if proximo_evento.is_none() {
                *proximo_evento = Some(EventoMenuPrincipal::IrAClasificacionAleatoria);
            }
        },
    );
}

fn render_boton_salir(ui: &mut egui::Ui, proximo_evento: &mut Option<EventoMenuPrincipal>) {
    let texto = crear_texto("Salir", 28.0, Color32::BLACK, false);

    agregar_boton(
        ui,
        PropiedadesBoton {
            texto,
            dimensiones: Vec2 {
                x: ui.available_width(),
                y: 35.0,
            },
            fondo_rgb: (255, 200, 200),
        },
        || {
            if proximo_evento.is_none() {
                *proximo_evento = Some(EventoMenuPrincipal::Salir);
            }
        },
    );
}
