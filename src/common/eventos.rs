use crate::ventanas::ModoInfoGrupos;

/// Decide entre continuar, cambiar de vista o salir del programa.
pub enum EventoGlobal {
    Continuar,
    CambiarDeVistaAMenuPrincipal,
    CambiarDeVistaASeleccionClasificados,
    CambiarDeVistaAMuestraClasificados,
    CambiarDeVistaAInicioMundial,
    CambiarDeVistaAInfoGrupos { modo: ModoInfoGrupos, idx_grupo: u8 },
    Salir,
}

/// Eventos para la ventana `MenuPrincipal`.
pub enum EventoMenuPrincipal {
    EstablecerCantidadDeEquipos(usize),
    IrASeleccionarEquiposClasificados,
    ClasificacionAleatoria,
    Salir,
}

/// Eventos para la ventana `SeleccionClasificados`.
pub enum EventoSeleccionClasificados {
    AgregarOQuitarEquipoClasificado(usize, bool),
    IrAMenuPrincipal,
    IrAMuestraClasificados,
}

/// Eventos para la ventana `MuestraClasificados`.
pub enum EventoMuestraClasificados {
    IrAInicioDelMundial,
    IrAEditarGrupos,
    IrASeleccionarEquiposClasificados,
}

/// Eventos para la ventana `InicioMundial`.
pub enum EventoInicioMundial {
    VerConformacionDeGrupos,
    IrAEditarGrupos,
    EmpezarSimulacion,
    VolverAlMenuPrincipal,
}

/// Eventos para la ventana `InfoGrupos`.
pub enum EventoInfoGrupos {
    VolverAInicioDelMundial,
    EquipoAnterior,
    EquipoSiguiente,
    VolverAlPartido
}