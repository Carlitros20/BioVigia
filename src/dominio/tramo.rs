//! Tramos de río (#12).

use std::fmt::Debug;

use crate::dominio::error::ErrorDominio;

/// Delimitación geográfica de un tramo de río.
///
/// No se fija todavía si será un polígono, una línea con margen u otra
/// representación, para no acoplar el dominio a una fuente concreta.
pub trait GeometriaTramo: Debug {}

/// Tramo de río, identificado por su nombre o código.
#[derive(Debug)]
pub struct TramoDeRio {
    codigo: String,
    geometria: Box<dyn GeometriaTramo>,
}

impl TramoDeRio {
    pub fn new(codigo: String, geometria: Box<dyn GeometriaTramo>) -> Result<Self, ErrorDominio> {
        if codigo.trim().is_empty() {
            return Err(ErrorDominio::CodigoTramoRequerido);
        }
        Ok(Self { codigo, geometria })
    }

    pub fn codigo(&self) -> &str {
        &self.codigo
    }

    pub fn geometria(&self) -> &dyn GeometriaTramo {
        self.geometria.as_ref()
    }
}

/// Dos tramos son el mismo si tienen el mismo código, aunque cambie su geometría.
impl PartialEq for TramoDeRio {
    fn eq(&self, otro: &Self) -> bool {
        self.codigo == otro.codigo
    }
}

impl Eq for TramoDeRio {}
