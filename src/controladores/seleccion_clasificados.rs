use std::any::TypeId;
use std::collections::BTreeSet;
use eframe::egui;
use crate::eventos::{EventoGlobal, EventoSeleccionClasificados};
use crate::ventanas::{MenuPrincipal, MuestraClasificados, SeleccionClasificados};
use crate::{Controlador, EquipoParticipanteRef, ModeloSimulador, Ventana};

impl Controlador for SeleccionClasificados {
    fn actualizar(&mut self, ui: &mut egui::Ui, modelo: &mut ModeloSimulador) -> EventoGlobal {
        if modelo.clasificados.is_none() {
            modelo.clasificados = Some(BTreeSet::new());
        }
        
        if let Some(evento) = self.renderizar(ui, modelo) {
            match evento {
                EventoSeleccionClasificados::AgregarOQuitarEquipoClasificado(idx_equipo, agregar) => {
                    let equipo = (&modelo.get_equipos_disponibles()[idx_equipo]).clone();
                    
                    if let Some(clasificados) = modelo.clasificados.as_mut() {
                        let equipo = EquipoParticipanteRef::from(equipo);

                        if agregar {
                            clasificados.insert(equipo);
                        } else {
                            clasificados.remove(&equipo);
                        }
                    }
                },
                EventoSeleccionClasificados::IrAMenuPrincipal => return EventoGlobal::CambiarDeVista(TypeId::of::<MenuPrincipal>()),
                EventoSeleccionClasificados::IrAMuestraClasificados => return EventoGlobal::CambiarDeVista(TypeId::of::<MuestraClasificados>()),
            }
        }

        EventoGlobal::Continuar
    }
}