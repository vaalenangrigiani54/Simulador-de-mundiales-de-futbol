use eframe::egui::{
    self, Align, Button, Color32, Label, Layout, Rect, RichText, ScrollArea, Stroke, StrokeKind,
};
use egui_extras::{Column, TableBuilder, TableRow};

/// Plantilla de configuración personalizada para una celda de tabla.
pub struct PropiedadesCelda {
    pub texto: RichText,
    pub fondo_rgb: Option<(u8, u8, u8)>,
    pub evento: Option<Box<dyn FnMut()>>,
}

/// Plantilla de configuración personalizada para una fila de tabla.
pub struct PropiedadesFila {
    pub datos: Vec<PropiedadesCelda>,
    pub altura: f32,
}

/// Agrega una tabla predefinida con sus encabezados de columna y datos dinámicos.
pub fn agregar_tabla(
    ui: &mut egui::Ui,
    pesos_columnas: &[u32],
    encabezados: &PropiedadesFila,
    filas: &mut Vec<PropiedadesFila>,
    altura_maxima: f32,
) {
    let num_columnas = pesos_columnas.len();
    if num_columnas == 0 {
        return;
    }

    // Estilos de líneas adaptados al tema.
    let color_lineas = ui.visuals().widgets.noninteractive.bg_stroke.color;
    let stroke_borde_exterior = Stroke::new(3.0, color_lineas);
    let stroke_grid_interna = Stroke::new(1.0, color_lineas.linear_multiply(0.6));

    // Renderizar tabla dentro de un scroll area con altura limitada
    let punto_inicio_y = ui.cursor().top();

    ScrollArea::vertical()
        .max_height(altura_maxima)
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            renderizar_tabla_interna(
                ui,
                pesos_columnas,
                encabezados,
                filas,
                num_columnas,
                stroke_grid_interna,
            );
        });

    dibujar_borde_exterior(ui, punto_inicio_y, stroke_borde_exterior);
}

/// Renderiza la tabla sin bordes externos.
fn renderizar_tabla_interna(
    ui: &mut egui::Ui,
    pesos: &[u32],
    encabezados: &PropiedadesFila,
    filas: &mut Vec<PropiedadesFila>,
    num_columnas: usize,
    stroke_grid_interna: Stroke,
) {
    // Eliminar spacing entre columnas para evitar huecos
    let spacing_original = ui.spacing_mut().item_spacing.x;
    ui.spacing_mut().item_spacing.x = 0.0;

    let ancho_disponible = ui.available_width();

    // Inicialización de la tabla.
    let mut table = TableBuilder::new(ui)
        .striped(true)
        .resizable(false)
        .cell_layout(Layout::left_to_right(Align::Center));

    table = configurar_columnas(table, pesos, ancho_disponible);

    // Renderizar encabezado y filas.
    table
        .header(encabezados.altura, |header| {
            renderizar_encabezado(header, encabezados, num_columnas, stroke_grid_interna);
        })
        .body(|body| {
            renderizar_cuerpo(body, filas, num_columnas, stroke_grid_interna);
        });

    // Restaurar spacing original
    ui.spacing_mut().item_spacing.x = spacing_original;
}

/// Configura las columnas de la tabla con anchos proporcionales basados en los pesos proporcionados.
fn configurar_columnas<'a>(
    mut table: TableBuilder<'a>,
    pesos: &'a [u32],
    ancho_disponible: f32,
) -> TableBuilder<'a> {
    let num_columnas = pesos.len();
    let suma_pesos: u32 = pesos.iter().sum();
    let suma_pesos_f32 = suma_pesos as f32;

    for &peso in pesos {
        let porcentaje = if suma_pesos_f32 > 0.0 {
            (peso as f32) / suma_pesos_f32
        } else {
            1.0 / num_columnas as f32
        };
        table = table.column(Column::initial(ancho_disponible * porcentaje).at_least(20.0));
    }

    table
}

/// Dibuja las celdas del encabezado aplicando colores y líneas verticales de la grid.
fn renderizar_encabezado(
    mut header: TableRow<'_, '_>,
    encabezados: &PropiedadesFila,
    num_columnas: usize,
    stroke_grid: Stroke,
) {
    for i in 0..num_columnas {
        header.col(|ui| {
            if let Some(celda) = encabezados.datos.get(i) {
                if let Some((r, g, b)) = celda.fondo_rgb {
                    ui.painter()
                        .rect_filled(ui.max_rect(), 0.0, Color32::from_rgb(r, g, b));
                }

                // Grid vertical (Línea derecha) excepto en el último elemento
                if i < num_columnas - 1 {
                    let rect = ui.max_rect();
                    ui.painter()
                        .line_segment([rect.right_top(), rect.right_bottom()], stroke_grid);
                }

                ui.centered_and_justified(|ui| {
                    let label = Label::new(celda.texto.clone()).truncate();
                    ui.add(label);
                });
            }
        });
    }
}

/// Procesa las filas con alturas variables y renderiza el contenido dinámico de cada celda.
fn renderizar_cuerpo(
    body: egui_extras::TableBody<'_>,
    filas: &mut Vec<PropiedadesFila>,
    num_columnas: usize,
    stroke_grid: Stroke,
) {
    let alturas_filas: Vec<f32> = filas.iter().map(|f| f.altura).collect();

    body.heterogeneous_rows(alturas_filas.into_iter(), |mut row| {
        let indice_fila = row.index();
        let fila = &mut filas[indice_fila];

        for i in 0..num_columnas {
            row.col(|ui| {
                let rect_celda = ui.max_rect();

                if let Some(celda) = fila.datos.get_mut(i) {
                    if let Some((r, g, b)) = celda.fondo_rgb {
                        ui.painter()
                            .rect_filled(rect_celda, 0.0, Color32::from_rgb(r, g, b));
                    }

                    // Grid horizontal (Línea inferior)
                    ui.painter().line_segment(
                        [rect_celda.left_bottom(), rect_celda.right_bottom()],
                        stroke_grid,
                    );

                    // Grid vertical (Línea derecha)
                    if i < num_columnas - 1 {
                        ui.painter().line_segment(
                            [rect_celda.right_top(), rect_celda.right_bottom()],
                            stroke_grid,
                        );
                    }

                    ui.centered_and_justified(|ui| {
                        if let Some(mut accion) = celda.evento.take() {
                            let boton = Button::new(celda.texto.clone()).wrap();
                            if ui.add(boton).clicked() {
                                accion();
                            }
                            celda.evento = Some(accion);
                        } else {
                            let label = Label::new(celda.texto.clone()).truncate();
                            ui.add(label);
                        }
                    });
                }
            });
        }
    });
}

/// Mide el área final que ocupó la tabla y pinta un marco rectangular contundente encima.
fn dibujar_borde_exterior(ui: &mut egui::Ui, punto_inicio_y: f32, stroke_borde: Stroke) {
    let punto_final_y = ui.cursor().top();
    let ancho_total = ui.available_width();
    let punto_inicio_x = ui.cursor().left();

    let rect_tabla_completa = Rect::from_min_max(
        egui::pos2(punto_inicio_x, punto_inicio_y),
        egui::pos2(punto_inicio_x + ancho_total, punto_final_y),
    );

    ui.painter()
        .rect_stroke(rect_tabla_completa, 0.0, stroke_borde, StrokeKind::Middle);
}
