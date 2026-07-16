use crate::ModeloSimulador;
use crate::eventos::EventoGlobal;
use crate::mvc::ControladorObj;
use crate::ventanas::*;
use eframe::egui::{self, CentralPanel, Context, Frame, Ui, Vec2, ViewportCommand};
use std::any::TypeId;

pub struct AppSimulador {
    vista_actual: ControladorObj,
    modelo: Result<ModeloSimulador, String>,
}

impl Default for AppSimulador {
    fn default() -> Self {
        Self {
            vista_actual: ControladorObj::new::<MenuPrincipal>(),
            modelo: ModeloSimulador::try_new()
        }
    }
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
                match &mut self.modelo {
                    Ok(modelo) => {
                        match self.vista_actual.actualizar(ui, modelo) {
                            EventoGlobal::CambiarDeVista(type_id) => self.cambiar_de_vista(type_id),
                            EventoGlobal::Salir => Self::salir_del_programa(ui.ctx()),
                            _ => {}
                        }
                    },
                    Err(e) => Self::mostrar_error(e)
                }                
            });
    }
}

impl AppSimulador {
    fn cambiar_de_vista(&mut self, nueva_vista: TypeId) {
        if nueva_vista == TypeId::of::<MenuPrincipal>() {
            self.vista_actual = ControladorObj::new::<MenuPrincipal>();
        } else if nueva_vista == TypeId::of::<SeleccionClasificados>() {
            self.vista_actual = ControladorObj::new::<SeleccionClasificados>();
        } else if nueva_vista == TypeId::of::<MuestraClasificados>() {
            self.vista_actual = ControladorObj::new::<MuestraClasificados>();
        } else if nueva_vista == TypeId::of::<InicioMundial>() {
            self.vista_actual = ControladorObj::new::<InicioMundial>();
        } else {
            panic!("Tipo de vista no reconocido.")
        }
    }

    fn mostrar_error(e: impl AsRef<str>) {
        panic!("ERROR: {}", e.as_ref());
    }

    fn salir_del_programa(ctx: &Context) {
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

        let posicion_centrada = egui::pos2(centro_x, centro_y);
        ctx.send_viewport_cmd(ViewportCommand::OuterPosition(posicion_centrada));
    }
}
