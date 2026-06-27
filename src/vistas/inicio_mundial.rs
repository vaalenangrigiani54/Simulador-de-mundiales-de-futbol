use eframe::egui::{self, Color32};
use crate::eventos::EventoInicioMundial;
use crate::templates::boton::{agregar_boton, propiedades_boton_atras, propiedades_boton_menu, propiedades_boton_siguiente};
use crate::templates::popup::{ResultadoPopUp, popup_confirmacion};
use crate::templates::texto::crear_texto;
use crate::ventanas::InicioMundial;
use crate::{ModeloSimulador, Ventana};

impl Ventana for InicioMundial {
    type Evento = EventoInicioMundial;

    fn renderizar(&mut self, ui: &mut eframe::egui::Ui, global: &ModeloSimulador) -> Option<Self::Evento> {
        let mut proximo_evento = None;

        self.render_titulo(ui);
        ui.add_space(25.0);

        ui.columns(3, |columns| {
            self.render_boton_atras(&mut columns[0], &mut proximo_evento);
            self.render_boton_menu(&mut columns[1], &mut proximo_evento);
            self.render_boton_siguiente(&mut columns[2], &mut proximo_evento);
        });

        proximo_evento
    }
}

impl InicioMundial {
    fn render_titulo(&self, ui: &mut egui::Ui) {
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

    fn render_boton_atras(&self, ui: &mut egui::Ui, proximo_evento: &mut Option<EventoInicioMundial>) {
        let cfg = propiedades_boton_atras(true, None);
        
        agregar_boton(ui, cfg, || {
            if proximo_evento.is_none() {
                *proximo_evento = Some(EventoInicioMundial::IrAEditarGrupos)
            }
        });
    }

    fn render_boton_menu(&mut self, ui: &mut egui::Ui, proximo_evento: &mut Option<EventoInicioMundial>) {
        let cfg = propiedades_boton_menu(true, None);
        
        agregar_boton(ui, cfg, || {
            if proximo_evento.is_none() {
                self.mostrar_popup_menu = true;
            }
        });

        if self.mostrar_popup_menu
            && let Some(resultado) = popup_confirmacion(
                ui.ctx(), 
                "Volver al menú principal",
                "¿Seguro que desea volver al menú principal?\nSe perderán todos los datos de la simulación actual."
            )
        {
            match resultado {
                ResultadoPopUp::Afirmativo => *proximo_evento = Some(EventoInicioMundial::VolverAlMenuPrincipal),
                _ => {},
            }

            self.mostrar_popup_menu = false;
        }
    }

    fn render_boton_siguiente(
        &self,
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
}