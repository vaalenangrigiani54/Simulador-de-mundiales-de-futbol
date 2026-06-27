use crate::templates::texto::crear_texto;
use eframe::egui::{self, Button, Color32, RichText, Vec2};

/// Plantilla de configuración personalizada para el botón.
pub struct PropiedadesBoton {
    pub texto: RichText,
    pub dimensiones: Vec2,
    pub fondo_rgb: (u8, u8, u8),
    pub habilitado: bool
}

/// Altura por defecto (en píxeles) de un botón.
pub const ALTURA_BOTON_PREDETERMINADA: f32 = 30.0;

/// Agrega directamente a la UI un botón junto con una acción que se ejecuta al hacer click.
pub fn agregar_boton(ui: &mut egui::Ui, cfg: PropiedadesBoton, on_click: impl FnOnce()) {
    let boton = Button::new(cfg.texto).fill(Color32::from_rgb(
        cfg.fondo_rgb.0,
        cfg.fondo_rgb.1,
        cfg.fondo_rgb.2,
    ));

    ui.add_enabled_ui(cfg.habilitado, |ui| {
        if ui.add_sized(cfg.dimensiones, boton).clicked() {
            on_click()
        }
    });
}

/// Devuelve las propiedades del botón para ir para atrás en el programa.
pub fn propiedades_boton_atras(habilitado: bool, sobreescribir_texto: Option<&str>) -> PropiedadesBoton {
    let texto_str = sobreescribir_texto.unwrap_or("Atrás");
    
    PropiedadesBoton {
        texto: crear_texto(texto_str, 25.0, Color32::BLACK, true),
        dimensiones: Vec2 {
            x: 220.0,
            y: ALTURA_BOTON_PREDETERMINADA,
        },
        fondo_rgb: (255, 200, 200),
        habilitado
    }
}

/// Devuelve las propiedades del botón para volver al menú principal.
pub fn propiedades_boton_menu(habilitado: bool, sobreescribir_texto: Option<&str>) -> PropiedadesBoton {
    let texto_str = sobreescribir_texto.unwrap_or("Menú principal");
    
    PropiedadesBoton {
        texto: crear_texto(texto_str, 25.0, Color32::BLACK, true),
        dimensiones: Vec2 {
            x: 220.0,
            y: ALTURA_BOTON_PREDETERMINADA,
        },
        fondo_rgb: (255, 255, 200),
        habilitado
    }
}

/// Devuelve las propiedades del botón para ir para adelante en el programa.
pub fn propiedades_boton_siguiente(habilitado: bool, sobreescribir_texto: Option<&str>) -> PropiedadesBoton {
    let texto_str = sobreescribir_texto.unwrap_or("Siguiente");
    
    PropiedadesBoton {
        texto: crear_texto(texto_str, 25.0, Color32::BLACK, true),
        dimensiones: Vec2 {
            x: 220.0,
            y: ALTURA_BOTON_PREDETERMINADA,
        },
        fondo_rgb: (200, 255, 200),
        habilitado
    }
}
