//! Errores del dominio.

use std::error::Error;
use std::fmt;

/// Error que se produce cuando no se cumple una regla del dominio.
///
/// Los constructores que pueden fallar devuelven `Result<Self, ErrorDominio>`;
/// los errores de programación se dejan para `panic!`. Cada estructura del
/// dominio añade aquí la variante de la invariante que comprueba.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorDominio {
    /// Latitud o longitud fuera de rango (incluidos `NaN` e infinitos), o letra
    /// de hemisferio que no corresponde.
    CoordenadaInvalida,
    /// Tramo de río con el nombre/código vacío.
    CodigoTramoRequerido,
}

impl fmt::Display for ErrorDominio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mensaje = match self {
            Self::CoordenadaInvalida => "coordenada inválida",
            Self::CodigoTramoRequerido => "el tramo de río necesita un nombre o código",
        };
        f.write_str(mensaje)
    }
}

impl Error for ErrorDominio {}
