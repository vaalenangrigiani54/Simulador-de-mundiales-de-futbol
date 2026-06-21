use eframe::egui::{Color32, RichText};

/// Plantilla de configuración personalizada para un texto.
pub fn crear_texto(
    str: impl Into<String>,
    tamanio: f32,
    color: impl Into<Color32>,
    negrita: bool,
) -> RichText {
    let mut texto = RichText::new(str).size(tamanio).color(color);

    if negrita {
        texto = texto.strong();
    }

    texto
}
