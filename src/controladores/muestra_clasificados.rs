use crate::eventos::{EventoGlobal, EventoMuestraClasificados};
use crate::ventanas::MuestraClasificados;
use crate::{Controlador, ModeloSimulador, Ventana};
use eframe::egui;

impl Controlador for MuestraClasificados {
    fn actualizar(&mut self, ui: &mut egui::Ui, modelo: &mut ModeloSimulador) -> EventoGlobal {
        if let Some(evento) = self.renderizar(ui, modelo) {
            match evento {
                EventoMuestraClasificados::IrAInicioDelMundial => todo!(),
                EventoMuestraClasificados::IrAFormacionDeGrupos => todo!(),
                EventoMuestraClasificados::IrASeleccionarEquiposClasificados => todo!(),
            }
        }

        EventoGlobal::Continuar
    }
}
