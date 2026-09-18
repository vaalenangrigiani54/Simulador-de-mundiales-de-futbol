use crate::config::{FONDO_POR_DEFECTO, MARGEN_EXTERIOR_POR_DEFECTO};
use eframe::egui::{Color32, Vec2};

/// Información de ventana que es segura para usar a través de trait-objects.
pub trait VentanaInfo {
    /// Ancho y Alto de la ventana (en píxeles).
    fn dimensiones(&self) -> Vec2;

    /// Color de fondo de la ventana.
    fn color_de_fondo(&self) -> Color32 {
        FONDO_POR_DEFECTO
    }

    /// Margen exterior de la ventana sin contar las dimensiones (Padding inverso).
    fn margen_exterior(&self) -> f32 {
        MARGEN_EXTERIOR_POR_DEFECTO
    }
}

/*============================================================================*/
/*--------------------------------- VENTANAS ---------------------------------*/
/*============================================================================*/

#[derive(Default)]
pub struct MenuPrincipal {
    pub(crate) mostrar_popup_salir: bool,
}

#[derive(Default)]
pub struct SeleccionClasificados;

#[derive(Default)]
pub struct MuestraClasificados {
    pub(crate) mostrar_popup_sorteo_o_eleccion: bool,
}

#[derive(Default)]
pub struct InicioMundial {
    pub(crate) mostrar_popup_menu: bool,
}

#[derive(Default)]
pub struct InfoGrupos {
    pub(crate) modo: ModoInfoGrupos,
    pub(crate) idx_grupo: u8
}

impl VentanaInfo for MenuPrincipal {
    fn dimensiones(&self) -> Vec2 {
        Vec2::new(800.0, 480.0)
    }
}

impl VentanaInfo for SeleccionClasificados {
    fn dimensiones(&self) -> Vec2 {
        Vec2::new(900.0, 600.0)
    }
}

impl VentanaInfo for MuestraClasificados {
    fn dimensiones(&self) -> Vec2 {
        Vec2::new(640.0, 640.0)
    }
}

impl VentanaInfo for InicioMundial {
    fn dimensiones(&self) -> Vec2 {
        Vec2::new(900.0, 400.0)
    }
}

impl VentanaInfo for InfoGrupos {
    fn dimensiones(&self) -> Vec2 {
        Vec2::new(1000.0, 500.0)
    }
}

/*============================================================================*/
/*---------------------------------- EXTRAS ----------------------------------*/
/*============================================================================*/

#[derive(Default)]
pub enum ModoInfoGrupos {
    #[default] SoloMostrar,
    ResultadosParciales(u8),
    ResultadosFinales
}