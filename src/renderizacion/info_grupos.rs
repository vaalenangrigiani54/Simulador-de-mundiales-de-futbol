use crate::eventos::EventoInfoGrupos;
use crate::mvc::Ventana;
use crate::templates::texto::crear_texto;
use crate::ventanas::{InfoGrupos, ModoInfoGrupos};
use crate::ModeloSimulador;
use eframe::egui::{self, Color32};

impl Ventana for InfoGrupos {
    type Evento = EventoInfoGrupos;

    fn renderizar(&mut self, ui: &mut egui::Ui, global: &ModeloSimulador) -> Option<Self::Evento> {
        let mut proximo_evento = None;
        let (
            titulo,
            texto_boton_atras
        ) = match self.modo {
            ModoInfoGrupos::SoloMostrar => ("Grupo A", "Anterior"),
            ModoInfoGrupos::ResultadosParciales(_) => todo!(),
            ModoInfoGrupos::ResultadosFinales => ("Resultados del Grupo A", "Anterior"),
        };
        
        Self::render_titulo(ui, titulo);
        
        proximo_evento
    }
}


impl InfoGrupos {
    fn render_titulo(ui: &mut egui::Ui, titulo: &str) {
        let texto = crear_texto(
            titulo,
            50.0,
            Color32::BLACK,
            true,
        );

        ui.vertical_centered(|ui| ui.heading(texto));
        ui.add_space(10.0);
        ui.separator();
    }
}