use std::any::TypeId;

/// Decide entre continuar, cambiar de vista o salir del programa.
pub enum EventoGlobal {
    Continuar,
    CambiarDeVista(TypeId),
    Salir,
}

/// Eventos para la vista `MenuPrincipal`.
pub enum EventoMenuPrincipal {
    EstablecerCantidadDeEquipos(usize),
    IrASeleccionarEquiposClasificados,
    IrAClasificacionAleatoria,
    Salir,
}
