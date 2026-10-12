use crate::dominio::error::ErrorDominio;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HemisferioNS {
    Norte,
    Sur,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HemisferioEW {
    Este,
    Oeste,
}

impl TryFrom<char> for HemisferioNS {
    type Error = ErrorDominio;

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

    fn try_from(letra: char) -> Result<Self, Self::Error> {
        match letra.to_ascii_uppercase() {
            'E' => Ok(Self::Este),
            'W' => Ok(Self::Oeste),
            _ => Err(ErrorDominio::CoordenadaInvalida),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Latitud(f64);

impl Latitud {
    pub fn new(grados: f64) -> Result<Self, ErrorDominio> {
        if (-90.0..=90.0).contains(&grados) {
            Ok(Self(grados))
        } else {
            Err(ErrorDominio::CoordenadaInvalida)
        }
    }

    pub fn con_hemisferio(grados: f64, hemisferio: HemisferioNS) -> Result<Self, ErrorDominio> {
        if !(0.0..=90.0).contains(&grados) {
            return Err(ErrorDominio::CoordenadaInvalida);
        }
        match hemisferio {
            HemisferioNS::Norte => Self::new(grados),
            HemisferioNS::Sur => Self::new(-grados),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Longitud(f64);

impl Longitud {
    pub fn new(grados: f64) -> Result<Self, ErrorDominio> {
        if (-180.0..=180.0).contains(&grados) {
            Ok(Self(grados))
        } else {
            Err(ErrorDominio::CoordenadaInvalida)
        }
    }

    pub fn con_hemisferio(grados: f64, hemisferio: HemisferioEW) -> Result<Self, ErrorDominio> {
        if !(0.0..=180.0).contains(&grados) {
            return Err(ErrorDominio::CoordenadaInvalida);
        }
        match hemisferio {
            HemisferioEW::Este => Self::new(grados),
            HemisferioEW::Oeste => Self::new(-grados),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordenada {
    latitud: Latitud,
    longitud: Longitud,
}

impl Coordenada {
    pub fn new(latitud: Latitud, longitud: Longitud) -> Self {
        Self { latitud, longitud }
    }
}
