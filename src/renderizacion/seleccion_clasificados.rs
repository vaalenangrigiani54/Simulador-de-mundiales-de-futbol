use crate::eventos::EventoSeleccionClasificados;
use crate::mvc::Ventana;
use crate::templates::boton::{
    ALTURA_BOTON_PREDETERMINADA, agregar_boton, propiedades_boton_atras,
    propiedades_boton_siguiente,
};
use crate::templates::tabla::{PropiedadesCelda, PropiedadesFila, agregar_tabla};
use crate::templates::texto::crear_texto;
use crate::ventanas::SeleccionClasificados;
use crate::{EquipoInfo, ModeloSimulador};
use eframe::egui::{self, Color32};
use std::cell::RefCell;
use std::rc::Rc;

const MARGEN_VERTICAL_TABLA: f32 = 25.0;

impl Ventana for SeleccionClasificados {
    type Evento = EventoSeleccionClasificados;

    fn renderizar(&mut self, ui: &mut egui::Ui, global: &ModeloSimulador) -> Option<Self::Evento> {
        let mut proximo_evento = None;

        self.render_titulo(ui);
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(MARGEN_VERTICAL_TABLA);

        self.render_tabla_equipos(ui, global, &mut proximo_evento);
        ui.add_space(MARGEN_VERTICAL_TABLA);

        ui.columns(3, |columns| {
            let cantidad = global.cantidad_clasificados();
            self.render_boton_atras(&mut columns[0], &mut proximo_evento);
            self.render_label_seleccionados(&mut columns[1], cantidad, global.cant_participantes);
            self.render_boton_siguiente(
                &mut columns[2],
                cantidad,
                global.cant_participantes,
                &mut proximo_evento,
            );
        });

        proximo_evento
    }
}

impl SeleccionClasificados {
    fn render_titulo(&self, ui: &mut egui::Ui) {
        let texto = crear_texto("SELECCIONAR A LOS CLASIFICADOS", 50.0, Color32::BLACK, true);

        ui.vertical_centered(|ui| ui.heading(texto));
    }

    fn render_tabla_equipos(
        &self,
        ui: &mut egui::Ui,
        global: &ModeloSimulador,
        proximo_evento: &mut Option<EventoSeleccionClasificados>,
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
                PropiedadesCelda {
                    texto: crear_texto("Clasificado", 30.0, Color32::BLACK, true),
                    fondo_rgb: None,
                    evento: None,
                },
            ],
            altura: 40.0,
        };

        let evento_detectado = Rc::new(RefCell::new(None));
        let mut filas: Vec<PropiedadesFila> = global
            .get_equipos_disponibles()
            .iter()
            .enumerate()
            .map(|(idx, equipo)| {
                let clasificado = global.esta_clasificado(equipo);
                let clasificado_txt = if clasificado { "Sí" } else { "No" };
                let evento_detectado_celda = Rc::clone(&evento_detectado);

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
                        PropiedadesCelda {
                            texto: crear_texto(clasificado_txt, 22.0, Color32::BLACK, false),
                            fondo_rgb: None,
                            evento: Some(Box::new(move || {
                                *evento_detectado_celda.borrow_mut() = Some(
                                    EventoSeleccionClasificados::AgregarOQuitarEquipoClasificado(
                                        idx,
                                        !clasificado,
                                    ),
                                );
                            })),
                        },
                    ],
                    altura: 30.0,
                }
            })
            .collect();

        let altura_maxima =
            ui.available_height() - ALTURA_BOTON_PREDETERMINADA - MARGEN_VERTICAL_TABLA - 5.0;

        agregar_tabla(ui, &[3, 2, 1], &encabezado, &mut filas, altura_maxima);

        if let Some(evento) = evento_detectado.borrow_mut().take()
            && proximo_evento.is_none()
        {
            *proximo_evento = Some(evento);
        }
    }

    fn render_label_seleccionados(&self, ui: &mut egui::Ui, cantidad: usize, total: usize) {
        ui.vertical_centered(|ui| {
            let texto = crear_texto(
                format!("Cantidad seleccionados: {cantidad} / {total}"),
                20.0,
                Color32::BLACK,
                false,
            );

            ui.label(texto);
        });
    }

    fn render_boton_atras(
        &self,
        ui: &mut egui::Ui,
        proximo_evento: &mut Option<EventoSeleccionClasificados>,
    ) {
        let cfg = propiedades_boton_atras(true, None);

        agregar_boton(ui, cfg, || {
            if proximo_evento.is_none() {
                *proximo_evento = Some(EventoSeleccionClasificados::IrAMenuPrincipal)
            }
        });
    }

    fn render_boton_siguiente(
        &self,
        ui: &mut egui::Ui,
        cantidad: usize,
        total: usize,
        proximo_evento: &mut Option<EventoSeleccionClasificados>,
    ) {
        let cfg = propiedades_boton_siguiente(cantidad == total, None);

        agregar_boton(ui, cfg, || {
            if proximo_evento.is_none() {
                *proximo_evento = Some(EventoSeleccionClasificados::IrAMuestraClasificados)
            }
        });
    }
}
