use eframe::egui::{
    self, Align2, Area, Color32, Context, CornerRadius, Frame, Id, Margin, Order, Sense, Stroke,
    Window,
};

/// Posibles resultados de una ventana emergente.
pub enum ResultadoPopUp {
    Afirmativo,
    Negativo,

    /// El usuario no se decidió o cerró la ventana emergente.
    Nada,
}

/// Muestra una ventana de confirmación personalizada.
///
/// # Devuelve
/// - `Some(true)` si el usuario presionó el botón de afirmación.
/// - `Some(false)` si el usuario presionó el botón de afirmación.
/// - `None` en cualquier otro caso.
pub fn popup_confirmacion(ctx: &Context, titulo: &str, mensaje: &str) -> Option<ResultadoPopUp> {
    let screen_rect = ctx.content_rect();

    // Variable mutable para capturar el clic de este frame
    let mut resultado = None;

    // Capa bloqueante trasera (Evita clicks en el fondo)
    Area::new(Id::new("area_bloqueante"))
        .order(Order::Foreground)
        .fixed_pos(egui::pos2(0.0, 0.0))
        .interactable(true)
        .show(ctx, |ui| {
            ui.allocate_rect(screen_rect, Sense::click());
            ui.painter().rect_filled(
                screen_rect,
                CornerRadius::ZERO,
                Color32::from_black_alpha(140),
            );
        });

    // Ventana del Popup forzada al centro
    Window::new(titulo)
        .id(Id::new("ventana_confirmacion"))
        .order(Order::Tooltip)
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .show(ctx, |ui| {
            let mut frame = Frame::window(ui.style());
            frame.inner_margin = Margin::same(15);

            frame.show(ui, |ui| {
                // Dibujar el efecto biselado personalizado
                let rect = ui.max_rect();
                let painter = ui.painter();

                // Bisel superior/izquierdo (Luz)
                painter.line_segment(
                    [rect.left_bottom(), rect.left_top()],
                    Stroke::new(1.5, Color32::from_white_alpha(50)),
                );
                painter.line_segment(
                    [rect.left_top(), rect.right_top()],
                    Stroke::new(1.5, Color32::from_white_alpha(50)),
                );

                // Bisel inferior/derecho (Sombra)
                painter.line_segment(
                    [rect.right_top(), rect.right_bottom()],
                    Stroke::new(1.5, Color32::from_black_alpha(160)),
                );
                painter.line_segment(
                    [rect.right_bottom(), rect.left_bottom()],
                    Stroke::new(1.5, Color32::from_black_alpha(160)),
                );

                // Contenido del popup
                ui.vertical_centered(|ui| {
                    ui.label(mensaje);
                    ui.add_space(15.0);

                    ui.horizontal(|ui| {
                        let total_width = ui.available_width();
                        ui.add_space((total_width - 130.0) / 2.0);

                        // 2. ASIGNACIÓN DE VALORES AL INTERCEPTAR CLICS
                        if ui.button("  Sí  ").clicked() {
                            resultado = Some(ResultadoPopUp::Afirmativo);
                        }
                        ui.add_space(10.0);
                        if ui.button("  No  ").clicked() {
                            resultado = Some(ResultadoPopUp::Negativo);
                        }
                    });
                });
            });
        });

    resultado
}
