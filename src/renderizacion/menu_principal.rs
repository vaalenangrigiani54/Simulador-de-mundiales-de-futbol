use crate::ModeloSimulador;
use crate::eventos::EventoMenuPrincipal;
use crate::mvc::Ventana;
use crate::templates::boton::*;
use crate::templates::popup::{ResultadoPopUp, popup_confirmacion};
use crate::templates::texto::crear_texto;
use crate::ventanas::MenuPrincipal;
use eframe::egui::{self, Color32, Vec2};

impl Ventana for MenuPrincipal {
    type Evento = EventoMenuPrincipal;

    fn renderizar(&mut self, ui: &mut egui::Ui, _global: &ModeloSimulador) -> Option<Self::Evento> {
        let mut proximo_evento = None;

        self.render_titulo(ui);
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(30.0);

        self.render_boton_seleccion_clasificados(ui, &mut proximo_evento);
        ui.add_space(10.0);
        self.render_boton_clasificacion_aleatoria(ui, &mut proximo_evento);
        ui.add_space(10.0);
        self.render_boton_salir(ui, &mut proximo_evento);

        proximo_evento
    }
}

impl MenuPrincipal {
    fn render_titulo(&self, ui: &mut egui::Ui) {
        let texto = crear_texto("SIMULADOR DE MUNDIALES", 50.0, Color32::BLACK, true);

        ui.vertical_centered(|ui| ui.heading(texto));
    }

    fn render_boton_seleccion_clasificados(
        &self,
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
                habilitado: true,
            },
            || {
                if proximo_evento.is_none() {
                    *proximo_evento = Some(EventoMenuPrincipal::IrASeleccionarEquiposClasificados);
                }
            },
        );
    }

    fn render_boton_clasificacion_aleatoria(
        &self,
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
                habilitado: true,
            },
            || {
                if proximo_evento.is_none() {
                    *proximo_evento = Some(EventoMenuPrincipal::ClasificacionAleatoria);
                }
            },
        );
    }

    fn render_boton_salir(
        &mut self,
        ui: &mut egui::Ui,
        proximo_evento: &mut Option<EventoMenuPrincipal>,
    ) {
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
                habilitado: true,
            },
            || {
                if proximo_evento.is_none() {
                    self.mostrar_popup_salir = true;
                }
            },
        );

        if self.mostrar_popup_salir
            && let Some(respuesta) = popup_confirmacion(
                ui.ctx(),
                "Simulador de mundiales de fútbol",
                "¿Está seguro que desea salir?",
            )
        {
            match respuesta {
                ResultadoPopUp::Afirmativo => *proximo_evento = Some(EventoMenuPrincipal::Salir),
                _ => {}
            }

            self.mostrar_popup_salir = false;
        }
    }
}
