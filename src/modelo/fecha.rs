use crate::Partido;

pub struct FechaDelTorneo(Vec<(usize, Partido)>);


impl FechaDelTorneo {
    pub fn new() -> Self {
        Self(Vec::with_capacity(3))
    }

    pub fn agregar_partido(&mut self, id_grupo: usize, partido: Partido) {
        self.0.push((id_grupo, partido));
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut Partido)> {
        self.0.iter_mut().map(|(id_grupo, partido)| (*id_grupo, partido))
    }
}