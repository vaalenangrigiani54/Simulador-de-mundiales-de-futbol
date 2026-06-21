use crate::eventos::{EventoGlobal, EventoMenuPrincipal};
use crate::ventanas::{MenuPrincipal, MuestraClasificados};
use crate::{Controlador, Equipo, EquipoParticipanteRef, ModeloSimulador, Ventana};
use eframe::egui;
use rand::seq::SliceRandom;
use std::any::TypeId;

impl Controlador for MenuPrincipal {
    fn actualizar(&mut self, ui: &mut egui::Ui, modelo: &mut ModeloSimulador) -> EventoGlobal {
        if let Some(evento) = self.renderizar(ui, modelo) {
            match evento {
                EventoMenuPrincipal::EstablecerCantidadDeEquipos(cantidad) => {
                    modelo.cant_participantes = cantidad
                }
                EventoMenuPrincipal::IrASeleccionarEquiposClasificados => todo!(),
                EventoMenuPrincipal::ClasificacionAleatoria => {
                    let disponibles = modelo.get_equipos_disponibles();
                    modelo.clasificados = Some(elegir_clasificados_aleatoriamente(
                        modelo.cant_participantes,
                        disponibles,
                    ));
                    return EventoGlobal::CambiarDeVista(TypeId::of::<MuestraClasificados>());
                }
                EventoMenuPrincipal::Salir => return EventoGlobal::Salir,
            }
        }

        EventoGlobal::Continuar
    }
}

fn elegir_clasificados_aleatoriamente(
    cantidad: usize,
    afiliados_fifa: &Vec<Equipo>,
) -> Vec<EquipoParticipanteRef> {
    let mut rng = rand::rng();
    let mut pool: Vec<usize> = (0..afiliados_fifa.len()).collect();

    pool.shuffle(&mut rng);
    pool.truncate(cantidad);

    let mut clasificados: Vec<EquipoParticipanteRef> = pool
        .into_iter()
        .map(|idx| EquipoParticipanteRef::from(&afiliados_fifa[idx]))
        .collect();

    clasificados.sort();
    clasificados
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clasificacion_aleatoria() {
        let simulador = ModeloSimulador::default();
        let afiliados_fifa = simulador.get_equipos_disponibles();

        let cantidad = 32;
        let clasificados = elegir_clasificados_aleatoriamente(cantidad, afiliados_fifa);

        println!("{cantidad} CLASIFICADOS:");
        for equipo in clasificados {
            println!("{}", equipo.borrow())
        }
    }
}
