use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{Equipo, EquipoParticipante};

/// Múltiples referencias mutables a un equipo participante.
#[derive(Clone)]
pub struct EquipoParticipanteRef(Rc<RefCell<EquipoParticipante>>);

impl From<Equipo> for EquipoParticipanteRef {
    fn from(value: Equipo) -> Self {
        Self(Rc::new(RefCell::new(EquipoParticipante::new(value))))
    }
}

impl Deref for EquipoParticipanteRef {
    type Target = Rc<RefCell<EquipoParticipante>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EquipoParticipanteRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
