use crate::eventos::EventoInfoGrupos;
use crate::mvc::Ventana;
use crate::templates::texto::crear_texto;
use crate::ventanas::{InfoGrupos, ModoInfoGrupos};
use crate::ModeloSimulador;
use eframe::egui::{self, Color32};

impl Ventana for InfoGrupos {
    type Evento = EventoInfoGrupos;

    fn renderizar(&mut self, ui: &mut egui::Ui, _global: &ModeloSimulador) -> Option<Self::Evento> {
        let letra_grupo = char::from(b'A' + self.idx_grupo);
        let titulo = match self.modo {
            ModoInfoGrupos::SoloMostrar => format!("Grupo {letra_grupo}"),
            ModoInfoGrupos::ResultadosParciales(_) => todo!(),
            ModoInfoGrupos::ResultadosFinales => format!("Resultados del Grupo {letra_grupo}"),
        };

        Self::render_titulo(ui, &titulo);
        None
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