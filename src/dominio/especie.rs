use crate::dominio::error::ErrorDominio;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NombreCientifico(String);

impl NombreCientifico {
    pub fn new(nombre: &str) -> Result<Self, ErrorDominio> {
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

impl PartialEq for Especie {
    fn eq(&self, otra: &Self) -> bool {
        self.id == otra.id
    }
}

impl Eq for Especie {}
