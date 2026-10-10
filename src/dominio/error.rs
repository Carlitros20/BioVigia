//! Errores del dominio.

use std::error::Error;
use std::fmt;

/// Error que se produce cuando no se cumple una regla del dominio.
///
/// Los constructores que pueden fallar devuelven `Result<Self, ErrorDominio>`;
/// los errores de programación se dejan para `panic!`. Cada estructura del
/// dominio añade aquí la variante de la invariante que comprueba.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorDominio {}

impl fmt::Display for ErrorDominio {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

impl Error for ErrorDominio {}
