use crate::{Controlador, EventoGlobal, EventoMenuPrincipal, MenuPrincipal, Mundial, Ventana};
use eframe::egui;

impl Controlador for MenuPrincipal {
    fn actualizar(&mut self, ui: &mut egui::Ui, _modelo: &mut Option<Mundial>) -> EventoGlobal {
        if let Some(evento) = Self::renderizar(ui) {
            match evento {
                EventoMenuPrincipal::EstablecerCantidadDeEquipos(cantidad) => {
                    self.establecer_cantidad_equipos(cantidad)
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

impl MenuPrincipal {
    fn establecer_cantidad_equipos(&mut self, cantidad: usize) {
        self.cantidad_equipos = cantidad;
    }
}
