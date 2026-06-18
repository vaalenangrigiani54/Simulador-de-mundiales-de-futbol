use eframe::egui;
use simulador_mundiales::app::AppSimulador;

fn main() -> Result<(), eframe::Error> {
    // Opciones nativas de la ventana (Viewport).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false),
        ..Default::default()
    };
    
    eframe::run_native(
        "Simulador de Mundiales de Fútbol",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::<AppSimulador>::default())
        }),
    )
}