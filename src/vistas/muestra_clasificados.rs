use crate::eventos::EventoMuestraClasificados;
use crate::templates::boton::{
    ALTURA_BOTON_PREDETERMINADA, agregar_boton_atras, agregar_boton_siguiente,
};
use crate::templates::tabla::{PropiedadesCelda, PropiedadesFila, agregar_tabla};
use crate::templates::texto::crear_texto;
use crate::ventanas::MuestraClasificados;
use crate::{EquipoInfo, EquipoParticipanteRef, ModeloSimulador, Ventana};
use eframe::egui::{self, Color32};

const MARGEN_VERTICAL_TABLA: f32 = 25.0;

impl Ventana for MuestraClasificados {
    type Evento = EventoMuestraClasificados;

    fn renderizar(&self, ui: &mut egui::Ui, global: &ModeloSimulador) -> Option<Self::Evento> {
        let mut proximo_evento = None;

        render_titulo(ui, global.cant_participantes);
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(MARGEN_VERTICAL_TABLA);

        let clasificados = global
            .clasificados
            .as_ref()
            .expect("No se definió la lista de clasificados");
        render_tabla_clasificados(ui, clasificados);
        ui.add_space(MARGEN_VERTICAL_TABLA);

        ui.columns(2, |columns| {
            render_boton_atras(&mut columns[0], &mut proximo_evento);
            render_boton_siguiente(&mut columns[1], &mut proximo_evento);
        });

        proximo_evento
    }
}

fn render_titulo(ui: &mut egui::Ui, cantidad: usize) {
    let texto = crear_texto(
        format!("LOS {cantidad} CLASIFICADOS"),
        50.0,
        Color32::BLACK,
        true,
    );

    ui.vertical_centered(|ui| ui.heading(texto));
}

fn render_tabla_clasificados(ui: &mut egui::Ui, clasificados: &Vec<EquipoParticipanteRef>) {
    let col_equipo = PropiedadesCelda {
        texto: crear_texto("Equipo", 30.0, Color32::BLACK, true),
        fondo_rgb: None,
        evento: None,
    };

    let col_codigo = PropiedadesCelda {
        texto: crear_texto("Código FIFA", 30.0, Color32::BLACK, true),
        fondo_rgb: None,
        evento: None,
    };

    let encabezado = PropiedadesFila {
        datos: vec![col_equipo, col_codigo],
        altura: 40.0,
    };

    let mut filas: Vec<PropiedadesFila> = clasificados
        .iter()
        .map(|equipo| {
            let celda_equipo = PropiedadesCelda {
                texto: crear_texto(equipo.borrow().get_nombre(), 22.0, Color32::BLACK, false),
                fondo_rgb: None,
                evento: None,
            };

            let celda_codigo = PropiedadesCelda {
                texto: crear_texto(equipo.borrow().get_codigo(), 22.0, Color32::BLACK, false),
                fondo_rgb: None,
                evento: None,
            };

            PropiedadesFila {
                datos: vec![celda_equipo, celda_codigo],
                altura: 30.0,
            }
        })
        .collect();

    let altura_maxima =
        ui.available_height() - ALTURA_BOTON_PREDETERMINADA - MARGEN_VERTICAL_TABLA - 5.0;

    agregar_tabla(ui, &[1, 1], &encabezado, &mut filas, altura_maxima);
}

fn render_boton_atras(ui: &mut egui::Ui, proximo_evento: &mut Option<EventoMuestraClasificados>) {
    agregar_boton_atras(ui, || {
        if proximo_evento.is_none() {
            *proximo_evento = Some(EventoMuestraClasificados::IrASeleccionarEquiposClasificados)
        }
    });
}

fn render_boton_siguiente(
    ui: &mut egui::Ui,
    proximo_evento: &mut Option<EventoMuestraClasificados>,
) {
    agregar_boton_siguiente(ui, || {
        if proximo_evento.is_none() {
            *proximo_evento = Some(EventoMuestraClasificados::IrAInicioDelMundial)
        }
    });
}
