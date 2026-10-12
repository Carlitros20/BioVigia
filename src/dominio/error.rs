use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorDominio {
    CoordenadasInvalidas,
    CodigoTramoRequerido,
    NombreEspecieRequerido,
    NombreEspecieRepetido,
    AvistamientoIncompleto,
    UrlFotoInvalida,
    DepredacionSobreMismaEspecie,
    FaseVitalRequerida,
    DepredacionRepetida,
}

impl fmt::Display for ErrorDominio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mensaje = match self {
            Self::CoordenadasInvalidas => "coordenadas inválidas",
            Self::CodigoTramoRequerido => "el tramo de río necesita un nombre o código",
            Self::NombreEspecieRequerido => "la especie necesita al menos un nombre no vacío",
            Self::NombreEspecieRepetido => "el mismo nombre está en dos especies del catálogo",
            Self::AvistamientoIncompleto => "el avistamiento necesita su identificador original y el nombre de la especie",
            Self::UrlFotoInvalida => "URL de foto inválida",
            Self::DepredacionSobreMismaEspecie => "el depredador y la presa no pueden ser la misma especie",
            Self::FaseVitalRequerida => "la depredación debe afectar al menos a una fase vital",
            Self::DepredacionRepetida => "la misma depredación entre dos especies está repetida",
        };
        f.write_str(mensaje)
    }
}

impl Error for ErrorDominio {}
