use eframe::egui;
use crate::{Mundial, VistaObj};

/// Color de fondo por defecto.
pub const FONDO_POR_DEFECTO: egui::Color32 = egui::Color32::from_rgb(220, 220, 220);

/// Márgen exterior para todas las vistas.
const MARGEN: f32 = 25.0;


#[derive(Default)]
pub struct AppSimulador {
    vista_actual: VistaObj,
    modelo: Option<Mundial>
}


impl eframe::App for AppSimulador {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx();
        
        // Redimensionar la ventana.
        let tamanio_vista = self.vista_actual.dimensiones();
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(tamanio_vista));

        // Obtener la resolución del monitor para centrar la ventana.
        if let Some(monitor_size) = ctx.input(|i| i.viewport().monitor_size) {
            // Aplicamos la fórmula matemática de centrado: (Monitor / 2) - (Ventana / 2)
            let centro_x = (monitor_size.x / 2.0) - (tamanio_vista.x / 2.0);
            let centro_y = (monitor_size.y / 2.0) - (tamanio_vista.y / 2.0);
            
            let posicion_centrada = egui::pos2(centro_x, centro_y);
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(posicion_centrada));
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE
                .fill(self.vista_actual.color_de_fondo())
                .inner_margin(MARGEN)
            )
            .show_inside(ui, |ui| {

            if let Some(proxima_vista) = self.vista_actual.actualizar(ui, &mut self.modelo) {
                self.vista_actual = proxima_vista;
            } 
        });
    }
}