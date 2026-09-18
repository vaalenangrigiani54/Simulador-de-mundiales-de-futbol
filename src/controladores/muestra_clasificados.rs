use crate::eventos::{EventoGlobal, EventoMuestraClasificados};
use crate::mvc::{Controlador, Ventana};
use crate::ventanas::MuestraClasificados;
use crate::{EquipoParticipanteRef, Grupo, ModeloSimulador};
use eframe::egui;
use rand::seq::SliceRandom;

const EQUIPOS_POR_GRUPO: usize = 4;

impl Controlador for MuestraClasificados {
    fn actualizar(&mut self, ui: &mut egui::Ui, modelo: &mut ModeloSimulador) -> EventoGlobal {
        modelo.grupos.take();
        modelo.mundial.take();

        if let Some(evento) = self.renderizar(ui, modelo) {
            match evento {
                EventoMuestraClasificados::IrAInicioDelMundial => {
                    modelo.grupos = Some(sortear_grupos(modelo));
                    return EventoGlobal::CambiarDeVistaAInicioMundial
                }
                EventoMuestraClasificados::IrAEditarGrupos => todo!(),
                EventoMuestraClasificados::IrASeleccionarEquiposClasificados => return EventoGlobal::CambiarDeVistaASeleccionClasificados,
            }
        }

        EventoGlobal::Continuar
    }
}

fn sortear_grupos(modelo: &ModeloSimulador) -> Vec<Grupo> {
    let equipos = modelo
        .clasificados
        .as_ref()
        .expect("No se definió la lista de clasificados");

    let mut rng = rand::rng();
    let mut equipos: Vec<EquipoParticipanteRef> =
        equipos.iter().map(|equipo| equipo.clone()).collect();
    equipos.shuffle(&mut rng);

    let mut grupos = Vec::with_capacity(equipos.len() / EQUIPOS_POR_GRUPO);
    let mut equipos = equipos.into_iter().peekable();
    let mut id = 0;

    loop {
        if equipos.peek().is_none() {
            break;
        }

        let mut grupo_vec = Vec::with_capacity(EQUIPOS_POR_GRUPO);

        for _ in 0..EQUIPOS_POR_GRUPO {
            let Some(equipo) = equipos.next() else {
                break;
            };

            grupo_vec.push(equipo);
        }

        match Grupo::try_new(id, grupo_vec) {
            Ok(grupo) => {
                grupos.push(grupo);
                id += 1;
            }
            Err(e) => eprintln!("ERROR: {e}"),
        }
    }

    grupos
}
