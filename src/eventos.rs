use std::any::TypeId;

use crate::Equipo;

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
    ClasificacionAleatoria,
    Salir,
}

/// Eventos para la vista `SeleccionClasificados`.
pub enum EventoSeleccionClasificados {
    AgregarEquipoClasificado(Equipo),
    QuitarEquipoClasificado(Equipo),
    IrAMenuPrincipal,
    IrAMuestraClasificados,
}

/// Eventos para la vista `MuestraClasificados`.
pub enum EventoMuestraClasificados {
    IrAInicioDelMundial,
    IrAFormacionDeGrupos,
    IrASeleccionarEquiposClasificados,
}
