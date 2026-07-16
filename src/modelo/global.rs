use crate::{Equipo, EquipoParticipanteRef, Grupo, Mundial};
use std::{collections::BTreeSet, fs::File, io::{BufRead, BufReader}};

/// Estructura principal del modelo global.
#[derive(Default)]
pub struct ModeloSimulador {
    /// Almacena todos los equipos afiliados a la FIFA.
    afiliados_fifa: Vec<Equipo>,

    /// Cantidad de participantes en el torneo.
    pub cant_participantes: usize,

    /// Campo auxiliar para los equipos clasificados.
    pub clasificados: Option<BTreeSet<EquipoParticipanteRef>>,

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
        return &self.afiliados_fifa;
    }

    /// Devuelve la cantidad actual de equipos clasificados
    pub fn cantidad_clasificados(&self) -> usize {
        if let Some(clasificados) = &self.clasificados {
            clasificados.len()
        } else {
            0
        }
    }

    /// Devuelve `true` si el equipo está clasificado. Caso contrario `false`.
    pub fn esta_clasificado(&self, equipo: &Equipo) -> bool {
        if let Some(clasificados) = &self.clasificados {
            clasificados.contains(&EquipoParticipanteRef::from(equipo.clone()))
        } else {
            false
        }
    }
}

fn inicializar_afiliados_fifa() -> Result<Vec<Equipo>, String> {
    let archivo = File::open("../assets/equipos.csv")
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