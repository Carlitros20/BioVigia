use std::collections::BTreeSet;

use crate::dominio::error::ErrorDominio;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Especie {
    nombres: BTreeSet<String>,
    invasora: bool,
}

impl Especie {
    pub fn new(nombres: BTreeSet<String>, invasora: bool) -> Result<Self, ErrorDominio> {
        let nombres: BTreeSet<String> = nombres.iter().map(|n| n.trim().to_owned()).collect();
        if nombres.is_empty() || nombres.contains("") {
            return Err(ErrorDominio::NombreEspecieRequerido);
        }
        Ok(Self { nombres, invasora })
    }
}

#[derive(Debug, Clone)]
pub struct CatalogoEspecies {
    especies: Vec<Especie>,
}

impl CatalogoEspecies {
    pub fn new(especies: Vec<Especie>) -> Result<Self, ErrorDominio> {
        for (i, especie) in especies.iter().enumerate() {
            if especies[..i]
                .iter()
                .any(|otra| !otra.nombres.is_disjoint(&especie.nombres))
            {
                return Err(ErrorDominio::NombreEspecieRepetido);
            }
        }
        Ok(Self { especies })
    }
}
