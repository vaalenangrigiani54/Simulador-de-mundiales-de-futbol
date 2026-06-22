use std::any::TypeId;
use eframe::egui;
use crate::eventos::{EventoGlobal, EventoSeleccionClasificados};
use crate::ventanas::{MenuPrincipal, MuestraClasificados, SeleccionClasificados};
use crate::{Controlador, Ventana};

impl Controlador for SeleccionClasificados {
    fn actualizar(&mut self, ui: &mut egui::Ui, modelo: &mut crate::ModeloSimulador) -> EventoGlobal {
        if let Some(evento) = self.renderizar(ui, modelo) {
            match evento {
                EventoSeleccionClasificados::AgregarEquipoClasificado(_) => {

                },
                EventoSeleccionClasificados::QuitarEquipoClasificado(_) => {
                    
                },
                EventoSeleccionClasificados::IrAMenuPrincipal => {
                    modelo.clasificados = None;
                    return EventoGlobal::CambiarDeVista(TypeId::of::<MenuPrincipal>());
                },
                EventoSeleccionClasificados::IrAMuestraClasificados => {
                    return EventoGlobal::CambiarDeVista(TypeId::of::<MuestraClasificados>());
                },
            }
        }

        EventoGlobal::Continuar
    }
}