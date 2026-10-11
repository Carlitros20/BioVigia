use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use crate::dominio::coordenada::Coordenada;
use crate::dominio::error::ErrorDominio;
use crate::dominio::especie::EspecieId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fuente {
    INaturalist,
    Gbif,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GradoCalidad {
    Investigacion,
    NecesitaIdentificacion,
    Casual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenciaFotografica {
    autor: String,
    urls: Vec<String>,
}

impl EvidenciaFotografica {
    const AUTOR_DESCONOCIDO: &'static str = "Anónimo";

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

fn es_url_valida(url: &str) -> bool {
    let con_esquema = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"));
    con_esquema.is_some_and(|resto| !resto.is_empty() && !url.contains(char::is_whitespace))
}

#[derive(Debug, Clone)]
pub struct Avistamiento {
    fuente: Fuente,
    id_original: String,
    especie: EspecieId,
    fecha: SystemTime,
    coordenada: Coordenada,
    calidad: Option<GradoCalidad>,
    evidencia: EvidenciaFotografica,
}

impl Avistamiento {
    pub fn new(
        fuente: Fuente,
        id_original: String,
        especie: EspecieId,
        fecha: SystemTime,
        coordenada: Coordenada,
        calidad: Option<GradoCalidad>,
        evidencia: EvidenciaFotografica,
    ) -> Result<Self, ErrorDominio> {
        let id_original = id_original.trim().to_owned();
        if id_original.is_empty() {
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

    pub fn calidad(&self) -> Option<GradoCalidad> {
        self.calidad
    }

    pub fn evidencia(&self) -> &EvidenciaFotografica {
        &self.evidencia
    }
}

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
