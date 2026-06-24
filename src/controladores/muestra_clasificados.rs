use std::any::TypeId;

use crate::eventos::{EventoGlobal, EventoMuestraClasificados};
use crate::ventanas::{InicioMundial, MuestraClasificados, SeleccionClasificados};
use crate::{Controlador, ModeloSimulador, Ventana};
use eframe::egui;

impl Controlador for MuestraClasificados {
    fn actualizar(&mut self, ui: &mut egui::Ui, modelo: &mut ModeloSimulador) -> EventoGlobal {
        modelo.grupos.take();
        modelo.mundial.take();
        
        if let Some(evento) = self.renderizar(ui, modelo) {
            match evento {
                EventoMuestraClasificados::IrAInicioDelMundial => {
                    return EventoGlobal::CambiarDeVista(TypeId::of::<InicioMundial>());
                },
                EventoMuestraClasificados::IrAEditarGrupos => todo!(),
                EventoMuestraClasificados::IrASeleccionarEquiposClasificados => return EventoGlobal::CambiarDeVista(TypeId::of::<SeleccionClasificados>()),
            }
        }

        EventoGlobal::Continuar
    }
}
