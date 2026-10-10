//! Especies (#10).

use crate::dominio::error::ErrorDominio;

/// Identificador interno y estable de una especie, independiente de su nombre.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EspecieId(u64);

impl EspecieId {
    pub fn new(valor: u64) -> Self {
        Self(valor)
    }

    pub fn valor(self) -> u64 {
        self.0
    }
}

/// Nombre científico de una especie en nomenclatura binomial (`Neogale vison`),
/// con una tercera palabra opcional para la subespecie.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NombreCientifico(String);

impl NombreCientifico {
    /// El género empieza por mayúscula y el resto de palabras van en minúscula.
    pub fn new(nombre: &str) -> Result<Self, ErrorDominio> {
        // `split` siempre devuelve al menos un elemento (vacío si el nombre lo
        // está), así que `palabras[0]` existe siempre.
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
        if genero_valido && resto_valido {
            Ok(Self(nombre.to_owned()))
        } else {
            Err(ErrorDominio::NombreCientificoInvalido)
        }
    }

    pub fn como_str(&self) -> &str {
        &self.0
    }
}

/// Especie, con identidad propia que se conserva aunque la taxonomía cambie su
/// nombre científico. No guarda si es "objetivo" o "depredadora": ese papel lo
/// da la depredación.
#[derive(Debug, Clone)]
pub struct Especie {
    id: EspecieId,
    nombre_vigente: NombreCientifico,
    nombres_anteriores: Vec<NombreCientifico>,
    nombre_comun: String,
    invasora: bool,
}

impl Especie {
    pub fn new(
        id: EspecieId,
        nombre_vigente: NombreCientifico,
        nombres_anteriores: Vec<NombreCientifico>,
        nombre_comun: String,
        invasora: bool,
    ) -> Result<Self, ErrorDominio> {
        if nombre_comun.trim().is_empty() {
            return Err(ErrorDominio::NombreComunRequerido);
        }
        if nombres_anteriores.contains(&nombre_vigente) {
            return Err(ErrorDominio::NombreCientificoDuplicado);
        }
        Ok(Self {
            id,
            nombre_vigente,
            nombres_anteriores,
            nombre_comun,
            invasora,
        })
    }

    pub fn id(&self) -> EspecieId {
        self.id
    }

    pub fn nombre_vigente(&self) -> &NombreCientifico {
        &self.nombre_vigente
    }

    pub fn nombres_anteriores(&self) -> &[NombreCientifico] {
        &self.nombres_anteriores
    }

    pub fn nombre_comun(&self) -> &str {
        &self.nombre_comun
    }

    pub fn es_invasora(&self) -> bool {
        self.invasora
    }
}

/// Dos especies son la misma si tienen el mismo identificador, aunque cambie su nombre.
impl PartialEq for Especie {
    fn eq(&self, otra: &Self) -> bool {
        self.id == otra.id
    }
}

impl Eq for Especie {}
