use crate::ModeloSimulador;
use crate::eventos::{EventoGlobal, EventoInfoGrupos};
use crate::mvc::{Controlador, Ventana};
use crate::ventanas::InfoGrupos;
use eframe::egui;

impl Controlador for InfoGrupos {
    fn actualizar(&mut self, ui: &mut egui::Ui, modelo: &mut ModeloSimulador) -> EventoGlobal {
        if let Some(evento) = self.renderizar(ui, modelo) {
            match evento {
                EventoInfoGrupos::VolverAInicioDelMundial => return EventoGlobal::CambiarDeVistaAInicioMundial,
                EventoInfoGrupos::EquipoAnterior => todo!(),
                EventoInfoGrupos::EquipoSiguiente => todo!(),
                EventoInfoGrupos::VolverAlPartido => todo!(),
            }
        }
        
        EventoGlobal::Continuar
    }
}