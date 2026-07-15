use crate::ModeloSimulador;
use crate::eventos::{EventoGlobal, EventoInicioMundial};
use crate::mvc::{Controlador, Ventana};
use crate::ventanas::{InicioMundial, MenuPrincipal};
use eframe::egui;
use std::any::TypeId;

impl Controlador for InicioMundial {
    fn actualizar(&mut self, ui: &mut egui::Ui, modelo: &mut ModeloSimulador) -> EventoGlobal {
        if let Some(evento) = self.renderizar(ui, modelo) {
            match evento {
                EventoInicioMundial::VerConformacionDeGrupos => todo!(),
                EventoInicioMundial::IrAEditarGrupos => todo!(),
                EventoInicioMundial::EmpezarSimulacion => todo!(),
                EventoInicioMundial::VolverAlMenuPrincipal => {
                    return EventoGlobal::CambiarDeVista(TypeId::of::<MenuPrincipal>());
                }
            }
        }

        EventoGlobal::Continuar
    }
}
