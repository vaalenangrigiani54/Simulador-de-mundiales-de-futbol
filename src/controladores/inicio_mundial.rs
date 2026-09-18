use crate::ModeloSimulador;
use crate::eventos::{EventoGlobal, EventoInicioMundial};
use crate::mvc::{Controlador, Ventana};
use crate::ventanas::{InicioMundial, ModoInfoGrupos};
use eframe::egui;

impl Controlador for InicioMundial {
    fn actualizar(&mut self, ui: &mut egui::Ui, modelo: &mut ModeloSimulador) -> EventoGlobal {
        if let Some(evento) = self.renderizar(ui, modelo) {
            match evento {
                EventoInicioMundial::VerConformacionDeGrupos => return EventoGlobal::CambiarDeVistaAInfoGrupos {
                    modo: ModoInfoGrupos::SoloMostrar,
                    idx_grupo: 0
                },
                EventoInicioMundial::IrAEditarGrupos => todo!(),
                EventoInicioMundial::EmpezarSimulacion => todo!(),
                EventoInicioMundial::VolverAlMenuPrincipal => todo!()
            }
        }

        EventoGlobal::Continuar
    }
}
