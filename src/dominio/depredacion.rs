//! Relación de depredación entre especies (#11).

use std::collections::BTreeSet;

use crate::dominio::avistamiento::Fuente;
use crate::dominio::error::ErrorDominio;
use crate::dominio::especie::EspecieId;

/// Fase vital de la presa a la que afecta una depredación.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FaseVital {
    HuevoEmbrion,
    LarvaAlevin,
    Juvenil,
    Adulto,
}

impl FaseVital {
    /// "Todas" no es una fase: se representa como el conjunto de las cuatro.
    pub const TODAS: [FaseVital; 4] = [
        FaseVital::HuevoEmbrion,
        FaseVital::LarvaAlevin,
        FaseVital::Juvenil,
        FaseVital::Adulto,
    ];
}

/// Una especie depreda a otra en unas fases vitales concretas de la presa. No
/// tiene identidad propia: la definen depredador, presa y fases afectadas.
#[derive(Debug, Clone)]
pub struct Depredacion {
    depredador: EspecieId,
    presa: EspecieId,
    fases_afectadas: BTreeSet<FaseVital>,
    fuente: Option<Fuente>,
}

impl Depredacion {
    /// La fuente es opcional: puede no existir en relaciones cargadas desde
    /// catálogos internos.
    pub fn new(
        depredador: EspecieId,
        presa: EspecieId,
        fases_afectadas: BTreeSet<FaseVital>,
        fuente: Option<Fuente>,
    ) -> Result<Self, ErrorDominio> {
        if depredador == presa {
            return Err(ErrorDominio::DepredacionSobreMismaEspecie);
        }
        if fases_afectadas.is_empty() {
            return Err(ErrorDominio::FaseVitalRequerida);
        }
        Ok(Self {
            depredador,
            presa,
            fases_afectadas,
            fuente,
        })
    }

    pub fn depredador(&self) -> EspecieId {
        self.depredador
    }

    pub fn presa(&self) -> EspecieId {
        self.presa
    }

    pub fn fases_afectadas(&self) -> &BTreeSet<FaseVital> {
        &self.fases_afectadas
    }

    pub fn fuente(&self) -> Option<Fuente> {
        self.fuente
    }
}

/// La fuente es solo una referencia de procedencia: no forma parte de lo que
/// define la relación.
impl PartialEq for Depredacion {
    fn eq(&self, otra: &Self) -> bool {
        self.depredador == otra.depredador
            && self.presa == otra.presa
            && self.fases_afectadas == otra.fases_afectadas
    }
}

impl Eq for Depredacion {}
