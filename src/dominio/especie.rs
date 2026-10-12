use crate::dominio::depredacion::Depredador;
use crate::dominio::error::ErrorDominio;

#[derive(Debug, Clone)]
pub struct Especie {
    nombre_vigente: String,
    nombres_anteriores: Vec<String>,
    nombre_comun: String,
    invasora: bool,
    depredadores: Vec<Depredador>,
}

impl Especie {
    pub fn new(
        nombre_vigente: String,
        nombres_anteriores: Vec<String>,
        nombre_comun: String,
        invasora: bool,
        depredadores: Vec<Depredador>,
    ) -> Result<Self, ErrorDominio> {
        if !es_nombre_cientifico(&nombre_vigente)
            || !nombres_anteriores.iter().all(|n| es_nombre_cientifico(n))
        {
            return Err(ErrorDominio::NombreCientificoInvalido);
        }
        if nombre_comun.trim().is_empty() {
            return Err(ErrorDominio::NombreComunRequerido);
        }
        if nombres_anteriores.contains(&nombre_vigente) {
            return Err(ErrorDominio::NombreCientificoDuplicado);
        }
        if depredadores
            .iter()
            .any(|d| d.especie == nombre_vigente || nombres_anteriores.contains(&d.especie))
        {
            return Err(ErrorDominio::DepredacionSobreMismaEspecie);
        }
        Ok(Self {
            nombre_vigente,
            nombres_anteriores,
            nombre_comun,
            invasora,
            depredadores,
        })
    }
}

pub(super) fn es_nombre_cientifico(nombre: &str) -> bool {
    let palabras: Vec<&str> = nombre.split(' ').collect();
    let (genero, resto) = (palabras[0], &palabras[1..]);
    let genero_valido = genero
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_uppercase())
        && genero.chars().skip(1).all(|c| c.is_ascii_lowercase());
    let resto_valido = (1..=2).contains(&resto.len())
        && resto
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_lowercase() || c == '-'));
    genero_valido && resto_valido
}

impl PartialEq for Especie {
    fn eq(&self, otra: &Self) -> bool {
        self.nombre_vigente == otra.nombre_vigente
    }
}

impl Eq for Especie {}
