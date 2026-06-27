use std::any::TypeId;
use eframe::egui;
use crate::eventos::{EventoGlobal, EventoInicioMundial};
use crate::ventanas::{InicioMundial, MenuPrincipal};
use crate::{Controlador, ModeloSimulador, Ventana};

impl Controlador for InicioMundial {
    fn actualizar(&mut self, ui: &mut egui::Ui, modelo: &mut ModeloSimulador) -> EventoGlobal {
        if let Some(evento) = self.renderizar(ui, modelo) {
            match evento {
                EventoInicioMundial::VerConformacionDeGrupos => todo!(),
                EventoInicioMundial::IrAEditarGrupos => todo!(),
                EventoInicioMundial::EmpezarSimulacion => todo!(),
                EventoInicioMundial::VolverAlMenuPrincipal => {
                    return EventoGlobal::CambiarDeVista(TypeId::of::<MenuPrincipal>());
                },
            }
        }

        EventoGlobal::Continuar
    }
}