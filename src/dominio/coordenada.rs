//! Coordenadas geográficas en grados decimales (#12).

use crate::dominio::error::ErrorDominio;

/// Hemisferio de una latitud: solo admite norte o sur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HemisferioNS {
    Norte,
    Sur,
}

/// Hemisferio de una longitud: solo admite este u oeste.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HemisferioEW {
    Este,
    Oeste,
}

impl TryFrom<char> for HemisferioNS {
    type Error = ErrorDominio;

    /// Lee la letra de hemisferio de una latitud (`N` o `S`).
    fn try_from(letra: char) -> Result<Self, Self::Error> {
        match letra.to_ascii_uppercase() {
            'N' => Ok(Self::Norte),
            'S' => Ok(Self::Sur),
            _ => Err(ErrorDominio::CoordenadaInvalida),
        }
    }
}

impl TryFrom<char> for HemisferioEW {
    type Error = ErrorDominio;

    /// Lee la letra de hemisferio de una longitud (`E` o `W`).
    fn try_from(letra: char) -> Result<Self, Self::Error> {
        match letra.to_ascii_uppercase() {
            'E' => Ok(Self::Este),
            'W' => Ok(Self::Oeste),
            _ => Err(ErrorDominio::CoordenadaInvalida),
        }
    }
}

/// Latitud en grados decimales, en [-90, 90].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Latitud(f64);

impl Latitud {
    /// Crea una latitud con signo (positiva al norte, negativa al sur).
    pub fn new(grados: f64) -> Result<Self, ErrorDominio> {
        if (-90.0..=90.0).contains(&grados) {
            Ok(Self(grados))
        } else {
            Err(ErrorDominio::CoordenadaInvalida)
        }
    }

    /// Crea una latitud a partir de los grados sin signo y su hemisferio.
    pub fn con_hemisferio(grados: f64, hemisferio: HemisferioNS) -> Result<Self, ErrorDominio> {
        if !(0.0..=90.0).contains(&grados) {
            return Err(ErrorDominio::CoordenadaInvalida);
        }
        match hemisferio {
            HemisferioNS::Norte => Self::new(grados),
            HemisferioNS::Sur => Self::new(-grados),
        }
    }

    pub fn grados(self) -> f64 {
        self.0
    }
}

/// Longitud en grados decimales, en [-180, 180].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Longitud(f64);

impl Longitud {
    /// Crea una longitud con signo (positiva al este, negativa al oeste).
    pub fn new(grados: f64) -> Result<Self, ErrorDominio> {
        if (-180.0..=180.0).contains(&grados) {
            Ok(Self(grados))
        } else {
            Err(ErrorDominio::CoordenadaInvalida)
        }
    }

    /// Crea una longitud a partir de los grados sin signo y su hemisferio.
    pub fn con_hemisferio(grados: f64, hemisferio: HemisferioEW) -> Result<Self, ErrorDominio> {
        if !(0.0..=180.0).contains(&grados) {
            return Err(ErrorDominio::CoordenadaInvalida);
        }
        match hemisferio {
            HemisferioEW::Este => Self::new(grados),
            HemisferioEW::Oeste => Self::new(-grados),
        }
    }

    pub fn grados(self) -> f64 {
        self.0
    }
}

/// Punto geográfico. Al recibir tipos distintos para latitud y longitud,
/// pasarlas en el orden equivocado no compila.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordenada {
    latitud: Latitud,
    longitud: Longitud,
}

impl Coordenada {
    pub fn new(latitud: Latitud, longitud: Longitud) -> Self {
        Self { latitud, longitud }
    }

    pub fn latitud(self) -> Latitud {
        self.latitud
    }

    pub fn longitud(self) -> Longitud {
        self.longitud
    }
}
