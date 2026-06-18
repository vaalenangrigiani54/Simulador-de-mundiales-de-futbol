use crate::{EquipoParticipanteRef, Fase, Grupo, Partido};

pub struct FaseDeGrupos {
    /// Por cada grupo, se definen en orden los partidos que se juegan.
    /// El campo booleano indica si el partido ya fue simulado o todavía no.
    partidos: Vec<Vec<Partido>>,

    /// (Grupo actual, Partido actual)
    partido_actual: Option<(usize, usize)>,
}

impl FaseDeGrupos {
    pub fn new(grupos: &Vec<Grupo>) -> Self {
        Self {
            partidos: generar_partidos(grupos),
            partido_actual: None,
        }
    }

    pub fn get_idx_partido_actual(&self) -> (usize, usize) {
        self.partido_actual.unwrap_or_default()
    }
}

impl Fase for FaseDeGrupos {
    fn get_partido_actual(&self) -> &Partido {
        let idx = self.get_idx_partido_actual();
        &self.partidos[idx.0][idx.1]
    }

    fn siguiente_partido(&mut self) -> bool {
        if let Some((mut idx_grupo, mut idx_partido)) = self.partido_actual {
            let Some(partidos_grupo_actual) = self.partidos.get(idx_grupo) else {
                return false;
            };

            idx_partido += 1;
            if idx_partido >= partidos_grupo_actual.len() {
                idx_grupo += 1;
                idx_partido = 0;
            }

            self.partido_actual = Some((idx_grupo, idx_partido));
            if idx_grupo >= self.partidos.len() {
                return false;
            }
        } else {
            self.partido_actual = Some((0, 0));
        }

        let (idx_grupo, idx_partido) = self.partido_actual.unwrap_or_default();
        let partido = &mut self.partidos[idx_grupo][idx_partido];
        partido.simular();

        true
    }

    fn partido_anterior(&mut self) -> bool {
        if let Some((mut idx_grupo, mut idx_partido)) = self.partido_actual {
            if idx_partido == 0 {
                if idx_grupo == 0 {
                    self.partido_actual = None;
                    return false;
                }

                idx_grupo -= 1;
            } else {
                idx_partido -= 1;
            }

            self.partido_actual = Some((idx_grupo, idx_partido));
        } else {
            return false;
        }

        true
    }
}

fn generar_partidos(grupos: &Vec<Grupo>) -> Vec<Vec<Partido>> {
    let mut partidos = Vec::with_capacity(grupos.len());

    for grupo in grupos {
        let partidos_grupo = generar_partidos_un_grupo(grupo);
        partidos.push(partidos_grupo);
    }

    partidos
}

/// Se utiliza el algoritmo de Berger para generar los partidos de manera ordenada y equilibrada.
fn generar_partidos_un_grupo(grupo: &Grupo) -> Vec<Partido> {
    let mut equipos: Vec<Option<EquipoParticipanteRef>> = grupo
        .get_equipos()
        .iter()
        .map(|(_, equipo)| Some(equipo.clone()))
        .collect();

    if equipos.len() & 1 == 1 {
        equipos.push(None);
    }

    let n = equipos.len();
    let total_rondas = n - 1;
    let partidos_por_ronda = n >> 1;
    let mut fixture_grupo = Vec::with_capacity(total_rondas * partidos_por_ronda);
    let mut rotacion = crear_semilla_de_rotacion(n);

    for ronda in 0..total_rondas {
        // Partido del equipo fijo (Pivote en índice 0) contra el primero de la rotación
        let equipo_pivote = equipos[0].clone();
        let equipo_rival = equipos[rotacion[0]].clone();

        // El pivote alterna localía estrictamente por ronda.
        let pivote_es_local = ronda & 1 == 0;

        if let Some(partido) = crear_partido(equipo_pivote, equipo_rival, pivote_es_local) {
            fixture_grupo.push(partido);
        }

        // Resto de los partidos de la ronda
        // Recorremos la mesa de rotación emparejando los elementos continuos de dos en dos
        for i in 1..partidos_por_ronda {
            let idx_a = rotacion[(i << 1) - 1];
            let idx_b = rotacion[i << 1];
            let equipo_a = equipos[idx_a].clone();
            let equipo_b = equipos[idx_b].clone();

            // Alternamos localía de manera equilibrada usando el índice del partido.
            let a_es_local = i & 1 == 0;

            if let Some(partido) = crear_partido(equipo_a, equipo_b, a_es_local) {
                fixture_grupo.push(partido);
            }
        }

        // Rotar la lista hacia la derecha para la siguiente fecha
        rotacion.rotate_right(1);
    }

    fixture_grupo
}

/// Esto se hace para lograr que la primera ronda sea secuencial (A vs B, C vs D...).
fn crear_semilla_de_rotacion(n: usize) -> Vec<usize> {
    let mut rotacion = Vec::with_capacity(n - 1);
    let mut izq = 1;
    let mut der = n - 1;

    while izq <= der {
        if izq == der {
            rotacion.push(izq);
        } else {
            rotacion.push(izq);
            rotacion.push(der);
        }

        izq += 1;
        der -= 1;
    }

    rotacion
}

/// Función auxiliar para crear un partido alternando las localías.
fn crear_partido(
    equipo_a: Option<EquipoParticipanteRef>,
    equipo_b: Option<EquipoParticipanteRef>,
    a_es_local: bool,
) -> Option<Partido> {
    let (Some(equipo_a), Some(equipo_b)) = (equipo_a, equipo_b) else {
        return None;
    };

    // Fórmula de paridad: distribuye las localías dinámicamente.
    if a_es_local {
        Some(Partido::new(equipo_a, equipo_b))
    } else {
        Some(Partido::new(equipo_b, equipo_a))
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use super::*;
    use crate::{Equipo, modelo::equipo::IDEquipo};

    #[test]
    fn test_algoritmo_berger_4_equipos() {
        let cod_madrid = "RMA";
        let cod_barcelona = "FCB";
        let cod_bayern = "BAY";
        let cod_psg = "PSG";
        let madrid = Equipo::new("Real Madrid", cod_madrid).clasificarse();
        let barcelona = Equipo::new("F.C. Barcelona", cod_barcelona).clasificarse();
        let bayern = Equipo::new("Bayern Munich", cod_bayern).clasificarse();
        let psg = Equipo::new("París Saint Germain", cod_psg).clasificarse();

        let equipos = vec![
            Rc::new(RefCell::new(madrid)),
            Rc::new(RefCell::new(barcelona)),
            Rc::new(RefCell::new(bayern)),
            Rc::new(RefCell::new(psg)),
        ];

        let orden_esperado = vec![
            (cod_madrid, cod_barcelona),
            (cod_bayern, cod_psg),
            (cod_bayern, cod_madrid),
            (cod_psg, cod_barcelona),
            (cod_madrid, cod_psg),
            (cod_barcelona, cod_bayern),
        ];

        let grupo = Grupo::try_new(1, equipos).expect("No se pudo crear el grupo");
        let partidos = generar_partidos_un_grupo(&grupo);

        let cant_partidos_esperada = orden_esperado.len();
        assert_eq!(
            partidos.len(),
            cant_partidos_esperada,
            "La cantidad de partidos generados debe ser {cant_partidos_esperada}"
        );

        for i in 0..cant_partidos_esperada {
            let (cod_loc_esperado, cod_vis_esperado) = orden_esperado
                .get(i)
                .expect("Error al obtener los equipos enfrentados");
            let partido = partidos
                .get(i)
                .expect("Error al obtener el partido generado");
            let loc_actual = partido.get_local();
            let vis_actual = partido.get_visitante();

            assert_eq!(
                loc_actual.get_codigo(),
                *cod_loc_esperado,
                "El equipo local de este partido debe ser '{cod_loc_esperado}'"
            );
            assert_eq!(
                vis_actual.get_codigo(),
                *cod_vis_esperado,
                "El equipo visitante de este partido debe ser '{cod_vis_esperado}'"
            );
        }
    }
}
