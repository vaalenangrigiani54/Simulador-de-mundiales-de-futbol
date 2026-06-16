use crate::{Mundial, VistaObj};


#[derive(Default)]
pub struct AppSimulador {
    vista_actual: VistaObj,
    modelo: Option<Mundial>
}


impl eframe::App for AppSimulador {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        if let Some(proxima_vista) = self.vista_actual.actualizar(ui, &mut self.modelo) {
            self.vista_actual = proxima_vista;
        }
    }
}