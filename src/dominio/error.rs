use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorDominio {
    CoordenadaInvalida,
    CodigoTramoRequerido,
    NombreCientificoInvalido,
    NombreCientificoDuplicado,
    NombreComunRequerido,
    AvistamientoIncompleto,
    UrlFotoInvalida,
    DepredacionSobreMismaEspecie,
    FaseVitalRequerida,
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
            Self::NombreComunRequerido => "la especie necesita un nombre común",
            Self::AvistamientoIncompleto => "el avistamiento necesita su identificador original",
            Self::UrlFotoInvalida => "URL de foto inválida",
            Self::DepredacionSobreMismaEspecie => {
                "el depredador y la presa no pueden ser la misma especie"
            }
            Self::FaseVitalRequerida => "la depredación debe afectar al menos a una fase vital",
        };
        f.write_str(mensaje)
    }
}

impl Error for ErrorDominio {}
