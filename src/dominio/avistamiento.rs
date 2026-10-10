//! Avistamientos y sus fuentes (#13).

use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use crate::dominio::coordenada::Coordenada;
use crate::dominio::error::ErrorDominio;
use crate::dominio::especie::EspecieId;

/// Plataforma de la que procede un avistamiento.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fuente {
    INaturalist,
    Gbif,
}

/// Fotos de un avistamiento, que permiten comprobar que la especie está bien
/// identificada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenciaFotografica {
    autor: String,
    urls: Vec<String>,
}

impl EvidenciaFotografica {
    const AUTOR_DESCONOCIDO: &'static str = "Anónimo";

    /// Si no se conoce el autor, toma el valor «Anónimo». Un avistamiento sin
    /// fotos tiene la lista de URLs vacía.
    pub fn new(autor: Option<String>, urls: Vec<String>) -> Result<Self, ErrorDominio> {
        if !urls.iter().all(|url| es_url_valida(url)) {
            return Err(ErrorDominio::UrlFotoInvalida);
        }
        let autor = autor
            .filter(|a| !a.trim().is_empty())
            .unwrap_or_else(|| Self::AUTOR_DESCONOCIDO.to_owned());
        Ok(Self { autor, urls })
    }

    pub fn autor(&self) -> &str {
        &self.autor
    }

    pub fn urls(&self) -> &[String] {
        &self.urls
    }
}

/// Validación mínima sin dependencias; la completa queda para el objetivo 3.
fn es_url_valida(url: &str) -> bool {
    let con_esquema = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"));
    con_esquema.is_some_and(|resto| !resto.is_empty() && !url.contains(char::is_whitespace))
}

/// Observación de una especie en un lugar y un momento, identificada por la
/// fuente de la que procede y su identificador original en esa fuente.
#[derive(Debug, Clone)]
pub struct Avistamiento {
    fuente: Fuente,
    id_original: String,
    especie: EspecieId,
    fecha: SystemTime,
    coordenada: Coordenada,
    calidad: Option<String>,
    evidencia: EvidenciaFotografica,
}

impl Avistamiento {
    /// La calidad solo existe si la fuente la proporciona (iNaturalist sí, GBIF no).
    pub fn new(
        fuente: Fuente,
        id_original: String,
        especie: EspecieId,
        fecha: SystemTime,
        coordenada: Coordenada,
        calidad: Option<String>,
        evidencia: EvidenciaFotografica,
    ) -> Result<Self, ErrorDominio> {
        if id_original.trim().is_empty() {
            return Err(ErrorDominio::AvistamientoIncompleto);
        }
        Ok(Self {
            fuente,
            id_original,
            especie,
            fecha,
            coordenada,
            calidad,
            evidencia,
        })
    }

    pub fn fuente(&self) -> Fuente {
        self.fuente
    }

    pub fn id_original(&self) -> &str {
        &self.id_original
    }

    pub fn especie(&self) -> EspecieId {
        self.especie
    }

    pub fn fecha(&self) -> SystemTime {
        self.fecha
    }

    pub fn coordenada(&self) -> Coordenada {
        self.coordenada
    }

    pub fn calidad(&self) -> Option<&str> {
        self.calidad.as_deref()
    }

    pub fn evidencia(&self) -> &EvidenciaFotografica {
        &self.evidencia
    }
}

/// Dos avistamientos son el mismo si proceden de la misma fuente con el mismo
/// identificador original.
impl PartialEq for Avistamiento {
    fn eq(&self, otro: &Self) -> bool {
        self.fuente == otro.fuente && self.id_original == otro.id_original
    }
}

impl Eq for Avistamiento {}

impl Hash for Avistamiento {
    fn hash<H: Hasher>(&self, estado: &mut H) {
        self.fuente.hash(estado);
        self.id_original.hash(estado);
    }
}
