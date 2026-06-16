use eframe::egui;
use simulador_mundiales::app::AppSimulador;

fn main() -> Result<(), eframe::Error> {
    // Opciones nativas de la ventana (Viewport).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0]) // Ancho y alto inicial de la ventana
            .with_resizable(true)            // Permite al usuario estirar la ventana
            .with_title("Simulador de Mundiales de Fútbol"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Simulador de Mundiales de Fútbol",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::<AppSimulador>::default())
        }),
    )
}