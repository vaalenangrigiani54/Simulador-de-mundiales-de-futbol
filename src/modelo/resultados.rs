/// Indica dónde quedó (o donde está quedando actualmente) un equipo en el campeonato.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ResultadoEquipo {
    FaseDeGrupos,
    Dieciseisavos,
    Octavos,
    Cuartos,
    Semifinalista,
    TercerLugar,
    Subcampeon,
    Campeon,
}
