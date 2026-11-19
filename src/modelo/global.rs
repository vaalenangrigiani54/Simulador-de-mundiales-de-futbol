use crate::{Equipo, EquipoParticipanteRef, Grupo, Mundial};
use std::{collections::BTreeMap, fs::File, io::{BufRead, BufReader}};

/// Selecciones identificadas por un equipo inmutable, con estadísticas compartidas.
pub type EquiposClasificados = BTreeMap<Equipo, EquipoParticipanteRef>;

/// Estructura principal del modelo global.
#[derive(Default)]
pub struct ModeloSimulador {
    /// Almacena todos los equipos afiliados a la FIFA.
    afiliados_fifa: Vec<Equipo>,

    /// Cantidad de participantes en el torneo.
    pub cant_participantes: usize,

    /// Campo auxiliar para los equipos clasificados.
    pub clasificados: Option<EquiposClasificados>,

    /// Campo auxiliar para los grupos.
    pub grupos: Option<Vec<Grupo>>,

    /// Modelo del mundial. Si no está creado es None.
    pub mundial: Option<Mundial>,
}

impl ModeloSimulador {
    pub fn try_new() -> Result<Self, String> {
        let afiliados_fifa = inicializar_afiliados_fifa()?;
        
        Ok(Self {
            afiliados_fifa,
            cant_participantes: 16,
            clasificados: None,
            grupos: None,
            mundial: None,
        })
    }

    /// Devuelve todos los equipos afiliados a la FIFA.
    pub fn get_equipos_disponibles(&self) -> &Vec<Equipo> {
        &self.afiliados_fifa
    }

    /// Devuelve la cantidad actual de equipos clasificados
    pub fn cantidad_clasificados(&self) -> usize {
        self.clasificados.as_ref().map_or(0, BTreeMap::len)
    }

    /// Devuelve `true` si el equipo está clasificado. Caso contrario `false`.
    pub fn esta_clasificado(&self, equipo: &Equipo) -> bool {
        self.clasificados
            .as_ref()
            .is_some_and(|clasificados| clasificados.contains_key(equipo))
    }
}

fn inicializar_afiliados_fifa() -> Result<Vec<Equipo>, String> {
    let archivo = File::open("assets/equipos.csv")
        .map_err(|e| format!("No se pudo cargar la lista de equipos afiliados a la FIFA: {e}"))?;
    
    let reader = BufReader::new(archivo);
    let mut equipos = Vec::new();

    for linea_result in reader.lines() {
        if let Ok(linea) = linea_result
            && let Some((nombre, codigo)) = linea.rsplit_once(",") {
            equipos.push(Equipo::new(nombre, codigo));
        }
    }

    Ok(equipos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clasificacion_permanece_estable_al_cambiar_estadisticas() {
        let identidad = Equipo::new("Argentina", "ARG");
        let participante = EquipoParticipanteRef::from(identidad.clone());
        let mut modelo = ModeloSimulador {
            clasificados: Some(BTreeMap::from([(identidad.clone(), participante.clone())])),
            ..Default::default()
        };

        participante.borrow_mut().sumar_partido_ganado(true);
        assert!(modelo.esta_clasificado(&identidad));
        assert_eq!(modelo.cantidad_clasificados(), 1);
        assert!(modelo
            .clasificados
            .as_mut()
            .unwrap()
            .remove(&identidad)
            .is_some());
        assert!(!modelo.esta_clasificado(&identidad));
    }
}
