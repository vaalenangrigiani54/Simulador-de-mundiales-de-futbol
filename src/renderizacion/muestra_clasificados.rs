use std::collections::BTreeSet;

use crate::eventos::EventoMuestraClasificados;
use crate::mvc::Ventana;
use crate::templates::boton::{
    ALTURA_BOTON_PREDETERMINADA, agregar_boton, propiedades_boton_atras,
    propiedades_boton_siguiente,
};
use crate::templates::popup::{ResultadoPopUp, popup_confirmacion};
use crate::templates::tabla::{PropiedadesCelda, PropiedadesFila, agregar_tabla};
use crate::templates::texto::crear_texto;
use crate::ventanas::MuestraClasificados;
use crate::{EquipoInfo, EquipoParticipanteRef, ModeloSimulador};
use eframe::egui::{self, Color32};

const MARGEN_VERTICAL_TABLA: f32 = 25.0;

impl Ventana for MuestraClasificados {
    type Evento = EventoMuestraClasificados;

    fn renderizar(&mut self, ui: &mut egui::Ui, global: &ModeloSimulador) -> Option<Self::Evento> {
        let mut proximo_evento = None;

        self.render_titulo(ui, global.cant_participantes);
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(MARGEN_VERTICAL_TABLA);

        let clasificados = global
            .clasificados
            .as_ref()
            .expect("No se definió la lista de clasificados");
        self.render_tabla_clasificados(ui, clasificados);
        ui.add_space(MARGEN_VERTICAL_TABLA);

        ui.columns(2, |columns| {
            self.render_boton_atras(&mut columns[0], &mut proximo_evento);
            self.render_boton_siguiente(&mut columns[1], &mut proximo_evento);
        });

        proximo_evento
    }
}

impl MuestraClasificados {
    fn render_titulo(&self, ui: &mut egui::Ui, cantidad: usize) {
        let texto = crear_texto(
            format!("LOS {cantidad} CLASIFICADOS"),
            50.0,
            Color32::BLACK,
            true,
        );

        ui.vertical_centered(|ui| ui.heading(texto));
    }

    fn render_tabla_clasificados(
        &self,
        ui: &mut egui::Ui,
        clasificados: &BTreeSet<EquipoParticipanteRef>,
    ) {
        let encabezado = PropiedadesFila {
            datos: vec![
                PropiedadesCelda {
                    texto: crear_texto("Equipo", 30.0, Color32::BLACK, true),
                    fondo_rgb: None,
                    evento: None,
                },
                PropiedadesCelda {
                    texto: crear_texto("Código FIFA", 30.0, Color32::BLACK, true),
                    fondo_rgb: None,
                    evento: None,
                },
            ],
            altura: 40.0,
        };

        let mut filas: Vec<PropiedadesFila> = clasificados
            .iter()
            .map(|equipo| {
                let equipo = equipo.borrow();

                PropiedadesFila {
                    datos: vec![
                        PropiedadesCelda {
                            texto: crear_texto(equipo.get_nombre(), 22.0, Color32::BLACK, false),
                            fondo_rgb: None,
                            evento: None,
                        },
                        PropiedadesCelda {
                            texto: crear_texto(equipo.get_codigo(), 22.0, Color32::BLACK, false),
                            fondo_rgb: None,
                            evento: None,
                        },
                    ],
                    altura: 30.0,
                }
            })
            .collect();

        let altura_maxima =
            ui.available_height() - ALTURA_BOTON_PREDETERMINADA - MARGEN_VERTICAL_TABLA - 5.0;

        agregar_tabla(ui, &[1, 1], &encabezado, &mut filas, altura_maxima);
    }

    fn render_boton_atras(
        &self,
        ui: &mut egui::Ui,
        proximo_evento: &mut Option<EventoMuestraClasificados>,
    ) {
        let cfg = propiedades_boton_atras(true, None);

        agregar_boton(ui, cfg, || {
            if proximo_evento.is_none() {
                *proximo_evento = Some(EventoMuestraClasificados::IrASeleccionarEquiposClasificados)
            }
        });
    }

    fn render_boton_siguiente(
        &mut self,
        ui: &mut egui::Ui,
        proximo_evento: &mut Option<EventoMuestraClasificados>,
    ) {
        let cfg = propiedades_boton_siguiente(true, None);

        agregar_boton(ui, cfg, || {
            if proximo_evento.is_none() {
                self.mostrar_popup_sorteo_o_eleccion = true;
            }
        });

        if self.mostrar_popup_sorteo_o_eleccion
            && let Some(resultado) = popup_confirmacion(
                ui.ctx(),
                "Conformación de grupos",
                "¿Desea configurar los grupos manualmente?",
            )
        {
            match resultado {
                ResultadoPopUp::Afirmativo => {
                    *proximo_evento = Some(EventoMuestraClasificados::IrAEditarGrupos)
                }
                ResultadoPopUp::Negativo => {
                    *proximo_evento = Some(EventoMuestraClasificados::IrAInicioDelMundial)
                }
                _ => {}
            }

            self.mostrar_popup_sorteo_o_eleccion = false;
        }
    }
}
