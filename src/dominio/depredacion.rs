use std::collections::BTreeSet;

use crate::dominio::avistamiento::Fuente;
use crate::dominio::error::ErrorDominio;
use crate::dominio::especie::EspecieId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FaseVital {
    HuevoEmbrion,
    LarvaAlevin,
    Juvenil,
    Adulto,
}

impl FaseVital {
    pub const TODAS: [FaseVital; 4] = [
        FaseVital::HuevoEmbrion,
        FaseVital::LarvaAlevin,
        FaseVital::Juvenil,
        FaseVital::Adulto,
    ];
}

#[derive(Debug, Clone)]
pub struct Depredacion {
    depredador: EspecieId,
    presa: EspecieId,
    fases_afectadas: BTreeSet<FaseVital>,
    fuente: Option<Fuente>,
}

impl Depredacion {
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

impl PartialEq for Depredacion {
    fn eq(&self, otra: &Self) -> bool {
        self.depredador == otra.depredador
            && self.presa == otra.presa
            && self.fases_afectadas == otra.fases_afectadas
    }
}

impl Eq for Depredacion {}
