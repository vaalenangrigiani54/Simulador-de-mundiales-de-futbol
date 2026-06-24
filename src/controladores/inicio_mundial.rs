use crate::eventos::{EventoGlobal, EventoInicioMundial};
use crate::ventanas::InicioMundial;
use crate::{Controlador, ModeloSimulador, Ventana};

impl Controlador for InicioMundial {
    fn actualizar(&mut self, ui: &mut eframe::egui::Ui, modelo: &mut ModeloSimulador) -> EventoGlobal {
        if let Some(evento) = self.renderizar(ui, modelo) {
            match evento {
                EventoInicioMundial::VerConformacionDeGrupos => todo!(),
                EventoInicioMundial::IrAEditarGrupos => todo!(),
                EventoInicioMundial::EmpezarSimulacion => todo!(),
                EventoInicioMundial::VolverAlMenuPrincipal => todo!(),
            }
        }

        EventoGlobal::Continuar
    }
}