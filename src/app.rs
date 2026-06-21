use std::any::TypeId;
use crate::{ControladorObj, EventoGlobal, ModeloSimulador};
use crate::ventanas::MenuPrincipal;
use eframe::egui::{CentralPanel, Context, Frame, Pos2, Ui, Vec2, ViewportCommand};

#[derive(Default)]
pub struct AppSimulador {
    vista_actual: ControladorObj,
    modelo: ModeloSimulador,
}

impl eframe::App for AppSimulador {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let tamanio_vista = self.vista_actual.dimensiones();

        let ctx = ui.ctx();
        redimensionar_ventana(ctx, tamanio_vista);
        centrar_ventana(ctx, tamanio_vista);

        CentralPanel::default()
            .frame(
                Frame::NONE
                    .fill(self.vista_actual.color_de_fondo())
                    .inner_margin(self.vista_actual.margen_exterior()),
            )
            .show_inside(ui, |ui| {
                match self.vista_actual.actualizar(ui, &mut self.modelo) {
                    EventoGlobal::CambiarDeVista(type_id) => self.cambiar_de_vista(type_id),
                    EventoGlobal::Salir => self.salir_del_programa(ui.ctx()),
                    _ => {}
                }
            });
    }
}

impl AppSimulador {
    fn cambiar_de_vista(&mut self, nueva_vista: TypeId) {
        if nueva_vista == TypeId::of::<MenuPrincipal>() {
            self.vista_actual = ControladorObj::new::<MenuPrincipal>();
        } else {
            panic!("Tipo de vista no reconocido.")
        }
    }

    fn salir_del_programa(&self, ctx: &Context) {
        ctx.send_viewport_cmd(ViewportCommand::Close);
    }
}

/// Cambia el tamaño de la ventana de manera dinámica.
fn redimensionar_ventana(ctx: &Context, dimensiones: Vec2) {
    ctx.send_viewport_cmd(ViewportCommand::InnerSize(dimensiones));
}

/// Centra la ventana teniendo en cuenta la resolución del monitor.
fn centrar_ventana(ctx: &Context, dimensiones: Vec2) {
    if let Some(monitor_size) = ctx.input(|i| i.viewport().monitor_size) {
        let centro_x = (monitor_size.x / 2.0) - (dimensiones.x / 2.0);
        let centro_y = (monitor_size.y / 2.0) - (dimensiones.y / 2.0);

        let posicion_centrada = Pos2::new(centro_x, centro_y);
        ctx.send_viewport_cmd(ViewportCommand::OuterPosition(posicion_centrada));
    }
}
