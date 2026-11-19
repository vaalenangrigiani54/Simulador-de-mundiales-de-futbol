use crate::eventos::EventoGlobal;
use crate::ventanas::{InfoGrupos, InicioMundial, MenuPrincipal, ModoInfoGrupos, MuestraClasificados, SeleccionClasificados};
use crate::{ModeloSimulador, VentanaInfo};
use eframe::egui::Ui;
use std::ops::{Deref, DerefMut};

/// Todas las vistas deben implementar este trait para poder ser mostradas.
pub trait Ventana: VentanaInfo {
    type Evento;

    /// Dibuja los componentes y devuelve una posible acción a ejecutarse (evento).
    ///
    /// Se puede pasar el estado global para renderizar cosas de manera dinámica.
    fn renderizar(&mut self, ui: &mut Ui, global: &ModeloSimulador) -> Option<Self::Evento>;
}

/// Este trait también lo deben implementar todos los tipos de vista.
///
/// Requiere `VentanaInfo` para que los controladores expongan las configuraciones
/// generales a través de `dyn Controlador`.
pub trait Controlador: VentanaInfo {
    fn actualizar(&mut self, ui: &mut Ui, modelo: &mut ModeloSimulador) -> EventoGlobal;
}

// Trait `Controlador` tratado como un objeto.
pub struct ControladorObj(Box<dyn Controlador>);

impl ControladorObj {
    fn new<V: Controlador + Ventana + Default + 'static>(ventana: V) -> Self {
        Self(Box::new(ventana))
    }

    pub fn new_menu_principal() -> Self {
        Self::new(MenuPrincipal::default())
    }

    pub fn new_seleccion_clasificados() -> Self {
        Self::new(SeleccionClasificados)
    }

    pub fn new_muestra_clasificados() -> Self {
        Self::new(MuestraClasificados::default())
    }

    pub fn new_inicio_mundial() -> Self {
        Self::new(InicioMundial::default())
    }

    pub fn new_info_grupos(modo: ModoInfoGrupos, idx_grupo: u8) -> Self {
        Self::new(InfoGrupos { modo, idx_grupo })
    }
}

impl Deref for ControladorObj {
    type Target = dyn Controlador;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl DerefMut for ControladorObj {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}
