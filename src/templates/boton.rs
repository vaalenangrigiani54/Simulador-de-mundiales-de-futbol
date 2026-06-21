use eframe::egui::{self, Button, Color32, RichText, Vec2};

/// Plantilla de configuración personalizada para el botón.
pub struct PropiedadesBoton {
    pub texto: RichText,
    pub dimensiones: Vec2,
    pub fondo_rgb: (u8, u8, u8),
}

/// Agrega directamente a la UI un botón junto con una acción que se ejecuta al hacer click.
pub fn agregar_boton(ui: &mut egui::Ui, cfg: PropiedadesBoton, on_click: impl FnOnce()) {
    let boton = Button::new(cfg.texto).fill(Color32::from_rgb(
        cfg.fondo_rgb.0,
        cfg.fondo_rgb.1,
        cfg.fondo_rgb.2,
    ));

    if ui.add_sized(cfg.dimensiones, boton).clicked() {
        on_click()
    }
}
