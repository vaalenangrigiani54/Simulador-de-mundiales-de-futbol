use crate::templates::texto::crear_texto;
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

/// Botón para ir para atrás en el programa.
pub fn agregar_boton_atras(ui: &mut egui::Ui, on_click: impl FnOnce()) {
    let texto = crear_texto("Atrás", 25.0, Color32::BLACK, true);

    let cfg = PropiedadesBoton {
        texto,
        dimensiones: Vec2 { x: 220.0, y: 30.0 },
        fondo_rgb: (255, 200, 200),
    };

    agregar_boton(ui, cfg, on_click);
}

/// Botón para ir para adelante en el programa.
pub fn agregar_boton_siguiente(ui: &mut egui::Ui, on_click: impl FnOnce()) {
    let texto = crear_texto("Siguiente", 25.0, Color32::BLACK, true);

    let cfg = PropiedadesBoton {
        texto,
        dimensiones: Vec2 { x: 220.0, y: 30.0 },
        fondo_rgb: (200, 255, 200),
    };

    agregar_boton(ui, cfg, on_click);
}
