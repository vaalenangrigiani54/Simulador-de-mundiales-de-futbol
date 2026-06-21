use crate::ventanas::MenuPrincipal;
use crate::{
    Controlador, EventoGlobal, EventoMenuPrincipal, ModeloSimulador, Ventana,
};
use eframe::egui;

impl Controlador for MenuPrincipal {
    fn actualizar(&mut self, ui: &mut egui::Ui, modelo: &mut ModeloSimulador) -> EventoGlobal {
        if let Some(evento) = self.renderizar(ui) {
            match evento {
                EventoMenuPrincipal::EstablecerCantidadDeEquipos(cantidad) => {
                    modelo.cant_participantes = cantidad
                }
                EventoMenuPrincipal::IrASeleccionarEquiposClasificados => {
                    return EventoGlobal::Continuar;
                }
                EventoMenuPrincipal::IrAClasificacionAleatoria => return EventoGlobal::Continuar,
                EventoMenuPrincipal::Salir => return EventoGlobal::Salir,
            }
        }

        EventoGlobal::Continuar
    }
}
