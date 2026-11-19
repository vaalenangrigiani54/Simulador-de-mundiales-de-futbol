use crate::eventos::{EventoGlobal, EventoSeleccionClasificados};
use crate::mvc::{Controlador, Ventana};
use crate::ventanas::SeleccionClasificados;
use crate::{EquipoParticipanteRef, ModeloSimulador};
use eframe::egui;
use crate::modelo::global::EquiposClasificados;

impl Controlador for SeleccionClasificados {
    fn actualizar(&mut self, ui: &mut egui::Ui, modelo: &mut ModeloSimulador) -> EventoGlobal {
        modelo
            .clasificados
            .get_or_insert_with(EquiposClasificados::new);

        if let Some(evento) = self.renderizar(ui, modelo) {
            match evento {
                EventoSeleccionClasificados::AgregarOQuitarEquipoClasificado(
                    idx_equipo,
                    agregar,
                ) => {
                    let equipo = modelo.get_equipos_disponibles()[idx_equipo].clone();

                    if let Some(clasificados) = modelo.clasificados.as_mut() {
                        if agregar {
                            clasificados.entry(equipo.clone()).or_insert_with(|| {
                                EquipoParticipanteRef::from(equipo)
                            });
                        } else {
                            clasificados.remove(&equipo);
                        }
                    }
                }
                EventoSeleccionClasificados::IrAMenuPrincipal => return EventoGlobal::CambiarDeVistaAMenuPrincipal,
                EventoSeleccionClasificados::IrAMuestraClasificados => return EventoGlobal::CambiarDeVistaAMuestraClasificados,
            }
        }

        EventoGlobal::Continuar
    }
}
