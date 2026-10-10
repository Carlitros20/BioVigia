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
    /// Nombre científico vacío o que no sigue la nomenclatura binomial.
    NombreCientificoInvalido,
    /// Nombre científico vigente repetido en la lista de nombres anteriores.
    NombreCientificoDuplicado,
    /// Avistamiento con el identificador original vacío.
    AvistamientoIncompleto,
    /// URL de foto vacía, con espacios o que no empieza por `http://` o `https://`.
    UrlFotoInvalida,
}

impl fmt::Display for ErrorDominio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mensaje = match self {
            Self::CoordenadaInvalida => "coordenada inválida",
            Self::CodigoTramoRequerido => "el tramo de río necesita un nombre o código",
            Self::NombreCientificoInvalido => "nombre científico inválido",
            Self::NombreCientificoDuplicado => {
                "el nombre científico vigente está repetido entre los anteriores"
            }
            Self::AvistamientoIncompleto => "el avistamiento necesita su identificador original",
            Self::UrlFotoInvalida => "URL de foto inválida",
        };
        f.write_str(mensaje)
    }
}

impl Error for ErrorDominio {}
